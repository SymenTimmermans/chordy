use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::error::ParseError;
use crate::transposition::{ChromaticTransposer, Transposer};

use super::{Accidental, Key, Letter, NoteName};

/// A specific pitch with both note name and octave
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    /// The note name (letter + accidental)
    pub name: NoteName,
    /// The octave number, where C-2 is octave -2
    pub octave: i8,
}

impl Pitch {
    /// Creates a new `Pitch` from a letter, accidental, and octave.
    pub fn new(letter: Letter, accidental: Accidental, octave: i8) -> Self {
        Pitch {
            name: NoteName::new(letter, accidental),
            octave,
        }
    }

    /// The standard concert pitch frequency for A4 in Hz.
    pub const A440: f32 = 440.0;

    /// The frequency of C0 (MIDI note 0) in scientific pitch notation.
    pub const C0_FREQUENCY: f32 = 16.3516;

    /// Converts a frequency in Hz to the closest `Pitch` using equal temperament tuning.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::from_frequency(440.0);
    /// assert_eq!(pitch.to_string(), "A4");
    ///
    /// let pitch = Pitch::from_frequency(261.63);
    /// assert_eq!(pitch.to_string(), "C4");
    /// ```
    pub fn from_frequency(hz: f32) -> Self {
        // Calculate the number of semitones from A4
        // In this system, A4 has MIDI number 69 (base_midi_number=9 + (4+2)*12 = 9+72=81)
        // Wait, let me recalculate: A4 = base_midi_number=9 + ((4+2)*12) = 9 + 72 = 81
        // But standard MIDI A4 is 69, so there's a 12 semitone difference

        // Calculate the number of semitones from A4 using standard MIDI
        let semitones_from_a4 = 12.0 * (hz / Self::A440).log2();
        let standard_midi_number = (semitones_from_a4 + 69.0).round() as i8;

        // Convert from standard MIDI to this system's MIDI (add 12 semitones)
        let midi_number = standard_midi_number + 12;

        // Convert MIDI number to pitch
        Self::from_midi_number(midi_number as u8)
    }

    /// Converts this pitch to its frequency in Hz using equal temperament tuning.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::new(chordy::Letter::A, chordy::Accidental::Natural, 4);
    /// assert!((pitch.to_frequency() - 440.0).abs() < 0.01);
    ///
    /// let pitch = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 4);
    /// assert!((pitch.to_frequency() - 261.63).abs() < 0.01);
    /// ```
    pub fn to_frequency(&self) -> f32 {
        // Convert from this system's MIDI to standard MIDI (subtract 12 semitones)
        let standard_midi_number = self.midi_number() as f32 - 12.0;

        // Calculate the number of semitones from A4 (standard MIDI note 69)
        let semitones_from_a4 = standard_midi_number - 69.0;
        Self::A440 * 2.0f32.powf(semitones_from_a4 / 12.0)
    }

    /// Creates a `Pitch` from a MIDI note number.
    ///
    /// MIDI note numbers start at 0 for C-2 and go up to 127 for G8.
    /// This method uses sharps for black keys by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::from_midi_number(60);
    /// assert_eq!(pitch.to_string(), "C3");
    ///
    /// let pitch = Pitch::from_midi_number(69);
    /// assert_eq!(pitch.to_string(), "A3");
    /// ```
    pub fn from_midi_number(midi_number: u8) -> Self {
        // MIDI note 0 is C-2
        let octave = (midi_number as i8 / 12) - 2;
        let note_index = midi_number % 12;

        // Map note index to letter and accidental (using sharps by default)
        let (letter, accidental) = match note_index {
            0 => (Letter::C, Accidental::Natural),
            1 => (Letter::C, Accidental::Sharp),
            2 => (Letter::D, Accidental::Natural),
            3 => (Letter::D, Accidental::Sharp),
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => (Letter::F, Accidental::Sharp),
            7 => (Letter::G, Accidental::Natural),
            8 => (Letter::G, Accidental::Sharp),
            9 => (Letter::A, Accidental::Natural),
            10 => (Letter::A, Accidental::Sharp),
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        };

        Pitch::new(letter, accidental, octave)
    }

    /// Creates a `Pitch` from a MIDI note number, preferring flats over sharps for black keys.
    ///
    /// MIDI note numbers start at 0 for C-2 and go up to 127 for G8.
    /// This method uses flats instead of sharps for black keys where possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::from_midi_number_prefer_flats(61);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Db3");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "D♭3");
    ///
    /// let pitch = Pitch::from_midi_number_prefer_flats(66);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Gb3");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "G♭3");
    /// ```
    pub fn from_midi_number_prefer_flats(midi_number: u8) -> Self {
        // MIDI note 0 is C-2
        let octave = (midi_number as i8 / 12) - 2;
        let note_index = midi_number % 12;

        // Map note index to letter and accidental (using flats where possible)
        let (letter, accidental) = match note_index {
            0 => (Letter::C, Accidental::Natural),
            1 => (Letter::D, Accidental::Flat),
            2 => (Letter::D, Accidental::Natural),
            3 => (Letter::E, Accidental::Flat),
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => (Letter::G, Accidental::Flat),
            7 => (Letter::G, Accidental::Natural),
            8 => (Letter::A, Accidental::Flat),
            9 => (Letter::A, Accidental::Natural),
            10 => (Letter::B, Accidental::Flat),
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        };

        Pitch::new(letter, accidental, octave)
    }

    /// Creates a `Pitch` from a MIDI note number with key-aware enharmonic spelling.
    ///
    /// MIDI note numbers start at 0 for C-2 and go up to 127 for G8.
    /// This method chooses the enharmonic spelling that best fits the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Key};
    ///
    /// let key = Key::Major(chordy::note!("G"));
    /// let pitch = Pitch::from_midi_number_in_key(66, &key);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "F#3"); // F# fits better in G major than Gb
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "F♯3"); // F♯ fits better in G major than G♭
    ///
    /// let key = Key::Major(chordy::note!("F"));
    /// let pitch = Pitch::from_midi_number_in_key(66, &key);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Gb3"); // Gb fits better in F major than F#
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "G♭3"); // G♭ fits better in F major than F♯
    /// ```
    pub fn from_midi_number_in_key(midi_number: u8, key: &Key) -> Self {
        // MIDI note 0 is C-2
        let octave = (midi_number as i8 / 12) - 2;
        let note_index = midi_number % 12;

        // Get the key's accidental preference (positive for sharps, negative for flats)
        let key_accidentals = key.accidentals();

        // Map note index to letter and accidental based on key preference
        // For natural keys (0 accidentals), default to sharps to be consistent with from_midi_number
        let (letter, accidental) = match note_index {
            0 => (Letter::C, Accidental::Natural),
            1 => {
                // C#/Db
                if key_accidentals >= 0 {
                    (Letter::C, Accidental::Sharp)
                } else {
                    (Letter::D, Accidental::Flat)
                }
            }
            2 => (Letter::D, Accidental::Natural),
            3 => {
                // D#/Eb
                if key_accidentals >= 0 {
                    (Letter::D, Accidental::Sharp)
                } else {
                    (Letter::E, Accidental::Flat)
                }
            }
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => {
                // F#/Gb
                if key_accidentals >= 0 {
                    (Letter::F, Accidental::Sharp)
                } else {
                    (Letter::G, Accidental::Flat)
                }
            }
            7 => (Letter::G, Accidental::Natural),
            8 => {
                // G#/Ab
                if key_accidentals >= 0 {
                    (Letter::G, Accidental::Sharp)
                } else {
                    (Letter::A, Accidental::Flat)
                }
            }
            9 => (Letter::A, Accidental::Natural),
            10 => {
                // A#/Bb
                if key_accidentals >= 0 {
                    (Letter::A, Accidental::Sharp)
                } else {
                    (Letter::B, Accidental::Flat)
                }
            }
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        };

        Pitch::new(letter, accidental, octave)
    }

    /// Returns the full MIDI note number for this pitch.
    /// Starting from C-2 (MIDI note 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Letter, Accidental};
    ///
    /// let pitch = Pitch::new(Letter::C, Accidental::Natural, 3);
    /// assert_eq!(pitch.midi_number(), 60);
    ///
    /// let pitch = Pitch::new(Letter::G, Accidental::Sharp, 5);
    /// assert_eq!(pitch.midi_number(), 92);
    ///
    /// ```
    pub fn midi_number(&self) -> i8 {
        // MIDI octaves start at -2, where C-2 is note 0
        self.name.base_midi_number() + ((self.octave + 2) * 12)
    }

    /// Checks if two pitches represent the same frequency.
    /// Often used for enharmonic equivalence. This assumes equal temperament tuning.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Letter, Accidental};
    ///
    /// let p1 = Pitch::new(Letter::C, Accidental::Natural, 4);
    /// let p2 = Pitch::new(Letter::B, Accidental::Sharp, 3);
    ///
    /// assert!(p1.is_enharmonic_with(&p2));
    ///
    /// let p1 = Pitch::new(Letter::A, Accidental::Flat, 4);
    /// let p2 = Pitch::new(Letter::G, Accidental::Sharp, 4);
    ///
    /// assert!(p1.is_enharmonic_with(&p2));
    /// ```
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        self.midi_number() == other.midi_number()
    }

    /// Transpose this pitch by a number of semitones
    ///
    /// Uses the `ChromaticTransposer` algorithm, which handles enharmonic spelling.
    pub fn transpose(&self, semitone_offset: i8) -> Pitch {
        ChromaticTransposer::transpose(*self, semitone_offset)
    }

    /// Transpose using a specific transposition algorithm
    pub fn transpose_with<T: Transposer>(&self, interval: i8) -> Pitch {
        T::transpose(*self, interval)
    }

    /// Returns true if the note spelling is suspicious.
    pub fn is_suspicious_spelling(&self) -> bool {
        matches!(
            (self.name.letter(), self.name.accidental()),
            (Letter::B, Accidental::Sharp)
                | (Letter::E, Accidental::Sharp)
                | (Letter::C, Accidental::Flat)
                | (Letter::F, Accidental::Flat)
        )
    }

    /// Calculates the cents difference between this pitch and another pitch.
    ///
    /// Cents are a logarithmic measure of pitch difference where 100 cents = 1 semitone.
    /// Positive values indicate this pitch is higher than the other pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let a440 = Pitch::new(chordy::Letter::A, chordy::Accidental::Natural, 4);
    /// let a442 = Pitch::from_frequency(442.0);
    /// let cents_diff = a442.cents_from(&a440);
    /// assert!((cents_diff - 7.85).abs() < 0.1); // A442 is about 7.85 cents sharp of A440
    /// ```
    pub fn cents_from(&self, other: &Pitch) -> f32 {
        let f1 = self.to_frequency();
        let f2 = other.to_frequency();

        // Calculate cents using the formula: cents = 1200 * log2(f1 / f2)
        1200.0 * (f1 / f2).log2()
    }

    /// Transposes this pitch by a specified number of cents.
    ///
    /// Returns the closest equal-tempered pitch to the microtonal target.
    /// Note that the result will be snapped to the nearest equal-tempered pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let a440 = Pitch::new(chordy::Letter::A, chordy::Accidental::Natural, 4);
    /// let a442 = a440.transpose_cents(7.85);
    /// // A442 will be very close to A4 (might be A4 or A#4 depending on rounding)
    /// ```
    pub fn transpose_cents(&self, cents: f32) -> Pitch {
        let current_freq = self.to_frequency();
        let new_freq = current_freq * 2.0f32.powf(cents / 1200.0);
        Pitch::from_frequency(new_freq)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.octave)
    }
}

/// impl debug that just returns the display format
impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Add<i8> for Pitch {
    type Output = Pitch;

    fn add(self, semitones: i8) -> Self::Output {
        self.transpose(semitones)
    }
}

impl AddAssign<i8> for Pitch {
    fn add_assign(&mut self, semitones: i8) {
        *self = *self + semitones;
    }
}

impl FromStr for Pitch {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the index where the octave part begins
        let octave_start_index = s
            .char_indices()
            .find(|(_, c)| c.is_ascii_digit() || *c == '-')
            .map(|(i, _)| i)
            .ok_or_else(|| ParseError::InvalidPitch(s.to_string()))?;

        // If the octave starts with '-', verify the next character is a digit
        if s[octave_start_index..].starts_with('-')
            && (s.len() <= octave_start_index + 1
                || !s[octave_start_index + 1..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_digit()))
        {
            return Err(ParseError::InvalidPitch(s.to_string()));
        }

        // Split the string into note and octave parts using slices (no allocation)
        let note_part = &s[..octave_start_index];
        let octave_part = &s[octave_start_index..];

        // Verify parts are non-empty
        if note_part.is_empty() || octave_part.is_empty() {
            return Err(ParseError::InvalidPitch(s.to_string()));
        }

        // Parse the octave
        let octave = octave_part
            .parse::<i8>()
            .map_err(|_| ParseError::InvalidPitch(s.to_string()))?;

        // Parse the note name
        let name = NoteName::from_str(note_part)?;

        Ok(Pitch { name, octave })
    }
}
