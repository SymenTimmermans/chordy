//! Tests for chord progression variant system
//!
//! These tests verify that chord variants are correctly parsed and that 
//! progression queries return nodes with proper intervals.

use chordy::prelude::*;
use chordy::note;

/// Test that chord variants are correctly generated in the major progression system
#[test]
fn test_major_chord_variants() {
    let key = Key::Major(note!("C"));
    
    // Test basic triads
    let i_node = key.progression_node("I").unwrap();
    assert_eq!(i_node.display_name, "I");
    assert_eq!(i_node.intervals.len(), 3); // Root, third, fifth
    assert!(i_node.intervals.contains(&Interval::PERFECT_UNISON));
    assert!(i_node.intervals.contains(&Interval::MAJOR_THIRD));
    assert!(i_node.intervals.contains(&Interval::PERFECT_FIFTH));
    
    // Test I6 variant
    let i6_node = key.progression_node("I6").unwrap();
    assert_eq!(i6_node.display_name, "I6");
    assert!(i6_node.intervals.contains(&Interval::MAJOR_SIXTH));
    
    // Test I7 variant (dominant 7 - unusual but possible)
    let i7_node = key.progression_node("I7").unwrap();
    assert_eq!(i7_node.display_name, "I7");
    // I7 would be a dominant seventh on the tonic, so minor seventh
    assert!(i7_node.intervals.contains(&Interval::MINOR_SEVENTH));
    
    // Test Imaj7 variant
    let imaj7_node = key.progression_node("Imaj7").unwrap();
    assert_eq!(imaj7_node.display_name, "Imaj7");
    assert!(imaj7_node.intervals.contains(&Interval::MAJOR_SEVENTH));
    
    // Test ii7 variant (minor chord with minor seventh)
    let ii7_node = key.progression_node("ii7").unwrap();
    assert_eq!(ii7_node.display_name, "ii7");
    assert!(ii7_node.intervals.contains(&Interval::MINOR_THIRD));
    assert!(ii7_node.intervals.contains(&Interval::MINOR_SEVENTH));
    
    // Test V7 variant (dominant seventh)
    let v7_node = key.progression_node("V7").unwrap();
    assert_eq!(v7_node.display_name, "V7");
    assert!(v7_node.intervals.contains(&Interval::MAJOR_THIRD));
    assert!(v7_node.intervals.contains(&Interval::MINOR_SEVENTH));
    
    // Test V9 variant (dominant ninth)
    let v9_node = key.progression_node("V9").unwrap();
    assert_eq!(v9_node.display_name, "V9");
    assert!(v9_node.intervals.contains(&Interval::MINOR_SEVENTH));
    assert!(v9_node.intervals.contains(&Interval::MAJOR_NINTH));
}

/// Test that chord variants are correctly generated in the minor progression system
#[test]
fn test_minor_chord_variants() {
    let key = Key::Minor(note!("A"));
    
    // Test basic minor triad
    let i_node = key.progression_node("i").unwrap();
    assert_eq!(i_node.display_name, "i");
    assert!(i_node.intervals.contains(&Interval::MINOR_THIRD));
    
    // Test im7 variant
    let im7_node = key.progression_node("im7").unwrap();
    assert_eq!(im7_node.display_name, "im7");
    assert!(im7_node.intervals.contains(&Interval::MINOR_THIRD));
    assert!(im7_node.intervals.contains(&Interval::MINOR_SEVENTH));
    
    // Test V7 in minor (still dominant)
    let v7_node = key.progression_node("V7").unwrap();
    assert_eq!(v7_node.display_name, "V7");
    assert!(v7_node.intervals.contains(&Interval::MAJOR_THIRD));
    assert!(v7_node.intervals.contains(&Interval::MINOR_SEVENTH));
    
    // Test half-diminished ii chord
    let ii_b5_node = key.progression_node("iib5").unwrap();
    assert_eq!(ii_b5_node.display_name, "iib5");
    assert!(ii_b5_node.intervals.contains(&Interval::MINOR_THIRD));
    assert!(ii_b5_node.intervals.contains(&Interval::DIMINISHED_FIFTH));
}

/// Test that RomanChord conversion works properly for variants
#[test]
fn test_variant_to_roman_chord() {
    let key = Key::Major(note!("C"));
    
    let v7_node = key.progression_node("V7").unwrap();
    let roman_chord = v7_node.to_roman_chord();
    
    // Should create a RomanChord with V as the root and proper intervals
    assert_eq!(roman_chord.root().degree(), RomanDegree::V);
    assert_eq!(roman_chord.intervals().len(), 4); // Root, third, fifth, seventh
    assert!(roman_chord.intervals().contains(&Interval::PERFECT_UNISON));
    assert!(roman_chord.intervals().contains(&Interval::MAJOR_THIRD));
    assert!(roman_chord.intervals().contains(&Interval::PERFECT_FIFTH));
    assert!(roman_chord.intervals().contains(&Interval::MINOR_SEVENTH));
}

/// Test progression options include all variant nodes
#[test]
fn test_progression_options_include_variants() {
    let key = Key::Major(note!("C"));
    
    let i_node = key.progression_node("I").unwrap();
    let options = key.progression_options(i_node).unwrap();
    
    // Should have strong connections to IV variants and V variants
    let strong_display_names: Vec<&str> = options.strong.iter()
        .map(|node| node.display_name)
        .collect();
    
    // Should include various IV variants
    assert!(strong_display_names.contains(&"IV6"));
    assert!(strong_display_names.contains(&"IV7"));
    assert!(strong_display_names.contains(&"IVmaj7"));
    
    // Should include various V variants  
    assert!(strong_display_names.contains(&"V7"));
    assert!(strong_display_names.contains(&"V9"));
    assert!(strong_display_names.contains(&"V11"));
}

/// Test altered dominant chord variants (7+b9, 7+#9)
#[test]
fn test_altered_dominant_variants() {
    let key = Key::Major(note!("C"));
    
    // Test V7+b9 (dominant with flat nine)
    let v7b9_node = key.progression_node("V7+b9").unwrap();
    assert_eq!(v7b9_node.display_name, "V7+b9");
    assert!(v7b9_node.intervals.contains(&Interval::MINOR_SEVENTH));
    assert!(v7b9_node.intervals.contains(&Interval::MINOR_NINTH));
    
    // Test V7+#9 (dominant with sharp nine)
    let v7sharp9_node = key.progression_node("V7+#9").unwrap();
    assert_eq!(v7sharp9_node.display_name, "V7+#9");
    assert!(v7sharp9_node.intervals.contains(&Interval::MINOR_SEVENTH));
    assert!(v7sharp9_node.intervals.contains(&Interval::AUGMENTED_NINTH));
}

/// Test that chord quality is correctly determined from roman numeral case
#[test]
fn test_roman_numeral_quality_from_case() {
    let key = Key::Major(note!("C"));
    
    // Uppercase = major
    let i_node = key.progression_node("I").unwrap();
    assert!(i_node.intervals.contains(&Interval::MAJOR_THIRD));
    
    // Lowercase = minor (use variants that exist in progression data)
    let ii7_node = key.progression_node("ii7").unwrap();
    assert!(ii7_node.intervals.contains(&Interval::MINOR_THIRD));
    
    let vi7_node = key.progression_node("vi7").unwrap();
    assert!(vi7_node.intervals.contains(&Interval::MINOR_THIRD));
}

/// Test that unknown variants return None
#[test]
fn test_unknown_variant_returns_none() {
    let key = Key::Major(note!("C"));
    
    // Test invalid chord name
    assert!(key.progression_node("Xsus4add9").is_none());
    
    // Test valid roman numeral with invalid variant
    assert!(key.progression_node("Iunknown").is_none());
}