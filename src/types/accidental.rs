use std::{fmt, str::FromStr};

use crate::error::ParseError;

/// Accidentals that modify the pitch of a note,
/// with numeric backing representing semitone shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i8)]
#[allow(missing_docs)]
pub enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl Accidental {
    /// Returns an array with all possible accidentals
    pub fn all() -> [Accidental; 5] {
        [
            Accidental::DoubleFlat,
            Accidental::Flat,
            Accidental::Natural,
            Accidental::Sharp,
            Accidental::DoubleSharp,
        ]
    }

    /// Returns the semitone offset for this accidental
    pub fn semitone_offset(&self) -> i8 {
        *self as i8
    }

    /// Pitch naming penalty for this accidental.
    pub fn penalty(self) -> i32 {
        match self {
            Accidental::Natural => 0,
            Accidental::Sharp | Accidental::Flat => 1,
            Accidental::DoubleSharp | Accidental::DoubleFlat => 3,
        }
    }

    /// Returns true if the accidental is a sharp variant (sharp or double sharp)
    pub fn is_sharp(self) -> bool {
        matches!(self, Accidental::Sharp | Accidental::DoubleSharp)
    }

    /// Returns true if the accidental is a flat variant (flat or double flat)
    pub fn is_flat(self) -> bool {
        matches!(self, Accidental::Flat | Accidental::DoubleFlat)
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
            "♭♭" | "bb" | "𝄫" => Ok(Accidental::DoubleFlat),
            "♯♯" | "##" | "𝄪" => Ok(Accidental::DoubleSharp),
            _ => Err(ParseError::InvalidAccidental(s.to_string())),
        }
    }
}
