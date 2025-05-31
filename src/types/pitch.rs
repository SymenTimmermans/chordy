use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::error::ParseError;
use crate::transposition::{ChromaticTransposer, Transposer};

use super::{Accidental, Letter, NoteName};

/// A specific pitch with both note name and octave
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name, self.octave)
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
