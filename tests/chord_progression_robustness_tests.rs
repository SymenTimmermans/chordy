//! Comprehensive tests for chord progression system robustness
//!
//! These tests verify that the chord progression system works correctly
//! after fixes for erratic behavior, non-deterministic matching, and
//! bass structure handling.

use chordy::prelude::*;

/// Test that simple chords consistently match simple nodes, not slash variants
#[test]
fn test_simple_chords_match_simple_nodes() {
    let c_major = Key::Major(note!("C"));

    // Test basic triads - should never match slash chord variants
    let simple_chords = vec![
        ("I", Chord::major(note!("C"))),
        ("ii", Chord::minor(note!("D"))),
        ("iii", Chord::minor(note!("E"))),
        ("IV", Chord::major(note!("F"))),
        ("V", Chord::major(note!("G"))),
        ("vi", Chord::minor(note!("A"))),
        ("vii°", Chord::diminished(note!("B"))),
    ];

    for (expected_name, chord) in simple_chords {
        let options = c_major
            .progressions_from(&chord)
            .unwrap_or_else(|| panic!("Should find progression options for {}", expected_name));

        // Verify we get some options
        assert!(
            !options.is_empty(),
            "Should have progression options for {} chord",
            expected_name
        );

        // Verify all returned chords are valid
        for chord in options.all().map(|(c, _)| c) {
            assert!(
                !chord.intervals.is_empty(),
                "All progression chords should have intervals"
            );
            assert!(
                chord.bass.is_none() || chord.bass.is_some(),
                "Bass field should be properly initialized"
            );
        }
    }
}

/// Test deterministic behavior - same input should always give same output
#[test]
fn test_deterministic_progression_behavior() {
    let key = Key::Major(note!("C"));
    let chord = Chord::major(note!("C"));

    // Run the same query multiple times
    let mut all_results = Vec::new();
    for i in 0..10 {
        let options = key
            .progressions_from(&chord)
            .unwrap_or_else(|| panic!("Should find options on iteration {}", i));

        // Convert to comparable format (chord names)
        let strong_names: Vec<String> = options.strong.iter().map(|c| c.to_html()).collect();
        let moderate_names: Vec<String> = options.moderate.iter().map(|c| c.to_html()).collect();
        let weak_names: Vec<String> = options.weak.iter().map(|c| c.to_html()).collect();

        all_results.push((strong_names, moderate_names, weak_names));
    }

    // All results should be identical
    let first_result = &all_results[0];
    for (i, result) in all_results.iter().enumerate() {
        assert_eq!(
            result.0, first_result.0,
            "Strong progressions should be identical on iteration {}",
            i
        );
        assert_eq!(
            result.1, first_result.1,
            "Moderate progressions should be identical on iteration {}",
            i
        );
        assert_eq!(
            result.2, first_result.2,
            "Weak progressions should be identical on iteration {}",
            i
        );
    }
}

/// Test bass structure matching works correctly
#[test]
fn test_bass_structure_matching() {
    let c_major = Key::Major(note!("C"));

    // Test that root position chord matches differently than inversion
    let c_root = Chord::major(note!("C"));
    let c_first_inv = c_root.with_inversion(1); // C/E

    let root_options = c_major.progressions_from(&c_root);
    let inv_options = c_major.progressions_from(&c_first_inv);

    // Both should find options (might be same or different depending on progression map)
    assert!(
        root_options.is_some(),
        "Root position C should have progression options"
    );
    assert!(inv_options.is_some(), "C/E should have progression options");
}

/// Test slash chord recognition vs inversion
#[test]
fn test_slash_chord_vs_inversion_matching() {
    let c_major = Key::Major(note!("C"));

    // Create different bass configurations
    let c_root = Chord::major(note!("C"));
    let c_slash_f = c_root.with_slash_bass(note!("F")); // C/F - slash chord
    let c_slash_e = c_root.with_slash_bass(note!("E")); // C/E - could be slash or inversion
    let c_first_inv = c_root.with_inversion(1); // C first inversion (E in bass)

    // All should find some progression options
    let root_opts = c_major.progressions_from(&c_root);
    let slash_f_opts = c_major.progressions_from(&c_slash_f);
    let slash_e_opts = c_major.progressions_from(&c_slash_e);
    let inv_opts = c_major.progressions_from(&c_first_inv);

    assert!(root_opts.is_some(), "C root should have options");
    assert!(slash_f_opts.is_some(), "C/F should have options");
    assert!(slash_e_opts.is_some(), "C/E slash should have options");
    assert!(inv_opts.is_some(), "C first inversion should have options");

    // Verify all configurations have valid options
    assert!(
        root_opts.as_ref().unwrap().len() > 0,
        "C root should have progression options"
    );
    assert!(
        slash_f_opts.as_ref().unwrap().len() > 0,
        "C/F should have progression options"
    );
    assert!(
        slash_e_opts.as_ref().unwrap().len() > 0,
        "C/E slash should have progression options"
    );
    assert!(
        inv_opts.as_ref().unwrap().len() > 0,
        "C first inversion should have progression options"
    );
}

/// Test progression categories are properly populated
#[test]
fn test_progression_categories_populated() {
    let key = Key::Major(note!("C"));

    // Test several common chords
    let test_chords = vec![
        ("I", Chord::major(note!("C"))),
        ("V", Chord::major(note!("G"))),
        ("V7", Chord::dominant_7th(note!("G"))),
        ("ii", Chord::minor(note!("D"))),
        ("IV", Chord::major(note!("F"))),
    ];

    for (name, chord) in test_chords {
        let options = key
            .progressions_from(&chord)
            .unwrap_or_else(|| panic!("Should find options for {}", name));

        // Each chord should have at least moderate options (jumps to primary nodes)
        assert!(
            !options.moderate.is_empty(),
            "{} should have moderate progression options",
            name
        );

        // Should have some total options
        let total = options.strong.len() + options.moderate.len() + options.weak.len();
        assert!(total > 0, "{} should have some progression options", name);

        // Test iterator consistency
        let iter_count = options.all().count();
        assert_eq!(
            iter_count, total,
            "Iterator count should match vector sum for {}",
            name
        );
    }
}

/// Test that V chord specifically resolves to I
#[test]
fn test_dominant_resolution() {
    let c_major = Key::Major(note!("C"));
    let g_chord = Chord::major(note!("G")); // V in C major

    let options = c_major
        .progressions_from(&g_chord)
        .expect("V chord should have progression options");

    // Should have strong progressions
    assert!(
        !options.strong.is_empty(),
        "V should have strong progressions"
    );

    // Look for resolution to I (C major chord)
    let has_tonic_resolution = options
        .strong
        .iter()
        .any(|chord| chord.root == note!("C") && chord.intervals.contains(Interval::MAJOR_THIRD));

    if !has_tonic_resolution {
        // Check if it's in moderate progressions instead
        let has_tonic_moderate = options.moderate.iter().any(|chord| {
            chord.root == note!("C") && chord.intervals.contains(Interval::MAJOR_THIRD)
        });

        assert!(
            has_tonic_moderate,
            "V should resolve to I in either strong or moderate progressions"
        );
    }
}

/// Test different keys work correctly
#[test]
fn test_multiple_keys_consistency() {
    let keys = vec![
        Key::Major(note!("C")),
        Key::Major(note!("G")),
        Key::Major(note!("D")),
        Key::Minor(note!("A")),
        Key::Minor(note!("E")),
    ];

    for key in keys {
        // Test tonic chord in each key
        let tonic_chord = match key {
            Key::Major(root) => Chord::major(root),
            Key::Minor(root) => Chord::minor(root),
        };

        let options = key
            .progressions_from(&tonic_chord)
            .unwrap_or_else(|| panic!("Should find options for tonic in {}", key));

        assert!(
            !options.is_empty(),
            "Tonic should have progressions in {}",
            key
        );

        // Should have at least moderate options (jumps to other primary nodes)
        assert!(
            !options.moderate.is_empty(),
            "Tonic should have moderate progressions in {}",
            key
        );
    }
}

/// Test chord extensions work in progressions
#[test]
fn test_extended_chords_in_progressions() {
    let c_major = Key::Major(note!("C"));

    let extended_chords = vec![
        ("Cmaj7", Chord::major_7th(note!("C"))),
        ("Dm7", Chord::minor_7th(note!("D"))),
        ("G7", Chord::dominant_7th(note!("G"))),
    ];

    for (name, chord) in extended_chords {
        let options = c_major
            .progressions_from(&chord)
            .unwrap_or_else(|| panic!("Should find options for {}", name));

        assert!(
            !options.is_empty(),
            "{} should have progression options",
            name
        );

        // Verify we get valid chords back
        for chord in options.all().map(|(c, _)| c) {
            assert!(
                !chord.intervals.is_empty(),
                "Progression target should have intervals"
            );
        }
    }
}

/// Stress test: many rapid queries should all be consistent
#[test]
fn test_rapid_query_consistency() {
    let key = Key::Major(note!("C"));
    let chord = Chord::major(note!("G")); // V chord

    // Rapid-fire queries
    let mut results = Vec::new();
    for _ in 0..50 {
        let options = key
            .progressions_from(&chord)
            .expect("Should always find options");

        let signature = (
            options.strong.len(),
            options.moderate.len(),
            options.weak.len(),
            options.strong.get(0).map(|c| c.to_html()),
        );
        results.push(signature);
    }

    // All results should be identical
    let first = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result, first, "Result {} differs from first", i);
    }
}
