use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;
use super::{Letter, Accidental};

/// Represents a musical note name with a letter and accidental
/// 
/// # Examples
/// 
/// ```
/// use chordy::NoteName;
/// use chordy::{Letter, Accidental};
/// 
/// let c_sharp = NoteName::new(Letter::C, Accidental::Sharp);
/// assert_eq!(c_sharp.to_string(), "C♯");
/// 
/// // Enharmonic equivalence check
/// let d_flat = NoteName::new(Letter::D, Accidental::Flat);
/// assert!(c_sharp.is_enharmonic_with(&d_flat));
/// assert_ne!(c_sharp, d_flat); // Different note names
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteName {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl NoteName {
    pub fn new(letter: Letter, accidental: Accidental) -> Self {
        NoteName { letter, accidental }
    }

    /// Returns the MIDI note number for this note name in octave 0
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{NoteName, Letter, Accidental};
    /// let c_natural = NoteName::new(Letter::C, Accidental::Natural);
    /// assert_eq!(c_natural.base_midi_number(), 0);
    ///
    /// let f_sharp = NoteName::new(Letter::F, Accidental::Sharp);
    /// assert_eq!(f_sharp.base_midi_number(), 6);
    ///
    /// let b_flat = NoteName::new(Letter::B, Accidental::Flat);
    /// assert_eq!(b_flat.base_midi_number(), 10);
    /// ```
    pub fn base_midi_number(&self) -> i8 {
        self.letter.base_midi_number() + self.accidental.semitone_offset()
    }

    /// Checks if two note names are enharmonically equivalent
    pub fn is_enharmonic_with(&self, other: &Self) -> bool {
        // Notes are enharmonically equivalent if they represent the same pitch
        self.base_midi_number() % 12 == other.base_midi_number() % 12
    }
}

impl fmt::Display for NoteName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.accidental {
            Accidental::Natural => write!(f, "{}", self.letter),
            _ => write!(f, "{}{}", self.letter, self.accidental),
        }
    }
}

impl FromStr for NoteName {
    type Err = ParseError;

    /// Parses a string into a NoteName, taking the first character to be a Letter, and the rest of
    /// the string to be an accidental. 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The string should be at least 1 character long
        if s.len() < 1 {
            return Err(ParseError::InvalidNoteName(s.to_string()));
        }

        let letter = Letter::from_str(&s[0..1])?;

        let accidental = if s.len() > 1 {
            Accidental::from_str(&s[1..])?
        } else {
            Accidental::Natural
        };
        
        Ok(NoteName::new(letter, accidental))
    }
}
