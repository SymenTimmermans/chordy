use std::{fmt, str::FromStr};

use crate::error::ParseError;
use crate::types::chord::naming::ChordFormat;

/// Accidentals that modify the pitch of a note,
/// with numeric backing representing semitone shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
    /// Extreme accidentals beyond the standard range (triple sharps, etc.)
    Extreme(i8),
}

impl PartialOrd for Accidental {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Accidental {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.semitone_offset().cmp(&other.semitone_offset())
    }
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
        match self {
            Accidental::DoubleFlat => -2,
            Accidental::Flat => -1,
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
            Accidental::Extreme(level) => *level,
        }
    }

    /// Pitch naming penalty for this accidental.
    pub fn penalty(self) -> i32 {
        match self {
            Accidental::Natural => 0,
            Accidental::Sharp | Accidental::Flat => 1,
            Accidental::DoubleSharp | Accidental::DoubleFlat => 3,
            Accidental::Extreme(level) => 3 + level.abs() as i32,
        }
    }

    /// Returns true if the accidental is a sharp variant (sharp or double sharp)
    pub fn is_sharp(self) -> bool {
        matches!(self, Accidental::Sharp | Accidental::DoubleSharp) || 
        matches!(self, Accidental::Extreme(n) if n > 0)
    }

    /// Returns true if the accidental is a flat variant (flat or double flat)
    pub fn is_flat(self) -> bool {
        matches!(self, Accidental::Flat | Accidental::DoubleFlat) ||
        matches!(self, Accidental::Extreme(n) if n < 0)
    }

    /// Returns the string representation of the accidental for use as a component in another
    /// string. This means that only non-natural accidentals get written out, as natural is
    /// the default state.
    pub fn component_str(&self) -> String {
        use crate::symbols::*;
        match self {
            Accidental::Flat => FLAT.to_string(),
            Accidental::Sharp => SHARP.to_string(),
            Accidental::Natural => "".to_string(),
            Accidental::DoubleFlat => DOUBLE_FLAT.to_string(),
            Accidental::DoubleSharp => DOUBLE_SHARP.to_string(),
            Accidental::Extreme(level) => {
                if *level > 0 {
                    SHARP.repeat(*level as usize)
                } else if *level < 0 {
                    FLAT.repeat(level.abs() as usize)
                } else {
                    "".to_string()
                }
            }
        }
    }

    /// Returns the accidental as an alteration, meaning that if it's natural, it returns None,
    /// otherwise it returns Some(Accidental).
    pub fn as_alteration(&self) -> Option<Self> {
        match self {
            Accidental::Flat => Some(Accidental::Flat),
            Accidental::Sharp => Some(Accidental::Sharp),
            Accidental::Natural => None,
            Accidental::DoubleFlat => Some(Accidental::DoubleFlat),
            Accidental::DoubleSharp => Some(Accidental::DoubleSharp),
            Accidental::Extreme(0) => None, // Extreme with level 0 is natural
            Accidental::Extreme(_) => Some(*self),
        }
    }

    /// Returns the string representation of the accidental for a specific output format.
    /// This includes natural accidentals, unlike component_str.
    pub fn render_for_format(&self, format: ChordFormat) -> String {
        match (self, format) {
            (Accidental::Flat, ChordFormat::Unicode) => "♭".to_string(),
            (Accidental::Flat, ChordFormat::Ascii) => "b".to_string(),
            (Accidental::Flat, ChordFormat::Html) => "&flat;".to_string(),
            (Accidental::Sharp, ChordFormat::Unicode) => "♯".to_string(),
            (Accidental::Sharp, ChordFormat::Ascii) => "#".to_string(),
            (Accidental::Sharp, ChordFormat::Html) => "&sharp;".to_string(),
            (Accidental::Natural, ChordFormat::Unicode) => "♮".to_string(),
            (Accidental::Natural, ChordFormat::Ascii) => "♮".to_string(),
            (Accidental::Natural, ChordFormat::Html) => "&natural;".to_string(),
            (Accidental::DoubleFlat, ChordFormat::Unicode) => "𝄫".to_string(),
            (Accidental::DoubleFlat, ChordFormat::Ascii) => "bb".to_string(),
            (Accidental::DoubleFlat, ChordFormat::Html) => "&#119083;".to_string(), // 𝄫 Unicode entity
            (Accidental::DoubleSharp, ChordFormat::Unicode) => "𝄪".to_string(),
            (Accidental::DoubleSharp, ChordFormat::Ascii) => "##".to_string(),
            (Accidental::DoubleSharp, ChordFormat::Html) => "&#119082;".to_string(), // 𝄪 Unicode entity
            (Accidental::Extreme(level), ChordFormat::Unicode) => {
                if *level > 0 {
                    "♯".repeat(*level as usize)
                } else if *level < 0 {
                    "♭".repeat(level.abs() as usize)
                } else {
                    "♮".to_string()
                }
            },
            (Accidental::Extreme(level), ChordFormat::Ascii) => {
                if *level > 0 {
                    "#".repeat(*level as usize)
                } else if *level < 0 {
                    "b".repeat(level.abs() as usize)
                } else {
                    "♮".to_string()
                }
            },
            (Accidental::Extreme(level), ChordFormat::Html) => {
                if *level > 0 {
                    "&sharp;".repeat(*level as usize)
                } else if *level < 0 {
                    "&flat;".repeat(level.abs() as usize)
                } else {
                    "&natural;".to_string()
                }
            },
        }
    }

    /// Returns the string representation of the accidental for use as a component in 
    /// another string for a specific output format. Natural accidentals return empty string.
    pub fn component_str_for_format(&self, format: ChordFormat) -> String {
        match self {
            Accidental::Natural => "".to_string(),
            Accidental::Extreme(0) => "".to_string(),
            _ => self.render_for_format(format),
        }
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
            Accidental::Extreme(level) => {
                if *level > 0 {
                    write!(f, "{}", SHARP.repeat(*level as usize))
                } else if *level < 0 {
                    write!(f, "{}", FLAT.repeat(level.abs() as usize))
                } else {
                    write!(f, "{}", NATURAL)
                }
            }
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
