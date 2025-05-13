use std::{fmt, str::FromStr};

use crate::error::ParseError;

/// Accidentals that modify the pitch of a note,
/// with numeric backing representing semitone shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl Accidental {
    /// Returns the semitone offset for this accidental
    pub fn semitone_offset(&self) -> i8 {
        *self as i8
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::symbols::*;
        
        match self {
            Accidental::Flat => write!(f, "{}", FLAT),
            Accidental::Sharp => write!(f, "{}", SHARP),
            Accidental::Natural => write!(f, "{}", NATURAL),
            Accidental::DoubleFlat => write!(f, "{}", DOUBLE_FLAT),
            Accidental::DoubleSharp => write!(f, "{}", DOUBLE_SHARP),
        }
    }
}

impl FromStr for Accidental {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "♭" => Ok(Accidental::Flat),
            "#" | "♯" => Ok(Accidental::Sharp),
            "n" | "♮" => Ok(Accidental::Natural),
            "bb" | "𝄫" => Ok(Accidental::DoubleFlat),
            "##" | "𝄪" => Ok(Accidental::DoubleSharp),
            _ => Err(ParseError::InvalidAccidental(s.to_string())),
        }
    }
}
