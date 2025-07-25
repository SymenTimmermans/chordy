//! Simplified progression graph implementation
//!
//! This module contains a unified progression graph that works with both
//! static data generated from Stephen Mugglin's progression maps and
//! runtime-constructed graphs using Copy semantics for RomanChord.

use super::{NodeType, ProgressionEdge, ProgressionOptions};
use crate::types::RomanChord;
use std::collections::HashMap;

/// Unified progression graph using Copy semantics
///
/// This single graph type works with both static progression data
/// and runtime-constructed progressions, eliminating the need for
/// separate static and dynamic graph types.
#[derive(Debug, Clone)]
pub struct ProgressionGraph {
    /// All chords in the graph with their associated node types
    nodes: HashMap<String, (RomanChord, NodeType)>,
    /// Strong progression connections (explicit arrows)
    edges: Vec<ProgressionEdge>,
}

impl ProgressionGraph {
    /// Create a new empty progression graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Create a progression graph from static major key data
    pub fn major() -> Self {
        use crate::types::progression::major_data::{get_node_types, ALL_EDGES, ALL_NODES};

        let mut graph = Self::new();
        let node_types = get_node_types();

        // Add all nodes with their types using full display name as ID
        for &node in ALL_NODES {
            let node_type = node_types.get(node).copied().unwrap_or(NodeType::Primary);
            // Use the full display format to get proper chord symbols like I7, ii9, etc.
            let id = format!("{}", node);
            graph.nodes.insert(id, (*node, node_type));
        }

        // Add all edges
        for &edge in ALL_EDGES {
            graph.edges.push(*edge);
        }

        graph
    }

    /// Create a progression graph from static minor key data
    pub fn minor() -> Self {
        use crate::types::progression::minor_data::{get_node_types, ALL_EDGES, ALL_NODES};

        let mut graph = Self::new();
        let node_types = get_node_types();

        // Add all nodes with their types using full display name as ID
        for &node in ALL_NODES {
            let node_type = node_types.get(node).copied().unwrap_or(NodeType::Primary);
            // Use the full display format to get proper chord symbols like i7, ii9, etc.
            let id = format!("{}", node);
            graph.nodes.insert(id, (*node, node_type));
        }

        // Add all edges
        for &edge in ALL_EDGES {
            graph.edges.push(*edge);
        }

        graph
    }

    /// Add a chord to the graph
    pub fn add_node(&mut self, id: String, chord: RomanChord, node_type: NodeType) {
        self.nodes.insert(id, (chord, node_type));
    }

    /// Add a progression edge between two chords
    pub fn add_edge(&mut self, from: RomanChord, to: RomanChord) {
        self.edges.push(ProgressionEdge { from, to });
    }

    /// Add a progression edge by chord IDs
    pub fn add_edge_by_id(&mut self, from_id: &str, to_id: &str) -> Result<(), String> {
        let from_chord = self
            .get_node(from_id)
            .ok_or_else(|| format!("Node not found: {}", from_id))?;
        let to_chord = self
            .get_node(to_id)
            .ok_or_else(|| format!("Node not found: {}", to_id))?;

        self.add_edge(from_chord, to_chord);
        Ok(())
    }

    /// Get a chord by its string identifier
    pub fn get_node(&self, id: &str) -> Option<RomanChord> {
        self.nodes.get(id).map(|(chord, _)| *chord)
    }

    /// Get progressions that can be made FROM a given chord
    pub fn progressions_from(&self, from: impl Into<ChordRef>) -> Option<ProgressionOptions> {
        let from_chord = match from.into() {
            ChordRef::Chord(chord) => chord,
            ChordRef::Id(id) => self.get_node(&id)?,
        };

        let mut options = ProgressionOptions::new();

        // Find strong connections (explicit edges)
        for edge in &self.edges {
            if edge.from == from_chord {
                options.strong.push(edge.to);
            }
        }

        // Find moderate and weak connections (jumps to all other nodes)
        // Collect and sort nodes for deterministic iteration
        let mut node_entries: Vec<_> = self.nodes.values().collect();
        node_entries.sort_by_key(|(chord, _)| {
            (
                chord.root,
                chord
                    .bass
                    .map(|(bass, bass_type)| (bass, format!("{:?}", bass_type))),
                chord.intervals.len(),
                format!("{:?}", chord.intervals.as_slice()),
            )
        });

        for (chord, node_type) in node_entries {
            if *chord == from_chord {
                continue; // Skip self
            }

            // Skip if already in strong connections
            if options.strong.contains(chord) {
                continue;
            }

            match node_type {
                NodeType::Primary => options.moderate.push(*chord),
                NodeType::Secondary => options.weak.push(*chord),
            }
        }

        // Sort all lists for deterministic ordering
        // Use a comprehensive sort key including bass information
        let sort_key = |chord: &RomanChord| {
            (
                chord.root,
                chord
                    .bass
                    .map(|(bass, bass_type)| (bass, format!("{:?}", bass_type))),
                chord.intervals.len(),
                format!("{:?}", chord.intervals.as_slice()),
            )
        };
        options.strong.sort_by_key(sort_key);
        options.moderate.sort_by_key(sort_key);
        options.weak.sort_by_key(sort_key);

        Some(options)
    }

    /// Get progressions that lead TO a given chord
    pub fn progressions_to(&self, to: impl Into<ChordRef>) -> Option<ProgressionOptions> {
        let to_chord = match to.into() {
            ChordRef::Chord(chord) => chord,
            ChordRef::Id(id) => self.get_node(&id)?,
        };

        let mut options = ProgressionOptions::new();

        // Find strong connections (explicit edges that lead to this chord)
        for edge in &self.edges {
            if edge.to == to_chord {
                options.strong.push(edge.from);
            }
        }

        // Find moderate and weak connections (any chord that could jump to this chord)
        // Collect and sort nodes for deterministic iteration
        let mut node_entries: Vec<_> = self.nodes.values().collect();
        node_entries.sort_by_key(|(chord, _)| {
            (
                chord.root,
                chord
                    .bass
                    .map(|(bass, bass_type)| (bass, format!("{:?}", bass_type))),
                chord.intervals.len(),
                format!("{:?}", chord.intervals.as_slice()),
            )
        });

        for (chord, node_type) in node_entries {
            if *chord == to_chord {
                continue; // Skip self
            }

            // Skip if already in strong connections
            if options.strong.contains(chord) {
                continue;
            }

            // All chords can potentially lead to any chord via jumps
            match node_type {
                NodeType::Primary => options.moderate.push(*chord),
                NodeType::Secondary => options.weak.push(*chord),
            }
        }

        // Sort all lists for deterministic ordering
        // Use a comprehensive sort key including bass information
        let sort_key = |chord: &RomanChord| {
            (
                chord.root,
                chord
                    .bass
                    .map(|(bass, bass_type)| (bass, format!("{:?}", bass_type))),
                chord.intervals.len(),
                format!("{:?}", chord.intervals.as_slice()),
            )
        };
        options.strong.sort_by_key(sort_key);
        options.moderate.sort_by_key(sort_key);
        options.weak.sort_by_key(sort_key);

        Some(options)
    }

    /// Get all chords in the graph
    pub fn nodes(&self) -> impl Iterator<Item = RomanChord> + '_ {
        self.nodes.values().map(|(chord, _)| *chord)
    }

    /// Get the total number of chords in the graph
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the total number of edges in the graph  
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Check if the graph contains any chords
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Check if an edge exists between two chords
    pub fn has_edge(&self, from: RomanChord, to: RomanChord) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.from == from && edge.to == to)
    }

    /// Remove an edge between two chords
    pub fn remove_edge(&mut self, from: RomanChord, to: RomanChord) -> bool {
        let initial_len = self.edges.len();
        self.edges
            .retain(|edge| !(edge.from == from && edge.to == to));
        self.edges.len() < initial_len
    }
}

impl Default for ProgressionGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper enum for flexible chord reference in API methods
#[derive(Debug, Clone)]
pub enum ChordRef {
    /// Direct chord reference
    Chord(RomanChord),
    /// String ID requiring lookup
    Id(String),
}

impl From<RomanChord> for ChordRef {
    fn from(chord: RomanChord) -> Self {
        ChordRef::Chord(chord)
    }
}

impl From<&RomanChord> for ChordRef {
    fn from(chord: &RomanChord) -> Self {
        ChordRef::Chord(*chord)
    }
}

impl From<&str> for ChordRef {
    fn from(id: &str) -> Self {
        ChordRef::Id(id.to_string())
    }
}

impl From<String> for ChordRef {
    fn from(id: String) -> Self {
        ChordRef::Id(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Accidental, Interval, IntervalSet, RomanDegree, RomanNumeral};

    fn create_test_chord() -> RomanChord {
        RomanChord {
            root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
            intervals: IntervalSet::from_slice(&[
                Interval::PERFECT_UNISON,
                Interval::MAJOR_THIRD,
                Interval::PERFECT_FIFTH,
            ]),
            bass: None,
        }
    }

    #[test]
    fn test_empty_graph_creation() {
        let graph = ProgressionGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_node() {
        let mut graph = ProgressionGraph::new();
        let chord = create_test_chord();

        graph.add_node("I".to_string(), chord, NodeType::Primary);

        assert_eq!(graph.node_count(), 1);
        assert!(!graph.is_empty());
        assert_eq!(graph.get_node("I"), Some(chord));
    }

    #[test]
    fn test_major_graph_creation() {
        let graph = ProgressionGraph::major();
        assert!(graph.node_count() > 0);
        assert!(!graph.is_empty());
    }

    #[test]
    fn test_minor_graph_creation() {
        let graph = ProgressionGraph::minor();
        assert!(graph.node_count() > 0);
        assert!(!graph.is_empty());
    }

    #[test]
    fn test_progressions_from() {
        let graph = ProgressionGraph::major();
        let options = graph.progressions_from("I");
        assert!(options.is_some());

        let options = options.unwrap();
        assert!(!options.is_empty());
    }
}
