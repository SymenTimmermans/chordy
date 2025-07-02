//! Basic tests for the chord progression system

use chordy::prelude::*;
use chordy::RomanDegree;

#[test]
fn test_major_key_progression_options() {
    let c_major = Key::Major(note!("C"));
    
    // Test I chord progressions
    let options = c_major.progression_options("I").unwrap();
    
    // I should have strong progressions to IV, V, and vi
    assert!(!options.strong.is_empty(), "I should have strong progression options");
    assert!(!options.moderate.is_empty(), "I should have moderate progression options");
    
    // Find specific progressions
    let has_iv = options.strong.iter().any(|node| node.display_name == "IV");
    let has_v = options.strong.iter().any(|node| node.display_name == "V");
    let has_vi = options.strong.iter().any(|node| node.display_name == "vi");
    
    assert!(has_iv, "I should have strong progression to IV");
    assert!(has_v, "I should have strong progression to V");
    assert!(has_vi, "I should have strong progression to vi");
}

#[test]
fn test_minor_key_progression_options() {
    let a_minor = Key::Minor(note!("A"));
    
    // Test i chord progressions in minor
    let options = a_minor.progression_options("i").unwrap();
    
    assert!(!options.strong.is_empty(), "i should have strong progression options");
    
    // Find specific progressions common in minor
    let has_iv = options.strong.iter().any(|node| node.display_name == "iv");
    let has_v = options.strong.iter().any(|node| node.display_name == "V");
    
    assert!(has_iv, "i should have strong progression to iv");
    assert!(has_v, "i should have strong progression to V");
}

#[test]
fn test_progression_node_lookup() {
    let c_major = Key::Major(note!("C"));
    
    // Test basic node lookup
    let tonic = c_major.progression_node("I").unwrap();
    assert_eq!(tonic.display_name, "I");
    assert_eq!(tonic.roman_numeral.degree(), RomanDegree::I);
    
    let subdominant = c_major.progression_node("IV").unwrap();
    assert_eq!(subdominant.display_name, "IV");
    assert_eq!(subdominant.roman_numeral.degree(), RomanDegree::IV);
    
    // Test minor chord lookup
    let submediant = c_major.progression_node("vi").unwrap();
    assert_eq!(submediant.display_name, "vi");
    assert_eq!(submediant.roman_numeral.degree(), RomanDegree::VI);
}

#[test]
fn test_progression_strength_categories() {
    let c_major = Key::Major(note!("C"));
    let options = c_major.progression_options("V").unwrap();
    
    // V should have strong progressions (explicit arrows)
    assert!(!options.strong.is_empty(), "V should have strong progressions");
    
    // Should have at least I as a strong target
    let has_i = options.strong.iter().any(|node| node.display_name == "I");
    assert!(has_i, "V should have strong progression to I");
    
    // Should also have moderate and weak options
    assert!(!options.moderate.is_empty(), "V should have moderate progression options");
    assert!(!options.weak.is_empty(), "V should have weak progression options");
}

#[test]
fn test_all_progression_nodes() {
    let c_major = Key::Major(note!("C"));
    let all_nodes = c_major.all_progression_nodes();
    
    assert!(!all_nodes.is_empty(), "Should have progression nodes");
    
    // Should have basic diatonic functions
    let node_names: Vec<&str> = all_nodes.iter().map(|n| n.display_name).collect();
    assert!(node_names.contains(&"I"), "Should have I");
    assert!(node_names.contains(&"IV"), "Should have IV");
    assert!(node_names.contains(&"V"), "Should have V");
    assert!(node_names.contains(&"vi"), "Should have vi");
}