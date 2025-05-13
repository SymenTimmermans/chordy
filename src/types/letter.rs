use std::{fmt, str::FromStr};

use crate::error::ParseError;

/// Musical letter names A through G, with numeric backing
/// representing their position in the chromatic scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Letter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
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
        *self as i8
    }
    
    /// Gets the next letter in the sequence (wrapping from G to A)
    pub fn next(&self) -> Self {
        match self {
            Letter::A => Letter::B,
            Letter::B => Letter::C,
            Letter::C => Letter::D,
            Letter::D => Letter::E,
            Letter::E => Letter::F,
            Letter::F => Letter::G,
            Letter::G => Letter::A,
        }
    }
    
    /// Gets the previous letter in the sequence (wrapping from A to G)
    pub fn _prev(&self) -> Self {
        match self {
            Letter::A => Letter::G,
            Letter::B => Letter::A,
            Letter::C => Letter::B,
            Letter::D => Letter::C,
            Letter::E => Letter::D,
            Letter::F => Letter::E,
            Letter::G => Letter::F,
        }
    }

    pub fn from_char(c: char) -> Result<Self, ParseError> {
        match c {
            'C' | 'c' => Ok(Letter::C),
            'D' | 'd' => Ok(Letter::D),
            'E' | 'e' => Ok(Letter::E),
            'F' | 'f' => Ok(Letter::F),
            'G' | 'g' => Ok(Letter::G),
            'A' | 'a' => Ok(Letter::A),
            'B' | 'b' => Ok(Letter::B),
            _ => Err(ParseError::InvalidNoteName("Invalid letter".to_string())),
        }
    }
}

impl FromStr for Letter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError::InvalidNoteName(s.to_string()));
        }
        
        Letter::from_char(s.chars().next().unwrap())
    }
}
