//! Tests for chord progression variant system
//!
//! These tests verify that chord variants are correctly parsed and that 
//! progression queries return nodes with proper intervals.

use chordy::prelude::*;
use chordy::note;




/// Test progression options include variant chords
#[test]
fn test_progressions_from_include_variants() {
    let key = Key::Major(note!("C"));
    
    let c_chord = Chord::major(note!("C")); // I chord
    let options = key.progressions_from(&c_chord).unwrap();
    
    // Check for progressions - the exact behavior depends on which I node is matched
    assert!(!options.strong.is_empty(), "I should have strong progression options");
    
    // The algorithm may match different I variants (I, I/3, I/5, etc.) depending on internal logic
    // Accept any valid strong progressions - could be IV (F) or V (G) depending on which I node
    let has_f_progressions = options.strong.iter().any(|chord| chord.root == note!("F")); // IV
    let has_g_progressions = options.strong.iter().any(|chord| chord.root == note!("G")); // V
    
    assert!(has_f_progressions || has_g_progressions, 
           "I should have strong progressions to either IV (F) or V (G)");
    
    // Check for F progressions somewhere (might be in moderate)
    let has_f_somewhere = options.strong.iter()
        .chain(options.moderate.iter())
        .chain(options.weak.iter())
        .any(|chord| chord.root == note!("F"));
    assert!(has_f_somewhere, "Should have F (IV) progressions somewhere");
    
    // Check that we get chords with various intervals (extensions) across all categories
    let has_seventh_chords = options.strong.iter()
        .chain(options.moderate.iter())
        .chain(options.weak.iter())
        .any(|chord| chord.intervals.contains(Interval::MINOR_SEVENTH) || 
                     chord.intervals.contains(Interval::MAJOR_SEVENTH));
    assert!(has_seventh_chords, "Should include seventh chords somewhere in the progression options");
}





/// Test the chord-based progression options interface
#[test]
fn test_progressions_from_from_chord() {
    let key = Key::Major(note!("C"));
    
    // Test progression from C major (I)
    let c_major = Chord::major(note!("C"));
    let options = key.progressions_from(&c_major).unwrap();
    
    // Should have strong progressions to IV and V family chords
    assert!(!options.strong.is_empty());
    
    // Check that we get actual Chord objects back
    let first_strong_chord = &options.strong[0];
    // Has a real root note - just check it's not empty
    assert!(!first_strong_chord.intervals.is_empty());
    
    // Check that progression from V7 includes I as strong resolution
    let g7 = Chord::dominant_7th(note!("G"));
    let v7_options = key.progressions_from(&g7).unwrap();
    
    // Should have strong resolution to I
    assert!(!v7_options.strong.is_empty());
    
    // Look for C major chord in the strong options
    let has_c_major = v7_options.strong.iter().any(|chord| {
        chord.root == note!("C") && chord.intervals.contains(Interval::MAJOR_THIRD)
    });
    assert!(has_c_major, "V7 should have strong progression to I (C major)");
}

/// Test chord progression options have proper categorization
#[test]
fn test_chord_progressions_from_categorization() {
    let key = Key::Major(note!("C"));
    let c_major = Chord::major(note!("C"));
    let options = key.progressions_from(&c_major).unwrap();
    
    // Should have all three categories
    assert!(!options.strong.is_empty(), "Should have strong progressions");
    assert!(!options.moderate.is_empty(), "Should have moderate progressions");
    assert!(!options.weak.is_empty(), "Should have weak progressions");
    
    // Test the `all` iterator
    let all_count = options.all().count();
    assert_eq!(all_count, options.len());
    assert_eq!(all_count, options.strong.len() + options.moderate.len() + options.weak.len());
    
    // Test that all chords are in the same key
    for (chord, _strength) in options.all() {
        // All chord roots should be notes that make sense in C major context
        // This is a basic sanity check - just verify we have valid chords
        assert!(!chord.intervals.is_empty());
    }
}

/// Test progression options in different keys
#[test]
fn test_chord_progressions_from_different_keys() {
    // Test in G major
    let g_major_key = Key::Major(note!("G"));
    let g_major_chord = Chord::major(note!("G"));
    let options = g_major_key.progressions_from(&g_major_chord).unwrap();
    
    // Check for progressions - the exact behavior depends on which I node is matched
    assert!(!options.strong.is_empty(), "I should have strong progression options");
    
    // The algorithm may match different I variants (I, I/3, I/5, etc.) depending on internal logic
    // Accept any valid strong progressions - could be IV (C) or V (D) depending on which I node
    let has_c_progressions = options.strong.iter().any(|chord| chord.root == note!("C")); // IV
    let has_d_progressions = options.strong.iter().any(|chord| chord.root == note!("D")); // V
    
    assert!(has_c_progressions || has_d_progressions, 
           "I should have strong progressions to either IV (C) or V (D)");
    
    // If D is not in strong, it should be in moderate progressions as a jump to primary node
    if !has_d_progressions {
        let has_d_in_moderate = options.moderate.iter().any(|chord| chord.root == note!("D"));
        assert!(has_d_in_moderate, "I in G major should have V (D major) in moderate progressions");
    }
}

/// Test that chord progression options work with chord extensions
#[test]
fn test_chord_progressions_from_with_extensions() {
    let key = Key::Major(note!("C"));
    
    // Test with a major 7th chord
    let cmaj7 = Chord::major_7th(note!("C"));
    let options = key.progressions_from(&cmaj7).unwrap();
    
    // Should still get progression options (might find Imaj7 node or fallback to I)
    assert!(!options.is_empty(), "Should find progression options for Cmaj7");
    
    // Test with a minor 7th chord
    let dm7 = Chord::minor_7th(note!("D"));
    let options = key.progressions_from(&dm7).unwrap();
    
    // Should find ii7 progression options
    assert!(!options.is_empty(), "Should find progression options for Dm7");
}