//! Transposition algorithms for musical pitches
//!
//! Provides different strategies for pitch transposition:
//! - Chromatic (semitone-based)
//! - Diatonic (scale-degree based)
//! - Custom enharmonic spellings

use crate::Pitch;

mod chromatic;
pub use chromatic::ChromaticTransposer;

/// Trait for all transposition implementations
pub trait Transposer {
    /// Transposes a pitch by the given interval
    fn transpose(pitch: Pitch, interval: i8) -> Pitch;
    
    /// Returns the transposer's name for debugging
    fn name() -> &'static str;
}
