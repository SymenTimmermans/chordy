//! Tests for the new progression architecture with RomanChord
//!
//! These tests verify that the progression system works correctly with 
//! the new RomanChord-based architecture.

use chordy::types::progression::ProgressionGraph;
use chordy::types::{Key, Chord, NoteName, Letter, Accidental};

#[test]
fn test_major_graph_basic_functionality() {
    let graph = ProgressionGraph::major();
    
    // Graph should have nodes
    assert!(graph.node_count() > 0);
    assert!(!graph.is_empty());
    
    // Should be able to find common nodes
    // Note: ID generation uses debug format, so we need to use the correct format
    // This is not the primary API, but let's test it exists
    assert!(graph.node_count() > 10); // Should have many chord variants
}

#[test]
fn test_minor_graph_basic_functionality() {
    let graph = ProgressionGraph::minor();
    
    // Graph should have nodes
    assert!(graph.node_count() > 0);
    assert!(!graph.is_empty());
    assert!(graph.node_count() > 10); // Should have many chord variants
}

#[test]
fn test_progressions_from_basic() {
    // Get progression options using the simplified API
    // We can't easily use string IDs due to the debug format, so let's test via Key integration
    let c_major = Key::Major(NoteName::new(Letter::C, Accidental::Natural));
    let c_chord = Chord::major(NoteName::new(Letter::C, Accidental::Natural));
    
    let options = c_major.progressions_from(&c_chord);
    assert!(options.is_some());
    
    let options = options.unwrap();
    // I chord should have some progression options
    assert!(!options.is_empty());
}

#[test]
fn test_key_integration_with_new_architecture() {
    let c_major = Key::Major(NoteName::new(Letter::C, Accidental::Natural));
    let c_chord = Chord::major(NoteName::new(Letter::C, Accidental::Natural));
    
    // Key should be able to analyze chord progression options
    let progressions_from = c_major.progressions_from(&c_chord);
    assert!(progressions_from.is_some());
    
    let options = progressions_from.unwrap();
    // Should have some progression options
    assert!(!options.is_empty());
}

#[test]
fn test_roman_chord_based_node_iteration() {
    let graph = ProgressionGraph::major();
    
    // Should be able to iterate over nodes
    let nodes: Vec<_> = graph.nodes().collect();
    assert!(!nodes.is_empty());
    
    // Each node should be a RomanChord with a root and intervals
    for node in nodes {
        assert!(!node.intervals.is_empty());
        // Root should be a valid RomanNumeral (this tests the new structure)
    }
}

#[test]
fn test_chord_copy_semantics_in_progressions() {
    let c_major = Key::Major(NoteName::new(Letter::C, Accidental::Natural));
    let c_chord = Chord::major(NoteName::new(Letter::C, Accidental::Natural));
    
    // Test that copy semantics work in progression context
    let chord_copy1 = c_chord;
    let chord_copy2 = c_chord; // Should work since Chord implements Copy
    
    // Both copies should give same progression options
    let options1 = c_major.progressions_from(&chord_copy1);
    let options2 = c_major.progressions_from(&chord_copy2);
    
    assert_eq!(options1.is_some(), options2.is_some());
    if let (Some(opt1), Some(opt2)) = (options1, options2) {
        assert_eq!(opt1.len(), opt2.len());
    }
}
