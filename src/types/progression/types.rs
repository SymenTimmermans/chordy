//! Core types for chord progression system
//!
//! This module contains the fundamental types used to represent Stephen Mugglin's
//! chord progression graph, including nodes, edges, and connection strength.

use crate::types::{RomanChord, RomanNumeral, Interval};

/// A node in the progression graph representing a chord or chord family
/// 
/// Each node represents a specific chord variant (e.g., "ii7", "V9") with
/// a fixed roman numeral representation and harmonic function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressionNode {
    /// Unique identifier for this node (e.g., "ii_7", "V_9")
    pub id: &'static str,
    /// Human-readable display name (e.g., "ii7", "V9")
    pub display_name: &'static str,
    /// Whether this is a primary (stable) or secondary (transitional) node
    pub node_type: NodeType,
    /// The roman numeral root of this chord
    pub roman_numeral: RomanNumeral,
    /// The intervals that define this chord variant
    pub intervals: &'static [Interval],
    /// Base harmonic function without extensions (e.g., "ii", "V")
    pub base_function: &'static str,
}

impl ProgressionNode {
    /// Convert this progression node to a RomanChord
    pub fn to_roman_chord(&self) -> RomanChord {
        RomanChord::new(self.roman_numeral, self.intervals.to_vec())
    }
}

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
#[derive(Debug)]
pub struct ProgressionEdge {
    /// Source node of the progression
    pub from: &'static ProgressionNode,
    /// Destination node of the progression  
    pub to: &'static ProgressionNode,
}

/// Reference to a progression node that can be either static or dynamic
/// 
/// This allows the API to accept both compile-time static references (for performance)
/// and runtime string lookups (for convenience), with automatic conversion.
#[derive(Debug, Clone)]
pub enum NodeRef<'a> {
    /// Direct reference to a static node - zero-cost lookup
    Static(&'a ProgressionNode),
    /// String identifier requiring runtime lookup
    Dynamic(String),
}

impl<'a> From<&'a ProgressionNode> for NodeRef<'a> {
    fn from(node: &'a ProgressionNode) -> Self {
        NodeRef::Static(node)
    }
}

impl<'a> From<&str> for NodeRef<'a> {
    fn from(id: &str) -> Self {
        NodeRef::Dynamic(id.to_string())
    }
}

impl<'a> From<String> for NodeRef<'a> {
    fn from(id: String) -> Self {
        NodeRef::Dynamic(id)
    }
}

/// Available progression options from a given chord
/// 
/// Categorizes all possible next chords by their harmonic relationship strength,
/// allowing users to make informed choices about progression direction.
#[derive(Debug, Clone)]
pub struct ProgressionOptions<'a> {
    /// Strong progressions: follow explicit arrows (natural voice leading)
    /// These represent the most conventional and smooth harmonic motion
    pub strong: Vec<&'a ProgressionNode>,
    /// Moderate progressions: jump to primary nodes (stable but less directed)
    /// These provide harmonic stability without specific voice leading constraints  
    pub moderate: Vec<&'a ProgressionNode>,
    /// Weak progressions: jump to secondary nodes (creates tension)
    /// These introduce chromaticism and require eventual resolution to primary areas
    pub weak: Vec<&'a ProgressionNode>,
}

impl<'a> ProgressionOptions<'a> {
    /// Create new empty progression options
    pub fn new() -> Self {
        Self {
            strong: Vec::new(),
            moderate: Vec::new(),
            weak: Vec::new(),
        }
    }
    
    /// Get all progression options regardless of strength
    pub fn all(&'a self) -> impl Iterator<Item = (&'a ProgressionNode, ProgressionStrength)> + 'a {
        self.strong.iter().map(|&n| (n, ProgressionStrength::Strong))
            .chain(self.moderate.iter().map(|&n| (n, ProgressionStrength::Moderate)))
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

impl<'a> Default for ProgressionOptions<'a> {
    fn default() -> Self {
        Self::new()
    }
}