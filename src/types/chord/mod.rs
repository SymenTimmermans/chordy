//! Chord representation with support for inversions and slash chords
//!
//! This module provides comprehensive support for chord representation including:
//! - Root position chords
//! - Classical inversions (chord tones in bass)
//! - Slash chords (arbitrary bass notes)
//!
//! # Bass Note Concepts
//!
//! ## Root Position
//! A chord in root position has its root note as the lowest-sounding note (bass).
//! This is the default state for all chords.
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//! assert_eq!(c_major.bass_note(), note!("C"));  // Root is in bass
//! assert!(!c_major.is_inverted());
//! assert!(!c_major.is_slash_chord());
//! ```
//!
//! ## Classical Inversions
//! An inversion occurs when a chord tone other than the root is in the bass.
//! This creates different harmonic colors and voice leading possibilities.
//!
//! - **First Inversion**: Third in bass (e.g., C major = C/E)
//! - **Second Inversion**: Fifth in bass (e.g., C major = C/G)
//! - **Third Inversion**: Seventh in bass (e.g., C7 = C7/Bb)
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//!
//! // First inversion - third (E) in bass
//! let first_inv = c_major.with_inversion(1);
//! assert_eq!(first_inv.bass_note(), note!("E"));
//! assert!(first_inv.is_inverted());
//! assert_eq!(first_inv.inversion_number(), Some(1));
//!
//! // Second inversion - fifth (G) in bass
//! let second_inv = c_major.with_inversion(2);
//! assert_eq!(second_inv.bass_note(), note!("G"));
//! assert_eq!(second_inv.inversion_number(), Some(2));
//! ```
//!
//! ## Slash Chords
//! Slash chords have an arbitrary bass note that may or may not be a chord tone.
//! They're common in popular music and jazz for creating specific bass lines
//! or harmonic colors.
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let c_major = Chord::major(note!("C"));
//!
//! // C/F - C major with F in bass (not a chord tone)
//! let c_slash_f = c_major.with_slash_bass(note!("F"));
//! assert_eq!(c_slash_f.bass_note(), note!("F"));
//! assert!(c_slash_f.is_slash_chord());
//! assert!(!c_slash_f.is_inverted());
//!
//! // C/E - Could be written as slash chord or first inversion
//! let c_slash_e = c_major.with_slash_bass(note!("E"));
//! assert!(c_slash_e.is_slash_chord());  // Explicitly marked as slash
//! ```
//!
//! ## BassType Distinction
//! The [`BassType`] enum distinguishes between inversions and slash chords:
//!
//! - [`BassType::Inversion(n)`]: Classical inversion with chord tone in bass
//! - [`BassType::Slash`]: Arbitrary bass note (may or may not be chord tone)
//!
//! This distinction is important for:
//! - Music analysis and theory
//! - Voice leading algorithms
//! - Chord progression analysis
//! - Notation and display formatting
//!
//! ## Practical Usage
//!
//! ### Creating Bass Chords
//! ```rust
//! use chordy::prelude::*;
//!
//! let chord = Chord::major(note!("C"));
//!
//! // Method chaining for fluent interface
//! let complex_chord = chord
//!     .with_interval(Interval::MINOR_SEVENTH)  // Add 7th
//!     .with_inversion(1);                      // First inversion
//!
//! // Convenience methods
//! let first_inv = chord.in_first_inversion();
//! let second_inv = chord.in_second_inversion();
//! ```
//!
//! ### Bass Note Preservation
//! Bass notes are preserved during interval mutations:
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let mut chord = Chord::major(note!("C")).with_slash_bass(note!("F"));
//! let original_bass = chord.bass_note();
//!
//! // Modify intervals - bass is preserved
//! chord.add_interval(Interval::MINOR_SEVENTH);
//! chord.remove_interval(Interval::PERFECT_FIFTH);
//!
//! assert_eq!(chord.bass_note(), original_bass);  // Still F in bass
//! ```

mod analysis;
mod construction;
mod core;
mod inversions;
mod operations;
mod parsing;
mod voicing;

pub mod naming;
pub use naming::*;

mod quality;
pub use quality::ChordQuality;

// Re-export all public types and methods
pub use analysis::HarmonicFunction;
pub use construction::*;
pub use core::{BassType, Chord};
pub use inversions::*;
pub use operations::*;
pub use parsing::*;
pub use voicing::*;

// Import dependencies used in doc tests
#[cfg(test)]
use crate::{note, Interval, VoicingConfig, VoicingStyle};