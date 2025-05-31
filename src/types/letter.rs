use std::{fmt, str::FromStr};

use crate::error::ParseError;

/// Musical letter names A through G, with numeric backing
/// representing their position in the chromatic scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
#[allow(missing_docs)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::symbols::*;

        match self {
            Letter::C => write!(f, "{}", C),
            Letter::D => write!(f, "{}", D),
            Letter::E => write!(f, "{}", E),
            Letter::F => write!(f, "{}", F),
            Letter::G => write!(f, "{}", G),
            Letter::A => write!(f, "{}", A),
            Letter::B => write!(f, "{}", B),
        }
    }
}

impl Letter {
    /// Returns the base MIDI note number for this letter in octave 0
    pub fn base_midi_number(&self) -> i8 {
        match self {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 7,
            Letter::A => 9,
            Letter::B => 11,
        }
    }

    /// Gets the next letter in the sequence (wrapping from G to A)
    pub fn next(self) -> Letter {
        use Letter::*;
        match self {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C,
        }
    }

    /// Gets the previous letter in the sequence (wrapping from A to G)
    pub fn prev(self) -> Letter {
        use Letter::*;
        match self {
            C => B,
            D => C,
            E => D,
            F => E,
            G => F,
            A => G,
            B => A,
        }
    }

    /// Creates a `Letter` from a character, returning an error if the character is invalid
    pub const fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'C' | 'c' => Ok(Letter::C),
            'D' | 'd' => Ok(Letter::D),
            'E' | 'e' => Ok(Letter::E),
            'F' | 'f' => Ok(Letter::F),
            'G' | 'g' => Ok(Letter::G),
            'A' | 'a' => Ok(Letter::A),
            'B' | 'b' => Ok(Letter::B),
            _ => Err("Invalid letter"),
        }
    }

    /// Returns an array of all musical letters in order
    pub fn all() -> [Letter; 7] {
        [
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
            Letter::A,
            Letter::B,
        ]
    }
}

impl FromStr for Letter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError::InvalidNoteName(s.to_string()));
        }

        Letter::from_char(s.chars().next().unwrap())
            .map_err(|_| ParseError::InvalidNoteName(s.to_string()))
    }
}
