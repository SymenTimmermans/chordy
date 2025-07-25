//! Tests for the new IntervalSet type and updated chord architecture
//!
//! These tests verify that the new stack-allocated IntervalSet works correctly
//! and that Chord, RomanChord, and Scale types function properly with it.

use chordy::types::{
    Accidental, Chord, Interval, IntervalSet, Letter, NoteName, RomanChord, RomanDegree,
    RomanNumeral, Scale,
};

#[test]
fn test_interval_set_creation() {
    let intervals: IntervalSet = IntervalSet::new();
    assert_eq!(intervals.len(), 0);
    assert!(intervals.is_empty());
}

#[test]
fn test_interval_set_push() {
    let mut intervals: IntervalSet = IntervalSet::new();
    intervals.push(Interval::PERFECT_UNISON);
    intervals.push(Interval::MAJOR_THIRD);
    intervals.push(Interval::PERFECT_FIFTH);

    assert_eq!(intervals.len(), 3);
    assert!(intervals.contains(Interval::MAJOR_THIRD));
    assert!(!intervals.contains(Interval::MINOR_THIRD));
}

#[test]
fn test_interval_set_from_slice() {
    let array = [
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
    ];
    let intervals: IntervalSet = IntervalSet::from_slice(&array);

    assert_eq!(intervals.len(), 3);
    assert!(intervals.contains(Interval::PERFECT_FIFTH));
}

#[test]
fn test_chord_with_interval_set() {
    let c_major = Chord::major(NoteName::new(Letter::C, Accidental::Natural));

    // Chord should still work the same way
    assert_eq!(c_major.root, NoteName::new(Letter::C, Accidental::Natural));
    assert_eq!(c_major.intervals.len(), 3);
    assert!(c_major.intervals.contains(Interval::PERFECT_UNISON));
    assert!(c_major.intervals.contains(Interval::MAJOR_THIRD));
    assert!(c_major.intervals.contains(Interval::PERFECT_FIFTH));
}

#[test]
fn test_chord_copy_semantics() {
    let c_major = Chord::major(NoteName::new(Letter::C, Accidental::Natural));
    let c_major_copy = c_major; // Should be a copy, not a move
    let c_major_copy2 = c_major; // Should still work

    assert_eq!(c_major.root, c_major_copy.root);
    assert_eq!(c_major.intervals.len(), c_major_copy2.intervals.len());
}

#[test]
fn test_roman_chord_with_interval_set() {
    let i_major = RomanChord {
        root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        intervals: IntervalSet::from_slice(&[
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ]),
        bass: None,
    };

    assert_eq!(i_major.intervals.len(), 3);
    assert!(i_major.intervals.contains(Interval::MAJOR_THIRD));
}

#[test]
fn test_roman_chord_copy_semantics() {
    let i_major = RomanChord {
        root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        intervals: IntervalSet::from_slice(&[
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ]),
        bass: None,
    };

    let copy1 = i_major;
    let copy2 = i_major; // Should work since RomanChord implements Copy

    assert_eq!(copy1.root, copy2.root);
    assert_eq!(copy1.intervals.len(), copy2.intervals.len());
}

#[test]
fn test_scale_with_interval_set() {
    let c_major_scale = Scale::major(NoteName::new(Letter::C, Accidental::Natural));

    // Scale should work with the larger IntervalSet<12>
    assert_eq!(c_major_scale.intervals.len(), 7); // Major scale has 7 intervals
    assert!(c_major_scale.intervals.contains(Interval::PERFECT_UNISON));
    assert!(c_major_scale.intervals.contains(Interval::MAJOR_SECOND));
}

#[test]
fn test_interval_set_const_from_array() {
    const MAJOR_TRIAD: IntervalSet = IntervalSet::const_from_array(
        [
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
            Interval::PERFECT_UNISON,
        ],
        3,
    );

    assert_eq!(MAJOR_TRIAD.len(), 3);
    assert!(MAJOR_TRIAD.contains(Interval::MAJOR_THIRD));
}
