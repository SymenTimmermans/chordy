/// Error type for parsing failures
///
/// Included in the crate prelude
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Error when an invalid accidental string is provided
    InvalidAccidental(String),

    /// Error when an invalid note name is provided
    InvalidNoteName(String),

    /// Error when an invalid chord symbol is provided
    InvalidChordSymbol(String),

    /// Error when an invalid scale type is provided
    InvalidScaleType(String),

    /// Error when a string doesn't match any known pattern
    UnrecognizedFormat(String),

    /// Error when an invalid pitch is provided
    InvalidPitch(String),
    InvalidInterval(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidAccidental(s) => write!(f, "Invalid accidental: '{}'", s),
            ParseError::InvalidNoteName(s) => write!(f, "Invalid note name: '{}'", s),
            ParseError::InvalidChordSymbol(s) => write!(f, "Invalid chord symbol: '{}'", s),
            ParseError::InvalidScaleType(s) => write!(f, "Invalid scale type: '{}'", s),
            ParseError::UnrecognizedFormat(s) => write!(f, "Unrecognized format: '{}'", s),
            ParseError::InvalidPitch(s) => write!(f, "Invalid pitch: '{}'", s),
            ParseError::InvalidInterval(s) => write!(f, "Invalid interval: '{}'", s),
        }
    }
}

impl std::error::Error for ParseError {}

/// Included in the crate prelude
#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    /// Error when an invalid scale degree is created
    InvalidScaleDegree(u8),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::InvalidScaleDegree(s) => write!(f, "Invalid scale degree: '{}'", s),
        }
    }
}

impl std::error::Error for TypeError {}
