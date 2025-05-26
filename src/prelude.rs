//! The `chordy` prelude
//!
//! Provides convenient access to commonly used types and traits:
//! 
//! ```
//! use chordy::prelude::*;
//! ```

// Core music theory types
pub use crate::types::{
    // Basic note types
    Pitch,
    NoteName,
    Letter,
    Accidental,

    // Chord types
    Chord,
    ChordQuality,
    ChordExtension,
    SeventhType,
    NinthType,
    EleventhType,
    ThirteenthType,

    // Scale/key types
    Scale,
    ScaleDefinition,
    ScaleDegree,
    Key,
};

// Error handling
pub use crate::error::ParseError;
pub use crate::error::TypeError;

// Musical symbols (ASCII or UTF-8 based on feature flag)
pub use crate::symbols::{
    FLAT,
    SHARP, 
    DOUBLE_FLAT,
    DOUBLE_SHARP,
    NATURAL,
    C, D, E, F, G, A, B  // Note letters for consistency
};

// Essential traits
pub mod traits {
    pub use std::str::FromStr;
    pub use std::fmt::Display;
}

// Macros
pub use crate::pitch;
