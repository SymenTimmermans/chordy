//! Core types for chord progression system
//!
//! This module contains the fundamental types used to represent Stephen Mugglin's
//! chord progression graph, including nodes, edges, and connection strength.

use crate::types::{Chord, RomanChord};

/// Classification of chord nodes in the progression graph
///
/// Based on Mugglin's color coding:
/// - Primary (blue): Diatonic chords that form stable harmonic centers
/// - Secondary (green): Chromatic/borrowed chords that create tension and require resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    /// Primary nodes (blue in Mugglin's map) - diatonic, stable harmonic centers
    /// These represent the core tonal framework and can sustain without resolution
    Primary,
    /// Secondary nodes (green in Mugglin's map) - chromatic, transitional
    /// These create tension and have a tendency to resolve back to primary areas
    Secondary,
}

/// Strength of harmonic progression between chords
///
/// Derived from the graph structure rather than explicitly encoded:
/// - Strong: Explicit arrows showing natural voice leading
/// - Moderate: Jumps to primary nodes (stable but less directed)
/// - Weak: Jumps to secondary nodes (creates tension, needs resolution)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgressionStrength {
    /// Strong connections follow explicit arrows - represent natural voice leading
    /// and conventional harmonic progressions (e.g., V→I, ii→V)
    Strong,
    /// Moderate connections are jumps to primary nodes - harmonically stable
    /// but don't follow specific voice leading patterns
    Moderate,
    /// Weak connections are jumps to secondary nodes - create harmonic tension
    /// and have a tendency to resolve back toward primary areas
    Weak,
}

/// An edge in the progression graph representing a harmonic connection
///
/// Only explicit arrows (strong connections) are stored as edges.
/// Moderate and weak connections are computed dynamically based on node types.
///
/// Uses Copy semantics for efficient storage and manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgressionEdge {
    /// Source chord of the progression
    pub from: RomanChord,
    /// Destination chord of the progression  
    pub to: RomanChord,
}

/// Available progression options from a given chord
///
/// Categorizes all possible next chords by their harmonic relationship strength,
/// allowing users to make informed choices about progression direction.
///
/// Uses owned RomanChords for simplicity since they implement Copy.
#[derive(Debug, Clone)]
pub struct ProgressionOptions {
    /// Strong progressions: follow explicit arrows (natural voice leading)
    /// These represent the most conventional and smooth harmonic motion
    pub strong: Vec<RomanChord>,
    /// Moderate progressions: jump to primary nodes (stable but less directed)
    /// These provide harmonic stability without specific voice leading constraints  
    pub moderate: Vec<RomanChord>,
    /// Weak progressions: jump to secondary nodes (creates tension)
    /// These introduce chromaticism and require eventual resolution to primary areas
    pub weak: Vec<RomanChord>,
}

impl ProgressionOptions {
    /// Create new empty progression options
    pub fn new() -> Self {
        Self {
            strong: Vec::new(),
            moderate: Vec::new(),
            weak: Vec::new(),
        }
    }

    /// Get all progression options regardless of strength
    pub fn all(&self) -> impl Iterator<Item = (RomanChord, ProgressionStrength)> + '_ {
        self.strong
            .iter()
            .map(|&n| (n, ProgressionStrength::Strong))
            .chain(
                self.moderate
                    .iter()
                    .map(|&n| (n, ProgressionStrength::Moderate)),
            )
            .chain(self.weak.iter().map(|&n| (n, ProgressionStrength::Weak)))
    }

    /// Check if there are any progression options available
    pub fn is_empty(&self) -> bool {
        self.strong.is_empty() && self.moderate.is_empty() && self.weak.is_empty()
    }

    /// Get the total number of progression options
    pub fn len(&self) -> usize {
        self.strong.len() + self.moderate.len() + self.weak.len()
    }
}

impl Default for ProgressionOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Available chord progression options from a given chord
///
/// Categorizes all possible next chords by their harmonic relationship strength,
/// using actual Chord objects rather than ProgressionNodes for direct usability.
#[derive(Debug, Clone)]
pub struct ChordProgressionOptions {
    /// Strong progressions: follow explicit arrows (natural voice leading)
    /// These represent the most conventional and smooth harmonic motion
    pub strong: Vec<Chord>,
    /// Moderate progressions: jump to primary nodes (stable but less directed)
    /// These provide harmonic stability without specific voice leading constraints  
    pub moderate: Vec<Chord>,
    /// Weak progressions: jump to secondary nodes (creates tension)
    /// These introduce chromaticism and require eventual resolution to primary areas
    pub weak: Vec<Chord>,
}

impl ChordProgressionOptions {
    /// Create new empty chord progression options
    pub fn new() -> Self {
        Self {
            strong: Vec::new(),
            moderate: Vec::new(),
            weak: Vec::new(),
        }
    }

    /// Get all chord progression options regardless of strength
    pub fn all(&self) -> impl Iterator<Item = (&Chord, ProgressionStrength)> {
        self.strong
            .iter()
            .map(|c| (c, ProgressionStrength::Strong))
            .chain(
                self.moderate
                    .iter()
                    .map(|c| (c, ProgressionStrength::Moderate)),
            )
            .chain(self.weak.iter().map(|c| (c, ProgressionStrength::Weak)))
    }

    /// Check if there are any chord progression options available
    pub fn is_empty(&self) -> bool {
        self.strong.is_empty() && self.moderate.is_empty() && self.weak.is_empty()
    }

    /// Get the total number of chord progression options
    pub fn len(&self) -> usize {
        self.strong.len() + self.moderate.len() + self.weak.len()
    }
}

impl Default for ChordProgressionOptions {
    fn default() -> Self {
        Self::new()
    }
}
