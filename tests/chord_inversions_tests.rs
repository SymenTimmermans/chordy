//! Tests for chord inversions and slash chords
//!
//! These tests verify that both Chord and RomanChord types properly handle
//! inversions and slash chords, including correct bass note calculation,
//! display formatting, and type distinctions.

use chordy::prelude::*;
use chordy::{note, roman, BassType, Chord, Key, RomanChord};

#[test]
fn test_chord_basic_inversions() {
    let c_major = Chord::major(note!("C"));

    // Test root position
    assert_eq!(c_major.bass_note(), note!("C"));
    assert!(!c_major.is_inverted());
    assert!(!c_major.is_slash_chord());
    assert_eq!(c_major.inversion_number(), None);

    // Test first inversion
    let first_inversion = c_major.with_inversion(1);
    assert_eq!(first_inversion.bass_note(), note!("E"));
    assert!(first_inversion.is_inverted());
    assert!(!first_inversion.is_slash_chord());
    assert_eq!(first_inversion.inversion_number(), Some(1));

    // Test second inversion
    let second_inversion = c_major.with_inversion(2);
    assert_eq!(second_inversion.bass_note(), note!("G"));
    assert!(second_inversion.is_inverted());
    assert!(!second_inversion.is_slash_chord());
    assert_eq!(second_inversion.inversion_number(), Some(2));
}

#[test]
fn test_chord_slash_chords() {
    let c_major = Chord::major(note!("C"));

    // Test slash chord with chord tone bass
    let c_slash_e = c_major.with_slash_bass(note!("E"));
    assert_eq!(c_slash_e.bass_note(), note!("E"));
    assert!(!c_slash_e.is_inverted());
    assert!(c_slash_e.is_slash_chord());
    assert_eq!(c_slash_e.inversion_number(), None);

    // Test slash chord with non-chord tone bass
    let c_slash_f = c_major.with_slash_bass(note!("F"));
    assert_eq!(c_slash_f.bass_note(), note!("F"));
    assert!(!c_slash_f.is_inverted());
    assert!(c_slash_f.is_slash_chord());
    assert_eq!(c_slash_f.inversion_number(), None);

    // Test slash chord with chromatic bass
    let c_slash_fs = c_major.with_slash_bass(note!("F#"));
    assert_eq!(c_slash_fs.bass_note(), note!("F#"));
    assert!(c_slash_fs.is_slash_chord());
}

#[test]
fn test_chord_convenience_methods() {
    let c_major = Chord::major(note!("C"));

    let first_inv = c_major.in_first_inversion();
    assert_eq!(first_inv.bass_note(), note!("E"));
    assert_eq!(first_inv.inversion_number(), Some(1));

    let second_inv = c_major.in_second_inversion();
    assert_eq!(second_inv.bass_note(), note!("G"));
    assert_eq!(second_inv.inversion_number(), Some(2));
}

#[test]
fn test_chord_inversion_zero_returns_root_position() {
    let c_major = Chord::major(note!("C"));
    let first_inversion = c_major.with_inversion(1);

    // Inversion 0 should return to root position
    let back_to_root = first_inversion.with_inversion(0);
    assert_eq!(back_to_root.bass_note(), note!("C"));
    assert!(!back_to_root.is_inverted());
    assert_eq!(back_to_root.inversion_number(), None);
}

#[test]
fn test_chord_seventh_chord_inversions() {
    let c_dom7 = Chord::dominant_7th(note!("C"));

    // Test all inversions of a seventh chord
    let first_inv = c_dom7.with_inversion(1);
    assert_eq!(first_inv.bass_note(), note!("E"));

    let second_inv = c_dom7.with_inversion(2);
    assert_eq!(second_inv.bass_note(), note!("G"));

    let third_inv = c_dom7.with_inversion(3);
    assert_eq!(third_inv.bass_note(), note!("Bb"));
    assert_eq!(third_inv.inversion_number(), Some(3));
}

#[test]
fn test_roman_chord_basic_inversions() {
    let i_major = RomanChord::major(roman!("I"));

    // Test root position
    assert_eq!(i_major.bass_note(), roman!("I"));
    assert!(!i_major.is_inverted());
    assert!(!i_major.is_slash_chord());
    assert_eq!(i_major.inversion_number(), None);

    // Test first inversion
    let first_inversion = i_major.with_inversion(1);
    assert_eq!(first_inversion.bass_note(), roman!("III"));
    assert!(first_inversion.is_inverted());
    assert!(!first_inversion.is_slash_chord());
    assert_eq!(first_inversion.inversion_number(), Some(1));

    // Test second inversion
    let second_inversion = i_major.with_inversion(2);
    assert_eq!(second_inversion.bass_note(), roman!("V"));
    assert!(second_inversion.is_inverted());
    assert_eq!(second_inversion.inversion_number(), Some(2));
}

#[test]
fn test_roman_chord_slash_chords() {
    let i_major = RomanChord::major(roman!("I"));

    // Test slash chord with chord tone bass
    let i_slash_iii = i_major.with_slash_bass(roman!("III"));
    assert_eq!(i_slash_iii.bass_note(), roman!("III"));
    assert!(!i_slash_iii.is_inverted());
    assert!(i_slash_iii.is_slash_chord());

    // Test slash chord with non-chord tone bass
    let i_slash_bvii = i_major.with_slash_bass(roman!("bVII"));
    assert_eq!(i_slash_bvii.bass_note(), roman!("bVII"));
    assert!(i_slash_bvii.is_slash_chord());
}

#[test]
fn test_roman_chord_convenience_methods() {
    let i_major = RomanChord::major(roman!("I"));

    let first_inv = i_major.in_first_inversion();
    assert_eq!(first_inv.bass_note(), roman!("III"));
    assert_eq!(first_inv.inversion_number(), Some(1));

    let second_inv = i_major.in_second_inversion();
    assert_eq!(second_inv.bass_note(), roman!("V"));
    assert_eq!(second_inv.inversion_number(), Some(2));
}

#[test]
fn test_invertible_trait_integration() {
    use chordy::traits::Invertible;

    let c_major = Chord::major(note!("C"));

    // Test Invertible trait
    let first_inv = c_major.inverted(1);
    assert_eq!(first_inv.bass_note(), note!("E"));
    assert!(first_inv.is_inverted());

    let second_inv = c_major.inverted(2);
    assert_eq!(second_inv.bass_note(), note!("G"));
    assert_eq!(second_inv.inversion_number(), Some(2));

    // Test with RomanChord
    let i_major = RomanChord::major(roman!("I"));
    let i_first_inv = i_major.inverted(1);
    assert_eq!(i_first_inv.bass_note(), roman!("III"));
    assert!(i_first_inv.is_inverted());
}

#[test]
fn test_chord_to_roman_with_bass() {
    let c_major_key = Key::Major(note!("C"));

    // Test inverted chord conversion to roman
    let c_first_inv = Chord::major(note!("C")).with_inversion(1);
    let roman_result = c_first_inv.to_roman(&c_major_key).unwrap();

    assert_eq!(roman_result.root(), roman!("I"));
    // Bass should be preserved in the conversion
    if let Some((bass, bass_type)) = roman_result.bass {
        assert_eq!(bass, roman!("III"));
        assert_eq!(bass_type, BassType::Inversion(1));
    } else {
        panic!("Bass information should be preserved");
    }
}

#[test]
fn test_roman_chord_to_concrete_with_bass() {
    let c_major_key = Key::Major(note!("C"));

    // Test RomanChord with inversion to concrete Chord
    let i_first_inv = RomanChord::major(roman!("I")).with_inversion(1);
    let concrete_chord = i_first_inv.in_key(&c_major_key);

    assert_eq!(concrete_chord.root, note!("C"));
    assert_eq!(concrete_chord.bass_note(), note!("E"));
    assert!(concrete_chord.is_inverted());
    assert_eq!(concrete_chord.inversion_number(), Some(1));

    // Test with slash chord
    let i_slash_bvii = RomanChord::major(roman!("I")).with_slash_bass(roman!("bVII"));
    let concrete_slash = i_slash_bvii.in_key(&c_major_key);

    assert_eq!(concrete_slash.root, note!("C"));
    assert_eq!(concrete_slash.bass_note(), note!("Bb"));
    assert!(concrete_slash.is_slash_chord());
}

#[test]
fn test_display_formatting() {
    let c_major = Chord::major(note!("C"));

    // Test inversion display
    let first_inv = c_major.with_inversion(1);
    assert_eq!(format!("{}", first_inv), "C/E");

    let second_inv = c_major.with_inversion(2);
    assert_eq!(format!("{}", second_inv), "C/G");

    // Test slash chord display
    let slash_f = c_major.with_slash_bass(note!("F"));
    assert_eq!(format!("{}", slash_f), "C/F");

    let slash_fs = c_major.with_slash_bass(note!("F#"));
    assert_eq!(format!("{}", slash_fs), "C/F♯");

    // Test roman chord display
    let i_major = RomanChord::major(roman!("I"));
    let i_first_inv = i_major.with_inversion(1);
    assert_eq!(format!("{}", i_first_inv), "I/III");

    let i_slash_bvii = i_major.with_slash_bass(roman!("bVII"));
    assert_eq!(format!("{}", i_slash_bvii), "I/♭VII");
}

#[test]
fn test_complex_chord_inversions() {
    // Test inversion of complex chords
    let cmaj9 = Chord::new(
        note!("C"),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SEVENTH,
            Interval::MAJOR_NINTH,
        ],
    );

    let cmaj9_first_inv = cmaj9.with_inversion(1);
    assert_eq!(cmaj9_first_inv.bass_note(), note!("E"));
    assert!(cmaj9_first_inv.is_inverted());

    // Test higher inversions
    let cmaj9_third_inv = cmaj9.with_inversion(3);
    assert_eq!(cmaj9_third_inv.bass_note(), note!("B")); // Major 7th
    assert_eq!(cmaj9_third_inv.inversion_number(), Some(3));

    let cmaj9_fourth_inv = cmaj9.with_inversion(4);
    assert_eq!(cmaj9_fourth_inv.bass_note(), note!("D")); // Major 9th
    assert_eq!(cmaj9_fourth_inv.inversion_number(), Some(4));
}

#[test]
fn test_edge_cases() {
    let c_major = Chord::major(note!("C"));

    // Test inversion beyond available intervals (should be safe)
    let invalid_inv = c_major.with_inversion(10);
    // Should not crash and should still have same root
    assert_eq!(invalid_inv.root, note!("C"));

    // Test empty chord
    let empty_chord = Chord::new(note!("C"), vec![]);
    let empty_inv = empty_chord.with_inversion(1);
    assert_eq!(empty_inv.bass_note(), note!("C")); // Should fall back to root
}

#[test]
fn test_bass_type_matching() {
    let c_major = Chord::major(note!("C"));

    // Test BassType::Inversion
    let inversion = c_major.with_inversion(1);
    if let Some((_, bass_type)) = inversion.bass {
        assert!(matches!(bass_type, BassType::Inversion(1)));
    } else {
        panic!("Inversion should have bass information");
    }

    // Test BassType::Slash
    let slash_chord = c_major.with_slash_bass(note!("F"));
    if let Some((_, bass_type)) = slash_chord.bass {
        assert!(matches!(bass_type, BassType::Slash));
    } else {
        panic!("Slash chord should have bass information");
    }
}

#[test]
fn test_different_keys_and_accidentals() {
    // Test with sharp key
    let fs_major = Chord::major(note!("F#"));
    let fs_first_inv = fs_major.with_inversion(1);
    assert_eq!(fs_first_inv.bass_note(), note!("A#"));

    // Test with flat key
    let db_major = Chord::major(note!("Db"));
    let db_first_inv = db_major.with_inversion(1);
    assert_eq!(db_first_inv.bass_note(), note!("F"));

    // Test slash chord with accidentals
    let c_slash_bb = Chord::major(note!("C")).with_slash_bass(note!("Bb"));
    assert_eq!(c_slash_bb.bass_note(), note!("Bb"));
    assert_eq!(format!("{}", c_slash_bb), "C/B♭");
}

#[test]
fn test_chord_name_integration() {
    let c_major = Chord::major(note!("C"));

    // Test that chord name includes bass
    let first_inv = c_major.with_inversion(1);
    let chord_name = first_inv.to_chord_name();
    assert!(chord_name.bass_note.is_some());

    let slash_chord = c_major.with_slash_bass(note!("F"));
    let slash_name = slash_chord.to_chord_name();
    assert!(slash_name.bass_note.is_some());
}

#[test]
fn test_copy_semantics_with_bass() {
    let c_major = Chord::major(note!("C"));
    let first_inv = c_major.with_inversion(1);

    // Test that Copy works with bass field
    let copied_inv = first_inv;
    let another_copy = first_inv;

    assert_eq!(copied_inv.bass_note(), note!("E"));
    assert_eq!(another_copy.bass_note(), note!("E"));
    assert!(copied_inv.is_inverted());
    assert!(another_copy.is_inverted());
}
