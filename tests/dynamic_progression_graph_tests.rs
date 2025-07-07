//! Comprehensive tests for dynamic progression graphs
//!
//! This module tests the runtime-buildable progression graph functionality,
//! including unified API compatibility with static graphs and real-world
//! progression scenarios.

use chordy::types::progression::{
    ProgressionGraph, ProgressionGraphLike, ProgressionNode, NodeType,
    StaticMajorGraph
};
use chordy::types::{RomanNumeral, RomanDegree, Accidental, Interval};

// Static interval arrays for testing
static MAJOR_TRIAD_INTERVALS: [Interval; 3] = [
    Interval::PERFECT_UNISON,
    Interval::MAJOR_THIRD,
    Interval::PERFECT_FIFTH,
];

static MINOR_TRIAD_INTERVALS: [Interval; 3] = [
    Interval::PERFECT_UNISON,
    Interval::MINOR_THIRD,
    Interval::PERFECT_FIFTH,
];

static DOMINANT_SEVENTH_INTERVALS: [Interval; 4] = [
    Interval::PERFECT_UNISON,
    Interval::MAJOR_THIRD,
    Interval::PERFECT_FIFTH,
    Interval::MINOR_SEVENTH,
];

static UNISON_ONLY: [Interval; 1] = [
    Interval::PERFECT_UNISON,
];

/// Helper function to create a basic major I chord node
fn create_i_major_node() -> ProgressionNode {
    ProgressionNode {
        id: "I",
        node_type: NodeType::Primary,
        roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        intervals: &MAJOR_TRIAD_INTERVALS,
    }
}

/// Helper function to create a ii minor chord node  
fn create_ii_minor_node() -> ProgressionNode {
    ProgressionNode {
        id: "ii",
        node_type: NodeType::Primary,
        roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
        intervals: &MINOR_TRIAD_INTERVALS,
    }
}

/// Helper function to create a V dominant chord node
fn create_v_dominant_node() -> ProgressionNode {
    ProgressionNode {
        id: "V",
        node_type: NodeType::Primary,
        roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
        intervals: &MAJOR_TRIAD_INTERVALS,
    }
}

/// Helper function to create a V7 dominant seventh chord node
fn create_v7_dominant_node() -> ProgressionNode {
    ProgressionNode {
        id: "V7",
        node_type: NodeType::Primary,
        roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
        intervals: &DOMINANT_SEVENTH_INTERVALS,
    }
}

/// Helper function to create a bVII secondary chord node (borrowed from mixolydian)
fn create_bvii_secondary_node() -> ProgressionNode {
    ProgressionNode {
        id: "bVII",
        node_type: NodeType::Secondary,
        roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Flat),
        intervals: &MAJOR_TRIAD_INTERVALS,
    }
}

/// Helper to create a VI minor chord (vi in major keys) - used across modules
fn create_vi_minor_node() -> ProgressionNode {
    ProgressionNode {
        id: "vi",
        node_type: NodeType::Primary,
        roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
        intervals: &MINOR_TRIAD_INTERVALS,
    }
}

#[cfg(test)]
mod basic_functionality {
    use super::*;

    #[test]
    fn test_empty_graph_creation() {
        let graph = ProgressionGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_single_node() {
        let mut graph = ProgressionGraph::new();
        let i_chord = create_i_major_node();
        
        graph.add_node(i_chord);
        
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
        assert!(!graph.is_empty());
    }

    #[test]
    fn test_node_lookup() {
        let mut graph = ProgressionGraph::new();
        let i_chord = create_i_major_node();
        
        graph.add_node(i_chord);
        
        let found_node = graph.get_node("I");
        assert!(found_node.is_some());
        assert_eq!(found_node.unwrap().display_name(), "I");
        
        // Test lookup for non-existent node
        let missing_node = graph.get_node("vi");
        assert!(missing_node.is_none());
    }

    #[test]
    fn test_add_multiple_nodes() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_ii_minor_node());
        graph.add_node(create_v_dominant_node());
        
        assert_eq!(graph.node_count(), 3);
        assert!(graph.get_node("I").is_some());
        assert!(graph.get_node("ii").is_some());
        assert!(graph.get_node("V").is_some());
    }

    #[test]
    fn test_node_iteration() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        let nodes: Vec<_> = graph.nodes().collect();
        assert_eq!(nodes.len(), 2);
        
        let display_names: Vec<_> = nodes.iter().map(|n| n.display_name()).collect();
        assert!(display_names.contains(&"I"));
        assert!(display_names.contains(&"V"));
    }
}

#[cfg(test)]
mod edge_functionality {
    use super::*;

    #[test]
    fn test_add_edge_success() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        let result = graph.add_edge("I", "V");
        assert!(result.is_ok());
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_add_edge_missing_from_node() {
        let mut graph = ProgressionGraph::new();
        graph.add_node(create_v_dominant_node());
        
        let result = graph.add_edge("I", "V");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Node not found: I"));
    }

    #[test]
    fn test_add_edge_missing_to_node() {
        let mut graph = ProgressionGraph::new();
        graph.add_node(create_i_major_node());
        
        let result = graph.add_edge("I", "V");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Node not found: V"));
    }

    #[test]
    fn test_add_duplicate_edge() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        assert!(graph.add_edge("I", "V").is_ok());
        
        let result = graph.add_edge("I", "V");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Edge already exists"));
    }

    #[test]
    fn test_has_edge() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_ii_minor_node());
        
        assert!(!graph.has_edge("I", "V"));
        graph.add_edge("I", "V").unwrap();
        assert!(graph.has_edge("I", "V"));
        assert!(!graph.has_edge("V", "I")); // Direction matters
        assert!(!graph.has_edge("I", "ii")); // No edge exists
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        graph.add_edge("I", "V").unwrap();
        assert!(graph.has_edge("I", "V"));
        
        let result = graph.remove_edge("I", "V");
        assert!(result.is_ok());
        assert!(!graph.has_edge("I", "V"));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_edge() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        let result = graph.remove_edge("I", "V");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Edge not found"));
    }
}

#[cfg(test)]
mod progression_analysis {
    use super::*;

    #[test]
    fn test_progression_options_empty_graph() {
        let graph = ProgressionGraph::new();
        let options = graph.progression_options("I");
        assert!(options.is_none());
    }

    #[test]
    fn test_progression_options_single_node() {
        let mut graph = ProgressionGraph::new();
        graph.add_node(create_i_major_node());
        
        let options = graph.progression_options("I").unwrap();
        assert!(options.strong.is_empty());
        assert!(options.moderate.is_empty());
        assert!(options.weak.is_empty());
        assert!(options.is_empty());
    }

    #[test]
    fn test_strong_progressions() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_ii_minor_node());
        
        // Add explicit edges (strong connections)
        graph.add_edge("I", "V").unwrap();
        graph.add_edge("ii", "V").unwrap();
        
        let options = graph.progression_options("I").unwrap();
        assert_eq!(options.strong.len(), 1);
        assert_eq!(options.strong[0].display_name(), "V");
        
        let options_ii = graph.progression_options("ii").unwrap();
        assert_eq!(options_ii.strong.len(), 1);
        assert_eq!(options_ii.strong[0].display_name(), "V");
    }

    #[test]
    fn test_moderate_progressions() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_ii_minor_node());
        
        // No explicit edges, so all primary nodes become moderate options
        let options = graph.progression_options("I").unwrap();
        assert!(options.strong.is_empty());
        assert_eq!(options.moderate.len(), 2); // V and ii are primary
        
        let moderate_names: Vec<_> = options.moderate.iter().map(|n| n.display_name()).collect();
        assert!(moderate_names.contains(&"V"));
        assert!(moderate_names.contains(&"ii"));
    }

    #[test]
    fn test_weak_progressions() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_bvii_secondary_node());
        
        let options = graph.progression_options("I").unwrap();
        assert!(options.strong.is_empty());
        assert_eq!(options.moderate.len(), 1); // V is primary
        assert_eq!(options.weak.len(), 1); // bVII is secondary
        
        assert_eq!(options.moderate[0].display_name(), "V");
        assert_eq!(options.weak[0].display_name(), "bVII");
    }

    #[test]
    fn test_mixed_progression_strengths() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_ii_minor_node());
        graph.add_node(create_bvii_secondary_node());
        
        // Add one strong connection
        graph.add_edge("I", "V").unwrap();
        
        let options = graph.progression_options("I").unwrap();
        assert_eq!(options.strong.len(), 1); // V (explicit edge)
        assert_eq!(options.moderate.len(), 1); // ii (primary, no edge)
        assert_eq!(options.weak.len(), 1); // bVII (secondary)
        
        assert_eq!(options.strong[0].display_name(), "V");
        assert_eq!(options.moderate[0].display_name(), "ii");
        assert_eq!(options.weak[0].display_name(), "bVII");
    }

    #[test]
    fn test_progression_options_all_iterator() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        graph.add_node(create_bvii_secondary_node());
        
        graph.add_edge("I", "V").unwrap();
        
        let options = graph.progression_options("I").unwrap();
        let all_options: Vec<_> = options.all().collect();
        
        assert_eq!(all_options.len(), 2);
        
        // Check that we have one strong and one weak option
        let strong_count = all_options.iter().filter(|(_, strength)| {
            matches!(strength, chordy::types::progression::ProgressionStrength::Strong)
        }).count();
        let weak_count = all_options.iter().filter(|(_, strength)| {
            matches!(strength, chordy::types::progression::ProgressionStrength::Weak)
        }).count();
        
        assert_eq!(strong_count, 1);
        assert_eq!(weak_count, 1);
    }
}

#[cfg(test)]
mod real_world_progressions {
    use super::*;

    /// Helper to create a VI minor chord (vi in major keys)
    fn create_vi_minor_node() -> ProgressionNode {
        ProgressionNode {
            id: "vi",
            node_type: NodeType::Primary,
            roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
            intervals: &MINOR_TRIAD_INTERVALS,
        }
    }

    /// Helper to create a IV major chord
    fn create_iv_major_node() -> ProgressionNode {
        ProgressionNode {
            id: "IV",
            node_type: NodeType::Primary,
            roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
            intervals: &MAJOR_TRIAD_INTERVALS,
        }
    }

    #[test]
    fn test_ii_v_i_jazz_progression() {
        let mut graph = ProgressionGraph::new();
        
        // Build classic ii-V-I progression
        graph.add_node(create_ii_minor_node());
        graph.add_node(create_v7_dominant_node());
        graph.add_node(create_i_major_node());
        
        // Add the classic voice leading edges
        graph.add_edge("ii", "V7").unwrap();
        graph.add_edge("V7", "I").unwrap();
        
        // Test ii → V7 (strong)
        let ii_options = graph.progression_options("ii").unwrap();
        assert_eq!(ii_options.strong.len(), 1);
        assert_eq!(ii_options.strong[0].display_name(), "V7");
        
        // Test V7 → I (strong)
        let v7_options = graph.progression_options("V7").unwrap();
        assert_eq!(v7_options.strong.len(), 1);
        assert_eq!(v7_options.strong[0].display_name(), "I");
        
        // Verify the progression chain works
        assert!(graph.has_edge("ii", "V7"));
        assert!(graph.has_edge("V7", "I"));
    }

    #[test]
    fn test_vi_iv_i_v_pop_progression() {
        let mut graph = ProgressionGraph::new();
        
        // Build the vi-IV-I-V "pop progression"
        graph.add_node(create_vi_minor_node());
        graph.add_node(create_iv_major_node());
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        // Add the cycle edges
        graph.add_edge("vi", "IV").unwrap();
        graph.add_edge("IV", "I").unwrap();
        graph.add_edge("I", "V").unwrap();
        graph.add_edge("V", "vi").unwrap(); // Back to start
        
        // Test each step of the progression
        let vi_options = graph.progression_options("vi").unwrap();
        assert!(vi_options.strong.iter().any(|n| n.display_name() == "IV"));
        
        let iv_options = graph.progression_options("IV").unwrap();
        assert!(iv_options.strong.iter().any(|n| n.display_name() == "I"));
        
        let i_options = graph.progression_options("I").unwrap();
        assert!(i_options.strong.iter().any(|n| n.display_name() == "V"));
        
        let v_options = graph.progression_options("V").unwrap();
        assert!(v_options.strong.iter().any(|n| n.display_name() == "vi"));
    }

    #[test]
    fn test_borrowed_chord_progressions() {
        let mut graph = ProgressionGraph::new();
        
        graph.add_node(create_i_major_node());
        graph.add_node(create_bvii_secondary_node()); // Borrowed from mixolydian
        graph.add_node(create_v_dominant_node());
        
        // bVII often resolves to I (modal resolution)
        graph.add_edge("bVII", "I").unwrap();
        // I can go to bVII for modal color
        graph.add_edge("I", "bVII").unwrap();
        
        let i_options = graph.progression_options("I").unwrap();
        assert!(i_options.strong.iter().any(|n| n.display_name() == "bVII"));
        
        let bvii_options = graph.progression_options("bVII").unwrap();
        assert!(bvii_options.strong.iter().any(|n| n.display_name() == "I"));
        
        // V should be available as moderate (jump to primary)
        assert!(i_options.moderate.iter().any(|n| n.display_name() == "V"));
    }
}

#[cfg(test)]
mod unified_api_compatibility {
    use super::*;

    #[test]
    fn test_trait_method_consistency() {
        // Test that both static and dynamic graphs implement the same trait methods
        let static_graph = StaticMajorGraph::new();
        let dynamic_graph = ProgressionGraph::new();
        
        // Both should implement ProgressionGraphLike
        assert_eq!(static_graph.node_count() > 0, true); // Static has predefined nodes
        assert_eq!(dynamic_graph.node_count(), 0); // Dynamic starts empty
        
        assert_eq!(static_graph.edge_count() > 0, true); // Static has predefined edges
        assert_eq!(dynamic_graph.edge_count(), 0); // Dynamic starts empty
        
        // Both should support node lookup (though empty dynamic returns None)
        assert!(static_graph.get_node("I").is_some());
        assert!(dynamic_graph.get_node("I").is_none());
    }

    #[test]
    fn test_progression_options_api_consistency() {
        let static_graph = StaticMajorGraph::new();
        let mut dynamic_graph = ProgressionGraph::new();
        
        // Add same nodes to dynamic graph as exist in static
        dynamic_graph.add_node(create_i_major_node());
        dynamic_graph.add_node(create_v_dominant_node());
        
        // Both should return Some for valid nodes
        let static_options = static_graph.progression_options("I");
        let dynamic_options = dynamic_graph.progression_options("I");
        
        assert!(static_options.is_some());
        assert!(dynamic_options.is_some());
        
        // Both should return None for invalid nodes
        assert!(static_graph.progression_options("INVALID").is_none());
        assert!(dynamic_graph.progression_options("INVALID").is_none());
    }

    #[test]
    fn test_node_iteration_compatibility() {
        let static_graph = StaticMajorGraph::new();
        let mut dynamic_graph = ProgressionGraph::new();
        
        dynamic_graph.add_node(create_i_major_node());
        dynamic_graph.add_node(create_v_dominant_node());
        
        // Both should support iteration
        let static_nodes: Vec<_> = static_graph.nodes().collect();
        let dynamic_nodes: Vec<_> = dynamic_graph.nodes().collect();
        
        assert!(static_nodes.len() > 0);
        assert_eq!(dynamic_nodes.len(), 2);
        
        // Both should have nodes with the same structure
        for node in static_nodes.iter().take(1) {
            assert!(!node.display_name().is_empty());
            assert!(!node.intervals.is_empty());
        }
        
        for node in dynamic_nodes.iter() {
            assert!(!node.display_name().is_empty());
            assert!(!node.intervals.is_empty());
        }
    }

    #[test]
    fn test_empty_state_behavior() {
        let dynamic_graph = ProgressionGraph::new();
        
        // Empty dynamic graph should behave consistently
        assert!(dynamic_graph.is_empty());
        assert_eq!(dynamic_graph.node_count(), 0);
        assert_eq!(dynamic_graph.edge_count(), 0);
        assert!(dynamic_graph.get_node("I").is_none());
        assert!(dynamic_graph.progression_options("I").is_none());
        
        // Empty iterator
        assert_eq!(dynamic_graph.nodes().count(), 0);
    }
}

#[cfg(test)]
mod builder_patterns {
    use super::*;

    #[test] 
    fn test_fluent_graph_building() {
        let mut graph = ProgressionGraph::new();
        
        // Build a complete ii-V-I progression
        graph.add_node(create_ii_minor_node());
        graph.add_node(create_v7_dominant_node());
        graph.add_node(create_i_major_node());
        
        // Add edges to create the progression
        graph.add_edge("ii", "V7").unwrap();
        graph.add_edge("V7", "I").unwrap();
        
        // Verify the complete graph
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        
        // Test the progression works as expected
        let progression_path = vec!["ii", "V7", "I"];
        for i in 0..progression_path.len() - 1 {
            let current = progression_path[i];
            let next = progression_path[i + 1];
            assert!(graph.has_edge(current, next), 
                "Expected edge from {} to {}", current, next);
        }
    }

    #[test]
    fn test_error_resilient_building() {
        let mut graph = ProgressionGraph::new();
        
        // Add nodes first
        graph.add_node(create_i_major_node());
        graph.add_node(create_v_dominant_node());
        
        // Successful edge
        assert!(graph.add_edge("I", "V").is_ok());
        
        // Duplicate edge should fail gracefully
        assert!(graph.add_edge("I", "V").is_err());
        
        // Missing node should fail gracefully
        assert!(graph.add_edge("I", "vi").is_err());
        
        // Graph should still be in valid state
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.has_edge("I", "V"));
    }
}

#[cfg(test)]
mod performance_characteristics {
    use super::*;

    #[test]
    fn test_large_graph_performance() {
        let mut graph = ProgressionGraph::new();
        
        // Create a moderately sized graph
        let node_count = 50;
        for _i in 0..node_count {
            let node = ProgressionNode {
                id: "test",
                node_type: NodeType::Primary,
                roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
                intervals: &UNISON_ONLY,
            };
            graph.add_node(node);
        }
        
        assert_eq!(graph.node_count(), node_count);
        
        // Adding many edges should still be efficient
        for i in 0..node_count - 1 {
            let from = format!("test_{}", i);
            let to = format!("test_{}", i + 1);
            // These will fail since we used static strings, but test the lookup performance
            let _ = graph.add_edge(&from, &to);
        }
    }

    #[test] 
    fn test_lookup_performance() {
        let mut graph = ProgressionGraph::new();
        
        // Add several nodes
        let nodes = vec![
            create_i_major_node(),
            create_ii_minor_node(), 
            create_v_dominant_node(),
            create_v7_dominant_node(),
            create_vi_minor_node(),
        ];
        
        for node in nodes {
            graph.add_node(node);
        }
        
        // Repeated lookups should be efficient
        for _ in 0..100 {
            assert!(graph.get_node("I").is_some());
            assert!(graph.get_node("V7").is_some());
            assert!(graph.get_node("NONEXISTENT").is_none());
        }
    }
}