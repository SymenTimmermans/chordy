//! The `chordy` prelude - one import gives all core functionality
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! // All these work without additional imports:
//! let mut chord = Chord::major(note!("C"));
//! chord.transpose(Interval::MAJOR_THIRD);
//!
//! let scale = Scale::from_definition(note!("C"), scales::IONIAN);
//! scale.triads().for_each(|triad| println!("{}", triad));
//! ```

// Core Types
pub use crate::types::{
    Accidental, Chord, HarmonicFunction, Interval, Key, 
    Letter, NoteName, Pitch, Scale, ScaleDefinition, ScaleDegree
};

// All Musical Traits
pub use crate::traits::*;

// Error Types
pub use crate::error::{ParseError, TypeError};

// Constants and Symbols
pub use crate::symbols::*;
pub use crate::scales;

// Essential Std Traits
pub use std::fmt::{Debug, Display};
pub use std::str::FromStr;

// Macros
pub use crate::{note, pitch};

