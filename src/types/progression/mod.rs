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
//! let options = key.progression_options("I").unwrap();
//!
//! // Get different types of progressions
//! println!("Strong progressions: {:?}", options.strong);    // IV, V, vi
//! println!("Moderate progressions: {:?}", options.moderate); // ii, iii, vii
//! println!("Weak progressions: {:?}", options.weak);        // bIII, bVI, bVII
//! ```

pub mod types;
pub mod graph;

// Generated static data modules
pub mod major_data;
pub mod minor_data;

pub use types::*;
pub use graph::*;

// Re-export commonly used types
pub use types::{
    ProgressionNode, NodeType, ProgressionStrength, ProgressionEdge,
    NodeRef, ProgressionOptions
};
pub use graph::{
    StaticMajorGraph, StaticMinorGraph, ProgressionGraph, ProgressionGraphLike
};