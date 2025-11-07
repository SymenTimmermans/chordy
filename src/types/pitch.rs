use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::error::ParseError;
use crate::transposition::{ChromaticTransposer, Transposer};

use super::{Accidental, Key, Letter, NoteName};

/// Strategies for choosing enharmonic spellings when converting MIDI to pitches
///
/// These strategies help determine which enharmonic spelling to use when multiple
/// options exist for the same pitch (e.g., C♯ vs D♭).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellingStrategy {
    /// Prefer sharp spellings for black keys (C♯, D♯, F♯, G♯, A♯)
    PreferSharps,
    /// Prefer flat spellings for black keys (D♭, E♭, G♭, A♭, B♭)
    PreferFlats,
    /// Prefer natural spellings where possible, avoiding accidentals
    PreferNaturals,
    /// Use key context to determine appropriate spelling
    KeyContext(Key),
    /// Use directional chromatic spelling (ascending: sharps, descending: flats)
    DirectionalChromatic { ascending: bool },
    /// Minimize the total number of accidentals in a sequence
    MinimizeAccidentals,
}

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
        // Calculate the number of semitones from A4 using standard MIDI
        let semitones_from_a4 = 12.0 * (hz / Self::A440).log2();
        let midi_number = (semitones_from_a4 + 69.0).round() as i8;

        // Clamp MIDI number to valid range [0, 127]
        let clamped_midi = midi_number.clamp(0, 127) as u8;

        // Convert MIDI number to pitch
        Self::from_midi_number(clamped_midi)
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
        // Calculate the number of semitones from A4 (standard MIDI note 69)
        let semitones_from_a4 = self.midi_number() as f32 - 69.0;
        Self::A440 * 2.0f32.powf(semitones_from_a4 / 12.0)
    }

    /// Creates a `Pitch` from a MIDI note number.
    ///
    /// MIDI note numbers start at 0 for C-1 and go up to 127 for G9.
    /// This method uses sharps for black keys by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::from_midi_number(60);
    /// assert_eq!(pitch.to_string(), "C4");
    ///
    /// let pitch = Pitch::from_midi_number(69);
    /// assert_eq!(pitch.to_string(), "A4");
    /// ```
    pub fn from_midi_number(midi_number: u8) -> Self {
        // MIDI note 0 is C-1
        let octave = (midi_number as i8 / 12) - 1;
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
    /// MIDI note numbers start at 0 for C-1 and go up to 127 for G9.
    /// This method uses flats instead of sharps for black keys where possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let pitch = Pitch::from_midi_number_prefer_flats(61);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Db4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "D♭4");
    ///
    /// let pitch = Pitch::from_midi_number_prefer_flats(66);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Gb4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "G♭4");
    /// ```
    pub fn from_midi_number_prefer_flats(midi_number: u8) -> Self {
        // MIDI note 0 is C-1
        let octave = (midi_number as i8 / 12) - 1;
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
    /// MIDI note numbers start at 0 for C-1 and go up to 127 for G9.
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
    /// assert_eq!(pitch.to_string(), "F#4"); // F# fits better in G major than Gb
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "F♯4"); // F♯ fits better in G major than G♭
    ///
    /// let key = Key::Major(chordy::note!("F"));
    /// let pitch = Pitch::from_midi_number_in_key(66, &key);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Gb4"); // Gb fits better in F major than F#
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "G♭4"); // G♭ fits better in F major than F♯
    /// ```
    pub fn from_midi_number_in_key(midi_number: u8, key: &Key) -> Self {
        // MIDI note 0 is C-1
        let octave = (midi_number as i8 / 12) - 1;
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

    /// Creates a `Pitch` from a MIDI note number using a specific spelling strategy.
    ///
    /// MIDI note numbers start at 0 for C-1 and go up to 127 for G9.
    /// This method uses the specified strategy to choose enharmonic spellings.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, SpellingStrategy};
    ///
    /// // Prefer sharps
    /// let pitch = Pitch::from_midi_with_strategy(61, SpellingStrategy::PreferSharps);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "C#4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "C♯4");
    ///
    /// // Prefer flats
    /// let pitch = Pitch::from_midi_with_strategy(61, SpellingStrategy::PreferFlats);
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Db4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "D♭4");
    ///
    /// // Directional chromatic (ascending)
    /// let pitch = Pitch::from_midi_with_strategy(61, SpellingStrategy::DirectionalChromatic { ascending: true });
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "C#4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "C♯4");
    ///
    /// // Directional chromatic (descending)
    /// let pitch = Pitch::from_midi_with_strategy(61, SpellingStrategy::DirectionalChromatic { ascending: false });
    /// #[cfg(not(feature = "utf8_symbols"))]
    /// assert_eq!(pitch.to_string(), "Db4");
    /// #[cfg(feature = "utf8_symbols")]
    /// assert_eq!(pitch.to_string(), "D♭4");
    /// ```
    pub fn from_midi_with_strategy(midi_number: u8, strategy: SpellingStrategy) -> Self {
        // MIDI note 0 is C-1
        let octave = (midi_number as i8 / 12) - 1;
        let note_index = midi_number % 12;

        // Apply the specified spelling strategy
        let (letter, accidental) = match strategy {
            SpellingStrategy::PreferSharps => Self::spell_with_sharps(note_index),
            SpellingStrategy::PreferFlats => Self::spell_with_flats(note_index),
            SpellingStrategy::PreferNaturals => Self::spell_with_naturals(note_index),
            SpellingStrategy::KeyContext(key) => Self::spell_with_key_context(note_index, &key),
            SpellingStrategy::DirectionalChromatic { ascending } => {
                if ascending {
                    Self::spell_with_sharps(note_index)
                } else {
                    Self::spell_with_flats(note_index)
                }
            }
            SpellingStrategy::MinimizeAccidentals => {
                // Default to sharps for MinimizeAccidentals (simplest implementation)
                // In a real implementation, this would analyze surrounding context
                Self::spell_with_sharps(note_index)
            }
        };

        Pitch::new(letter, accidental, octave)
    }

    /// Helper method: spell with sharps (default strategy)
    fn spell_with_sharps(note_index: u8) -> (Letter, Accidental) {
        match note_index {
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
        }
    }

    /// Helper method: spell with flats
    fn spell_with_flats(note_index: u8) -> (Letter, Accidental) {
        match note_index {
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
        }
    }

    /// Helper method: spell with naturals where possible
    /// This strategy avoids accidentals by using enharmonic equivalents
    fn spell_with_naturals(note_index: u8) -> (Letter, Accidental) {
        match note_index {
            0 => (Letter::C, Accidental::Natural),
            1 => (Letter::C, Accidental::Sharp), // No natural equivalent
            2 => (Letter::D, Accidental::Natural),
            3 => (Letter::D, Accidental::Sharp), // No natural equivalent
            4 => (Letter::E, Accidental::Natural),
            5 => (Letter::F, Accidental::Natural),
            6 => (Letter::F, Accidental::Sharp), // No natural equivalent
            7 => (Letter::G, Accidental::Natural),
            8 => (Letter::G, Accidental::Sharp), // No natural equivalent
            9 => (Letter::A, Accidental::Natural),
            10 => (Letter::A, Accidental::Sharp), // No natural equivalent
            11 => (Letter::B, Accidental::Natural),
            _ => unreachable!(),
        }
    }

    /// Helper method: spell with key context
    fn spell_with_key_context(note_index: u8, key: &Key) -> (Letter, Accidental) {
        // Get the key's accidental preference (positive for sharps, negative for flats)
        let key_accidentals = key.accidentals();

        // Map note index to letter and accidental based on key preference
        // For natural keys (0 accidentals), default to sharps to be consistent with from_midi_number
        match note_index {
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
        }
    }

    /// Returns the full MIDI note number for this pitch.
    /// Starting from C-1 (MIDI note 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{Pitch, Letter, Accidental};
    ///
    /// let pitch = Pitch::new(Letter::C, Accidental::Natural, 4);
    /// assert_eq!(pitch.midi_number(), 60);
    ///
    /// let pitch = Pitch::new(Letter::G, Accidental::Sharp, 5);
    /// assert_eq!(pitch.midi_number(), 80);
    ///
    /// ```
    pub fn midi_number(&self) -> i8 {
        // MIDI octaves start at -1, where C-1 is note 0
        self.name.base_midi_number() + ((self.octave + 1) * 12)
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
    /// let a4 = Pitch::new(chordy::Letter::A, chordy::Accidental::Natural, 4);
    /// let a_sharp4 = Pitch::new(chordy::Letter::A, chordy::Accidental::Sharp, 4);
    /// let cents_diff = a_sharp4.cents_from(&a4);
    /// assert!((cents_diff - 100.0).abs() < 0.01); // A#4 is 100 cents above A4
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

    /// Returns the nth harmonic of this pitch.
    ///
    /// The harmonic series is a fundamental concept in acoustics where each harmonic
    /// has a frequency that is an integer multiple of the fundamental frequency.
    /// This method returns the closest equal-tempered pitch to the exact harmonic frequency.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let c2 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 2);
    /// let second_harmonic = c2.harmonic(2);
    /// assert_eq!(second_harmonic.to_string(), "C3"); // Octave above
    ///
    /// let third_harmonic = c2.harmonic(3);
    /// assert_eq!(third_harmonic.to_string(), "G3"); // Perfect fifth above
    /// ```
    pub fn harmonic(&self, n: usize) -> Pitch {
        if n == 0 {
            panic!("Harmonic number must be positive (n >= 1)");
        }
        let fundamental_freq = self.to_frequency();
        let harmonic_freq = fundamental_freq * n as f32;

        // Ensure frequency is not too high for MIDI conversion
        // MIDI note 127 (G8) has frequency ~12543.85 Hz
        if harmonic_freq > 12500.0 {
            panic!("Harmonic frequency {} Hz is above maximum supported frequency (12500 Hz)", harmonic_freq);
        }

        Pitch::from_frequency(harmonic_freq)
    }

    /// Returns a vector of harmonics up to the nth harmonic.
    ///
    /// This includes the fundamental (harmonic 1) through the specified harmonic.
    /// The harmonics are returned in order from lowest to highest frequency.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let c2 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 2);
    /// let harmonics = c2.harmonics(8);
    /// // Returns: [C2, C3, G3, C4, E4, G4, Bb4, C5]
    /// assert_eq!(harmonics.len(), 8);
    /// assert_eq!(harmonics[0].to_string(), "C2"); // Fundamental
    /// assert_eq!(harmonics[1].to_string(), "C3"); // 2nd harmonic (octave)
    /// assert_eq!(harmonics[2].to_string(), "G3"); // 3rd harmonic (perfect fifth)
    /// ```
    pub fn harmonics(&self, up_to: usize) -> Vec<Pitch> {
        if up_to == 0 {
            return Vec::new();
        }
        (1..=up_to).map(|n| self.harmonic(n)).collect()
    }

    /// Returns the nth subharmonic of this pitch.
    ///
    /// Subharmonics are the inverse of harmonics - they have frequencies that are
    /// integer divisions of the fundamental frequency. The nth subharmonic has
    /// frequency = fundamental_frequency / n.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let c4 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 4);
    /// let first_subharmonic = c4.subharmonic(2);
    /// assert_eq!(first_subharmonic.to_string(), "C3"); // Octave below
    ///
    /// let second_subharmonic = c4.subharmonic(3);
    /// assert_eq!(second_subharmonic.to_string(), "F2"); // Major third below
    /// ```
    pub fn subharmonic(&self, n: usize) -> Pitch {
        if n == 0 {
            panic!("Subharmonic number must be positive (n >= 1)");
        }
        let fundamental_freq = self.to_frequency();
        let subharmonic_freq = fundamental_freq / n as f32;

        // Ensure frequency is not too low for MIDI conversion
        // MIDI note 0 (C-2) has frequency ~8.18 Hz
        if subharmonic_freq < 8.0 {
            panic!("Subharmonic frequency {} Hz is below minimum supported frequency (8 Hz)", subharmonic_freq);
        }

        Pitch::from_frequency(subharmonic_freq)
    }

    /// Given a pitch that is the nth harmonic of some fundamental,
    /// returns the fundamental pitch.
    ///
    /// This is useful for fundamental frequency detection from overtones.
    /// The method works by dividing the frequency by n and finding the closest
    /// equal-tempered pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let g3 = Pitch::new(chordy::Letter::G, chordy::Accidental::Natural, 3);
    /// let fundamental = g3.fundamental_of_harmonic(3);
    /// assert_eq!(fundamental.to_string(), "C2"); // G3 is the 3rd harmonic of C2
    ///
    /// let c5 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 5);
    /// let fundamental = c5.fundamental_of_harmonic(8);
    /// assert_eq!(fundamental.to_string(), "C2"); // C5 is the 8th harmonic of C2
    /// ```
    pub fn fundamental_of_harmonic(&self, n: usize) -> Pitch {
        if n == 0 {
            panic!("Harmonic number must be positive (n >= 1)");
        }
        let harmonic_freq = self.to_frequency();
        let fundamental_freq = harmonic_freq / n as f32;
        Pitch::from_frequency(fundamental_freq)
    }

    /// Determines if this pitch is exactly the nth harmonic of the given fundamental.
    ///
    /// Returns `Some(n)` if this pitch matches the nth harmonic of the fundamental
    /// within a small tolerance (accounting for equal temperament), or `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let c2 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 2);
    /// let c3 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 3);
    /// let g3 = Pitch::new(chordy::Letter::G, chordy::Accidental::Natural, 3);
    ///
    /// // C3 is the 2nd harmonic of C2
    /// assert_eq!(c3.is_harmonic_of(&c2), Some(2));
    ///
    /// // G3 is the 3rd harmonic of C2
    /// assert_eq!(g3.is_harmonic_of(&c2), Some(3));
    ///
    /// // A4 is not a harmonic of C2
    /// let a4 = Pitch::new(chordy::Letter::A, chordy::Accidental::Natural, 4);
    /// assert_eq!(a4.is_harmonic_of(&c2), None);
    /// ```
    pub fn is_harmonic_of(&self, fundamental: &Pitch) -> Option<usize> {
        let fundamental_freq = fundamental.to_frequency();
        let this_freq = self.to_frequency();

        // Calculate the harmonic ratio
        let ratio = this_freq / fundamental_freq;

        // Check if ratio is close to an integer within tolerance
        // We use a tolerance of 13% to account for equal temperament deviations
        // Equal temperament doesn't perfectly match the harmonic series, especially
        // for higher harmonics like the 7th harmonic which has ~12.7% deviation
        let tolerance = 0.13;
        let n_float = ratio;
        let n_rounded = n_float.round();

        if (n_float - n_rounded).abs() < tolerance && n_rounded >= 1.0 {
            let n = n_rounded as usize;
            // Verify that the harmonic actually matches the expected pitch
            let expected_harmonic = fundamental.harmonic(n);
            if self.is_enharmonic_with(&expected_harmonic) {
                return Some(n);
            }
        }

        None
    }

    /// Finds the nearest harmonic of the given fundamental to this pitch.
    ///
    /// Returns a tuple `(harmonic_number, cents_deviation)` where:
    /// - `harmonic_number` is the harmonic number (1 = fundamental, 2 = octave, etc.)
    /// - `cents_deviation` is the deviation in cents from the exact harmonic frequency
    ///
    /// A positive cents deviation means this pitch is higher than the exact harmonic,
    /// while negative means it's lower.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::Pitch;
    ///
    /// let c2 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 2);
    /// let c3 = Pitch::new(chordy::Letter::C, chordy::Accidental::Natural, 3);
    /// let (harmonic, cents) = c3.nearest_harmonic(&c2);
    /// assert_eq!(harmonic, 2); // C3 is the 2nd harmonic of C2
    /// assert!((cents - 0.0).abs() < 0.1); // Should be very close to 0 cents
    ///
    /// // Test with a detuned pitch that snaps to a different equal-tempered pitch
    /// let sharp_c3 = c3.transpose_cents(100.0);
    /// let (harmonic, cents) = sharp_c3.nearest_harmonic(&c2);
    /// assert_eq!(harmonic, 2);
    /// assert!((cents - 100.0).abs() < 2.0); // Should be about 100 cents sharp
    /// ```
    pub fn nearest_harmonic(&self, fundamental: &Pitch) -> (usize, f32) {
        let fundamental_freq = fundamental.to_frequency();
        let this_freq = self.to_frequency();

        // Calculate the harmonic ratio
        let ratio = this_freq / fundamental_freq;

        // Find the nearest integer harmonic number
        let n_float = ratio;
        let n_rounded = n_float.round();
        let n = n_rounded.max(1.0) as usize;

        // Calculate the exact frequency of the nth harmonic
        let exact_harmonic_freq = fundamental_freq * n as f32;

        // Calculate cents deviation from the exact harmonic
        let cents_deviation = 1200.0 * (this_freq / exact_harmonic_freq).log2();

        (n, cents_deviation)
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
