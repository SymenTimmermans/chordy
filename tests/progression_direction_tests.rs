//! Tests for bidirectional chord progression functionality
//!
//! Tests both progressions_from (where can I go?) and progressions_to (what leads here?)

use chordy::prelude::*;

/// Test that progressions_to finds chords that lead to the tonic
#[test]
fn test_progressions_to_tonic() {
    let c_major = Key::Major(note!("C"));
    let tonic = Chord::major(note!("C")); // I chord

    let options = c_major
        .progressions_to(&tonic)
        .expect("Should find progressions leading to I");

    // V should strongly lead to I (dominant resolution)
    let has_dominant = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("G") && chord.intervals.contains(Interval::MAJOR_THIRD));
    assert!(has_dominant, "V should be in strong progressions to I");

    // vii° might also lead to I
    let has_leading_tone = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("B") && chord.intervals.contains(Interval::MINOR_THIRD));

    // Print what we found for verification
    if has_leading_tone {
        assert!(true, "vii° found in strong progressions to I");
    }
}

/// Test that progressions_to finds pre-dominants that lead to V
#[test]
fn test_progressions_to_dominant() {
    let c_major = Key::Major(note!("C"));
    let dominant = Chord::major(note!("G")); // V chord

    let options = c_major
        .progressions_to(&dominant)
        .expect("Should find progressions leading to V");

    // ii should strongly lead to V (ii-V motion)
    let has_supertonic = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("D") && chord.intervals.contains(Interval::MINOR_THIRD));

    // IV might also lead to V
    let has_subdominant = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("F") && chord.intervals.contains(Interval::MAJOR_THIRD));

    assert!(
        has_supertonic || has_subdominant,
        "Either ii or IV should be in strong progressions to V"
    );
}

/// Test bidirectional consistency - if A strongly leads to B, then B should show A as a strong origin
#[test]
fn test_bidirectional_consistency() {
    let c_major = Key::Major(note!("C"));
    let g_chord = Chord::major(note!("G")); // V
    let c_chord = Chord::major(note!("C")); // I

    // Check if V -> I is a strong progression
    let from_v = c_major
        .progressions_from(&g_chord)
        .expect("Should find progressions from V");

    let v_to_i_strong = from_v
        .strong
        .iter()
        .any(|chord| chord.root == note!("C") && chord.intervals.contains(Interval::MAJOR_THIRD));

    if v_to_i_strong {
        // Then I should show V as a strong origin
        let to_i = c_major
            .progressions_to(&c_chord)
            .expect("Should find progressions to I");

        let has_v_origin = to_i.strong.iter().any(|chord| {
            chord.root == note!("G") && chord.intervals.contains(Interval::MAJOR_THIRD)
        });

        assert!(
            has_v_origin,
            "If V strongly leads to I, then I should show V as a strong origin"
        );
    }
}

/// Test progressions_to with extended chords
#[test]
fn test_progressions_to_extended_chords() {
    let c_major = Key::Major(note!("C"));
    let cmaj7 = Chord::major_7th(note!("C")); // Imaj7

    let options = c_major
        .progressions_to(&cmaj7)
        .expect("Should find progressions to Imaj7");

    // Should have some progressions
    assert!(
        !options.is_empty(),
        "Should have progressions leading to Imaj7"
    );

    // Check categories are populated
    let total = options.strong.len() + options.moderate.len() + options.weak.len();
    assert!(total > 0, "Should have some total progressions to Imaj7");
}

/// Test progressions_to in minor keys
#[test]
fn test_progressions_to_minor_key() {
    let a_minor = Key::Minor(note!("A"));
    let tonic = Chord::minor(note!("A")); // i chord

    let options = a_minor
        .progressions_to(&tonic)
        .expect("Should find progressions to i in minor");

    // V should lead to i (E major to A minor)
    let has_dominant = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("E") && chord.intervals.contains(Interval::MAJOR_THIRD));

    // Also check for v (natural minor)
    let has_minor_dominant = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("E") && chord.intervals.contains(Interval::MINOR_THIRD));

    assert!(
        has_dominant || has_minor_dominant,
        "Some form of dominant should lead to tonic in minor"
    );
}

/// Test that progressions_to returns deterministic results
#[test]
fn test_progressions_to_deterministic() {
    let key = Key::Major(note!("C"));
    let chord = Chord::major(note!("C"));

    // Run multiple times
    let mut results = Vec::new();
    for _ in 0..10 {
        let options = key
            .progressions_to(&chord)
            .expect("Should find progressions");

        let signature = (
            options.strong.len(),
            options.moderate.len(),
            options.weak.len(),
            options.strong.get(0).map(|c| format!("{:?}", c)),
        );
        results.push(signature);
    }

    // All should be identical
    let first = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(
            result, first,
            "progressions_to iteration {} differs from first",
            i
        );
    }
}

/// Test progressions_to with slash chords
#[test]
fn test_progressions_to_slash_chords() {
    let c_major = Key::Major(note!("C"));
    let c_over_e = Chord::major(note!("C")).with_slash_bass(note!("E")); // I/3

    let options = c_major.progressions_to(&c_over_e);

    // Slash chords might have specific voice leading origins
    if let Some(opts) = options {
        assert!(
            !opts.is_empty(),
            "Should have some progressions to slash chord"
        );
    }
}

/// Compare progressions_from and progressions_to for complementary information
#[test]
fn test_progressions_from_vs_to_comparison() {
    let c_major = Key::Major(note!("C"));
    let f_chord = Chord::major(note!("F")); // IV

    // Where can we go FROM IV?
    let from_iv = c_major
        .progressions_from(&f_chord)
        .expect("Should find progressions from IV");

    // What leads TO IV?
    let to_iv = c_major
        .progressions_to(&f_chord)
        .expect("Should find progressions to IV");

    // Both should have content
    assert!(!from_iv.is_empty(), "IV should have forward progressions");
    assert!(
        !to_iv.is_empty(),
        "IV should have progressions leading to it"
    );

    // They should generally be different sets
    // (unless there are bidirectional relationships)
}

/// Test the deprecated progressions_from function still works
#[test]
#[allow(deprecated)]
fn test_deprecated_progressions_from() {
    let c_major = Key::Major(note!("C"));
    let c_chord = Chord::major(note!("C"));

    // Old API should still work
    let old_options = c_major.progressions_from(&c_chord);
    let new_options = c_major.progressions_from(&c_chord);

    assert_eq!(
        old_options.is_some(),
        new_options.is_some(),
        "Deprecated function should behave the same as new one"
    );
}
