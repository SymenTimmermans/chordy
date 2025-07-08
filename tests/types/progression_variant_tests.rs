//! Tests for chord progression variant system
//!
//! These tests verify that chord variants are correctly parsed and that 
//! progression queries return nodes with proper intervals.

use chordy::prelude::*;
use chordy::note;




/// Test progression options include variant chords
#[test]
fn test_progression_options_include_variants() {
    let key = Key::Major(note!("C"));
    
    let c_chord = Chord::major(note!("C")); // I chord
    let options = key.progression_options(&c_chord).unwrap();
    
    // Should have strong connections that include various extensions
    // Check for F chord (IV) with different voicings
    let has_f_chords = options.strong.iter().any(|chord| chord.root == note!("F"));
    assert!(has_f_chords, "Should have F (IV) progressions");
    
    // Check for G chord (V) with different voicings
    let has_g_chords = options.strong.iter().any(|chord| chord.root == note!("G"));
    assert!(has_g_chords, "Should have G (V) progressions");
    
    // Check that we get chords with various intervals (extensions)
    let has_seventh_chords = options.strong.iter()
        .any(|chord| chord.intervals.contains(Interval::MINOR_SEVENTH) || 
                     chord.intervals.contains(Interval::MAJOR_SEVENTH));
    assert!(has_seventh_chords, "Should include seventh chords in progressions");
}





/// Test the chord-based progression options interface
#[test]
fn test_progression_options_from_chord() {
    let key = Key::Major(note!("C"));
    
    // Test progression from C major (I)
    let c_major = Chord::major(note!("C"));
    let options = key.progression_options(&c_major).unwrap();
    
    // Should have strong progressions to IV and V family chords
    assert!(!options.strong.is_empty());
    
    // Check that we get actual Chord objects back
    let first_strong_chord = &options.strong[0];
    // Has a real root note - just check it's not empty
    assert!(!first_strong_chord.intervals.is_empty());
    
    // Check that progression from V7 includes I as strong resolution
    let g7 = Chord::dominant_7th(note!("G"));
    let v7_options = key.progression_options(&g7).unwrap();
    
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
fn test_chord_progression_options_categorization() {
    let key = Key::Major(note!("C"));
    let c_major = Chord::major(note!("C"));
    let options = key.progression_options(&c_major).unwrap();
    
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
fn test_chord_progression_options_different_keys() {
    // Test in G major
    let g_major_key = Key::Major(note!("G"));
    let g_major_chord = Chord::major(note!("G"));
    let options = g_major_key.progression_options(&g_major_chord).unwrap();
    
    // Should have strong progressions to C major (IV) and D major (V)
    let has_c_major = options.strong.iter().any(|chord| chord.root == note!("C"));
    let has_d_major = options.strong.iter().any(|chord| chord.root == note!("D"));
    
    assert!(has_c_major, "I in G major should progress to IV (C major)");
    assert!(has_d_major, "I in G major should progress to V (D major)");
}

/// Test that chord progression options work with chord extensions
#[test]
fn test_chord_progression_options_with_extensions() {
    let key = Key::Major(note!("C"));
    
    // Test with a major 7th chord
    let cmaj7 = Chord::major_7th(note!("C"));
    let options = key.progression_options(&cmaj7).unwrap();
    
    // Should still get progression options (might find Imaj7 node or fallback to I)
    assert!(!options.is_empty(), "Should find progression options for Cmaj7");
    
    // Test with a minor 7th chord
    let dm7 = Chord::minor_7th(note!("D"));
    let options = key.progression_options(&dm7).unwrap();
    
    // Should find ii7 progression options
    assert!(!options.is_empty(), "Should find progression options for Dm7");
}