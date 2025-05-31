//! The `chordy` prelude
//!
//! Provides convenient access to commonly used types and traits:
//!
//! ```
//! use chordy::prelude::*;
//! ```

// Core music theory types
pub use crate::types::{
    Accidental,

    // Chord types
    Chord,

    Key,
    Letter,
    NoteName,
    // Basic note types
    Pitch,
    // Scale/key types
    Scale,
    ScaleDefinition,
    ScaleDegree,
};

// Error handling
pub use crate::error::ParseError;
pub use crate::error::TypeError;

// Musical symbols (ASCII or UTF-8 based on feature flag)
pub use crate::symbols::{
    A,
    B, // Note letters for consistency
    C,
    D,
    DOUBLE_FLAT,
    DOUBLE_SHARP,
    E,
    F,
    FLAT,
    G,
    NATURAL,
    SHARP,
};

// Essential traits
pub mod traits {
    pub use std::fmt::Display;
    pub use std::str::FromStr;
    pub use crate::traits::*;
}

// Macros
pub use crate::pitch;
