//! Tests for mutable interval operations on HasIntervals types

use chordy::prelude::*;

/// Test IntervalSet remove() method
#[test]
fn test_interval_set_remove() {
    let mut intervals: IntervalSet = IntervalSet::from_slice(&[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
    ]);
    
    assert_eq!(intervals.len(), 4);
    
    // Remove existing interval
    assert!(intervals.remove(Interval::MAJOR_THIRD));
    assert_eq!(intervals.len(), 3);
    assert!(!intervals.contains(Interval::MAJOR_THIRD));
    
    // Try to remove non-existent interval
    assert!(!intervals.remove(Interval::MAJOR_SECOND));
    assert_eq!(intervals.len(), 3);
    
    // Verify remaining intervals are correct
    assert!(intervals.contains(Interval::PERFECT_UNISON));
    assert!(intervals.contains(Interval::PERFECT_FIFTH));
    assert!(intervals.contains(Interval::MINOR_SEVENTH));
}

/// Test Chord set_intervals() implementation
#[test]
fn test_chord_set_intervals() {
    let mut chord = Chord::major(note!("C"));
    assert!(chord.contains_interval(Interval::MAJOR_THIRD));
    assert!(chord.contains_interval(Interval::PERFECT_FIFTH));
    
    // Set new intervals
    chord.set_intervals(vec![
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
    ]);
    
    // Verify changes
    assert!(!chord.contains_interval(Interval::MAJOR_THIRD));
    assert!(chord.contains_interval(Interval::MINOR_THIRD));
    assert!(chord.contains_interval(Interval::MINOR_SEVENTH));
    
    // Root should be preserved
    assert_eq!(chord.root(), note!("C"));
}

/// Test Chord remove_interval() implementation
#[test]
fn test_chord_remove_interval() {
    let mut chord = Chord::major_7th(note!("D"));
    assert!(chord.contains_interval(Interval::MAJOR_SEVENTH));
    
    // Remove the seventh
    chord.remove_interval(Interval::MAJOR_SEVENTH);
    assert!(!chord.contains_interval(Interval::MAJOR_SEVENTH));
    
    // Should now be a major triad
    assert!(chord.contains_interval(Interval::PERFECT_UNISON));
    assert!(chord.contains_interval(Interval::MAJOR_THIRD));
    assert!(chord.contains_interval(Interval::PERFECT_FIFTH));
    assert_eq!(chord.intervals().len(), 3);
}

/// Test Chord add_interval() implementation
#[test]
fn test_chord_add_interval() {
    let mut chord = Chord::major(note!("F"));
    
    // Add a seventh
    chord.add_interval(Interval::MAJOR_SEVENTH);
    assert!(chord.contains_interval(Interval::MAJOR_SEVENTH));
    
    // Try to add an existing interval - should be no-op
    let len_before = chord.intervals().len();
    chord.add_interval(Interval::MAJOR_THIRD);
    assert_eq!(chord.intervals().len(), len_before);
}

/// Test that bass note is preserved during interval mutations
#[test]
fn test_chord_preserves_bass() {
    let mut chord = Chord::major(note!("G")).with_slash_bass(note!("B"));
    let original_bass = chord.bass_note();
    
    // Modify intervals
    chord.add_interval(Interval::MINOR_SEVENTH);
    chord.remove_interval(Interval::PERFECT_FIFTH);
    
    // Bass should still be the same
    assert_eq!(chord.bass_note(), original_bass);
    assert_eq!(chord.bass_note(), note!("B"));
}

/// Test Scale set_intervals() implementation
#[test]
fn test_scale_set_intervals() {
    let mut scale = Scale::major(note!("C"));
    
    // Change to harmonic minor intervals
    scale.set_intervals(vec![
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ]);
    
    // Verify it's now harmonic minor
    assert!(scale.contains_interval(Interval::MINOR_THIRD));
    assert!(scale.contains_interval(Interval::MINOR_SIXTH));
    assert!(scale.contains_interval(Interval::MAJOR_SEVENTH));
    
    // Root should be preserved
    assert_eq!(scale.root(), note!("C"));
}

/// Test Scale remove_interval() implementation
#[test]
fn test_scale_remove_interval() {
    let mut scale = Scale::major(note!("D"));
    assert_eq!(scale.intervals().len(), 7);
    
    // Remove the seventh degree
    scale.remove_interval(Interval::MAJOR_SEVENTH);
    assert_eq!(scale.intervals().len(), 6);
    assert!(!scale.contains_interval(Interval::MAJOR_SEVENTH));
}

/// Test Scale add_interval() implementation
#[test]
fn test_scale_add_interval() {
    let mut scale = Scale::major(note!("E"));
    
    // Add a chromatic passing tone
    scale.add_interval(Interval::MINOR_THIRD);
    assert!(scale.contains_interval(Interval::MINOR_THIRD));
    assert!(scale.contains_interval(Interval::MAJOR_THIRD)); // Original third still there
    
    // Try to add existing interval
    let len_before = scale.intervals().len();
    scale.add_interval(Interval::PERFECT_FIFTH);
    assert_eq!(scale.intervals().len(), len_before);
}

/// Test RomanChord set_intervals() implementation
#[test]
fn test_roman_chord_set_intervals() {
    let mut roman_chord = RomanChord::new(
        RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ],
    );
    
    // Change to seventh chord
    roman_chord.set_intervals(vec![
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SEVENTH,
    ]);
    
    assert!(roman_chord.contains_interval(Interval::MAJOR_SEVENTH));
    assert_eq!(roman_chord.intervals().len(), 4);
    
    // Root should be preserved
    assert_eq!(roman_chord.root(), RomanNumeral::new(RomanDegree::I, Accidental::Natural));
}

/// Test RomanChord remove_interval() implementation
#[test]
fn test_roman_chord_remove_interval() {
    let mut roman_chord = RomanChord::new(
        RomanNumeral::new(RomanDegree::V, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SEVENTH,
        ],
    );
    
    // Remove the seventh to get a triad
    roman_chord.remove_interval(Interval::MINOR_SEVENTH);
    assert!(!roman_chord.contains_interval(Interval::MINOR_SEVENTH));
    assert_eq!(roman_chord.intervals().len(), 3);
}

/// Test RomanChord add_interval() implementation
#[test]
fn test_roman_chord_add_interval() {
    let mut roman_chord = RomanChord::new(
        RomanNumeral::new(RomanDegree::II, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FIFTH,
        ],
    );
    
    // Add a seventh
    roman_chord.add_interval(Interval::MINOR_SEVENTH);
    assert!(roman_chord.contains_interval(Interval::MINOR_SEVENTH));
    
    // Add a ninth
    roman_chord.add_interval(Interval::MAJOR_NINTH);
    assert!(roman_chord.contains_interval(Interval::MAJOR_NINTH));
}

/// Test that RomanChord preserves bass during mutations
#[test]
fn test_roman_chord_preserves_bass() {
    let mut roman_chord = RomanChord::new(
        RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ],
    ).with_slash_bass(RomanNumeral::new(RomanDegree::VI, Accidental::Natural));
    
    let original_bass = roman_chord.bass;
    
    // Modify intervals
    roman_chord.add_interval(Interval::MAJOR_SIXTH);
    roman_chord.remove_interval(Interval::PERFECT_FIFTH);
    
    // Bass should be preserved
    assert_eq!(roman_chord.bass, original_bass);
}

/// Test edge cases
#[test]
fn test_edge_cases() {
    // Empty chord
    let mut chord = Chord::new(note!("C"), vec![]);
    chord.add_interval(Interval::PERFECT_UNISON);
    chord.add_interval(Interval::MAJOR_THIRD);
    assert_eq!(chord.intervals().len(), 2);
    
    // Remove all intervals
    let mut scale = Scale::major(note!("D"));
    let all_intervals: Vec<_> = scale.intervals().to_vec();
    for interval in all_intervals {
        scale.remove_interval(interval);
    }
    assert_eq!(scale.intervals().len(), 0);
    
    // Set empty intervals
    let mut roman_chord = RomanChord::new(
        RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        vec![Interval::PERFECT_UNISON],
    );
    roman_chord.set_intervals(vec![]);
    assert_eq!(roman_chord.intervals().len(), 0);
}