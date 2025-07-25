//! Chord progression system based on Stephen Mugglin's progression map
//!
//! This module implements a graph-based approach to chord progressions, providing
//! tools for analyzing and generating harmonic progressions based on music theory
//! principles encoded in Mugglin's visual progression map.
//!
//! # Overview
//!
//! The progression system distinguishes between:
//! - **Primary nodes** (blue): Diatonic, stable harmonic centers  
//! - **Secondary nodes** (green): Chromatic, transitional chords
//!
//! Progression strength is derived from graph structure:
//! - **Strong**: Explicit arrows showing natural voice leading
//! - **Moderate**: Jumps to primary nodes (stable)
//! - **Weak**: Jumps to secondary nodes (creates tension)
//!
//! # Usage
//!
//! ```rust
//! use chordy::prelude::*;
//!
//! let key = Key::Major(note!("C"));
//! let tonic_chord = Chord::from_str("C").unwrap();
//! let options = key.progression_options(&tonic_chord).unwrap();
//!
//! // Get different types of progressions
//! println!("Strong progressions: {:?}", options.strong);    // IV, V, vi
//! println!("Moderate progressions: {:?}", options.moderate); // ii, iii, vii
//! println!("Weak progressions: {:?}", options.weak);        // bIII, bVI, bVII
//! ```

pub mod graph;
pub mod types;

// Generated static data modules
pub mod chord_progressions;
pub mod major_data;
pub mod minor_data;

// Re-export commonly used types
pub use chord_progressions::{
    select_progression_tier, PROGRESSION_METADATA, TIER_1_PROGRESSIONS, TIER_2_PROGRESSIONS,
    TIER_3_PROGRESSIONS, TIER_4_PROGRESSIONS,
};
pub use graph::{ChordRef, ProgressionGraph};
pub use types::{
    ChordProgressionOptions, NodeType, ProgressionEdge, ProgressionOptions, ProgressionStrength,
};
