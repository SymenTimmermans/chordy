use chordy::prelude::*;
use chordy::VoicingStyle;

/// Macro for testing guitar chord voicings with expected pitches in CSV format
macro_rules! guitar_voicing_test {
    ($test_name:ident, $chord_expr:expr, $expected_pitches:expr) => {
        #[test]
        fn $test_name() {
            let chord = $chord_expr;
            let voiced = chord.voice_for_guitar().unwrap();

            // Parse expected pitches from CSV string
            let expected: Vec<Pitch> = $expected_pitches
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            // Main assertion
            assert_eq!(
                voiced.pitches, expected,
                "Guitar voicing for {} does not match expected pitches",
                chord
            );

            // Additional validations
            assert_eq!(
                voiced.info.style,
                VoicingStyle::Guitar,
                "Should use Guitar voicing style"
            );
            assert!(!voiced.pitches.is_empty(), "Voicing should produce pitches");

            // Check guitar range (E2 to E6)
            for pitch in &voiced.pitches {
                assert!(
                    pitch.midi_number() >= 40 && pitch.midi_number() <= 88,
                    "Pitch {} outside guitar range (E2-E6)",
                    pitch
                );
            }
        }
    };
}

// Guitar voicing tests using the convenient macro
guitar_voicing_test!(
    test_e_minor_guitar_voicing,
    Chord::minor(note!("E")),
    "E2,B2,E3,G3,B3,E4"
);

guitar_voicing_test!(
    test_e_major_guitar_voicing,
    Chord::major(note!("E")),
    "E2,B2,E3,G#3,B3,E4"
);

guitar_voicing_test!(
    test_e7_guitar_voicing,
    Chord::dominant_7th(note!("E")),
    "E2,B2,D3,G♯3,B3,E4"
);

guitar_voicing_test!(
    test_a_minor_guitar_voicing,
    Chord::minor(note!("A")),
    "A2,E3,A3,C4,E4"
);

guitar_voicing_test!(
    test_a_major_guitar_voicing,
    Chord::major(note!("A")),
    "A2,E3,A3,C#4,E4"
);

guitar_voicing_test!(
    test_c_major_guitar_voicing,
    Chord::major(note!("C")),
    "C3,E3,G3,C4,E4"
);

guitar_voicing_test!(
    test_g_major_guitar_voicing,
    Chord::major(note!("G")),
    "G2,B2,D3,G3,B3,G4"
);

guitar_voicing_test!(
    test_d_major_guitar_voicing,
    Chord::major(note!("D")),
    "D3,A3,D4,F♯4"
);

guitar_voicing_test!(
    test_b7_guitar_voicing,
    Chord::dominant_7th(note!("B")),
    "B2,D♯3,A3,B3"
);

guitar_voicing_test!(
    test_dmaj7_guitar_voicing,
    Chord::major_7th(note!("D")),
    "D3,A3,C#4,F#4"
);

guitar_voicing_test!(
    test_d7_guitar_voicing,
    Chord::dominant_7th(note!("D")),
    "D3,A3,C4,F#4"
);

// Additional common chord voicings
guitar_voicing_test!(
    test_f_major_guitar_voicing,
    Chord::major(note!("F")),
    "F2,C3,F3,A3,C4,F4"
);

guitar_voicing_test!(
    test_g_minor_guitar_voicing,
    Chord::minor(note!("G")),
    "G2,D3,G3,A#3,D4,G4"
);

guitar_voicing_test!(
    test_b_minor_guitar_voicing,
    Chord::minor(note!("B")),
    "B2,F♯3,B3,D4,F♯4,B4"
);

// Slash chord tests - verifying bass note handling
guitar_voicing_test!(
    test_c_major_over_e,
    Chord::major(note!("C")).with_slash_bass(note!("E")),
    "E2,C3,E3,G3,C4,E4"
);

guitar_voicing_test!(
    test_c_minor_over_e,
    Chord::minor(note!("C")).with_slash_bass(note!("E")),
    "E2,C3,G3,C4,D♯4,G4"
);

guitar_voicing_test!(
    test_a_major_over_e,
    Chord::major(note!("A")).with_slash_bass(note!("E")),
    "E2,A2,E3,A3,C#4,E4"
);

guitar_voicing_test!(
    test_d_major_over_a,
    Chord::major(note!("D")).with_slash_bass(note!("A")),
    "A2,D3,A3,D4,F#4"
);

guitar_voicing_test!(
    test_c_major_over_g,
    Chord::major(note!("C")).with_slash_bass(note!("G")),
    "G2,C3,E3,G3,C4,E4"
);

#[test]
fn test_cm_over_e_slash_chord() {
    use chordy::IntervalFirstGuitarFinder;

    let finder = IntervalFirstGuitarFinder::new();

    // Test Cm/E - should find a pattern like 035543 (E bass with C minor chord)
    let cm_over_e = Chord::minor(note!("C")).with_slash_bass(note!("E"));
    let voicings = finder.find_voicings(&cm_over_e);

    // Should find at least one voicing
    assert!(!voicings.is_empty(), "Should find voicings for Cm/E chord");

    // The best voicing should be a C minor chord with E in the bass
    let (best_fingering, best_score) = &voicings[0];
    let pitches = best_fingering.to_pitches(&chordy::GuitarTuning::standard());

    // Verify it's actually C minor (has E♭/D♯)
    let has_c = pitches.iter().any(|p| p.name == note!("C"));
    let has_eb = pitches
        .iter()
        .any(|p| p.name == note!("E♭") || p.name == note!("D♯"));
    let has_g = pitches.iter().any(|p| p.name == note!("G"));
    let has_e_bass = pitches[0].name == note!("E");

    assert!(has_c, "Cm/E should contain C");
    assert!(has_eb, "Cm/E should contain E♭ (minor third)");
    assert!(has_g, "Cm/E should contain G");
    assert!(has_e_bass, "Cm/E should have E as the bass note");

    println!(
        "Cm/E best voicing: Score {:.2}, Pattern: {:?}",
        best_score, best_fingering
    );
    println!("Pitches: {:?}", pitches);
}

#[test]
fn test_interval_first_approach() {
    use chordy::IntervalFirstGuitarFinder;

    let finder = IntervalFirstGuitarFinder::new();

    // Test C major chord
    let c_major = Chord::major(note!("C"));
    let voicings = finder.find_voicings(&c_major);

    // Should find at least one voicing
    assert!(!voicings.is_empty(), "Should find voicings for C major");

    // Show all voicings found
    println!("C major voicings found:");
    for (i, (fingering, score)) in voicings.iter().take(3).enumerate() {
        let pitches = fingering.to_pitches(&chordy::GuitarTuning::standard());
        println!(
            "  {}. Score: {:.2}, Fingering: {:?}",
            i + 1,
            score,
            fingering
        );
        println!("     Pitches: {:?}", pitches);
    }

    if let Some((best_fingering, score)) = voicings.first() {
        let pitches = best_fingering.to_pitches(&chordy::GuitarTuning::standard());

        // Check that we have some of the expected notes
        let has_c = pitches.iter().any(|p| p.name == note!("C"));
        let has_e = pitches.iter().any(|p| p.name == note!("E"));
        let has_g = pitches.iter().any(|p| p.name == note!("G"));

        assert!(has_c, "Should contain C");
        assert!(has_e, "Should contain E");
        assert!(has_g, "Should contain G");
    }

    // Test E7 chord (should work better than old system)
    let e7 = Chord::dominant_7th(note!("E"));
    let e7_voicings = finder.find_voicings(&e7);

    // Show E7 voicings for debugging
    println!("E7 voicings found:");
    for (i, (fingering, score)) in e7_voicings.iter().take(3).enumerate() {
        let pitches = fingering.to_pitches(&chordy::GuitarTuning::standard());
        println!(
            "  {}. Score: {:.2}, Fingering: {:?}",
            i + 1,
            score,
            fingering
        );
        println!("     Pitches: {:?}", pitches);
    }

    if let Some((e7_fingering, _e7_score)) = e7_voicings.first() {
        let e7_pitches = e7_fingering.to_pitches(&chordy::GuitarTuning::standard());

        // Check for E7 notes (E, G#, B, D)
        let has_e = e7_pitches.iter().any(|p| p.name == note!("E"));
        let has_gs = e7_pitches.iter().any(|p| p.name == note!("G♯"));
        let has_b = e7_pitches.iter().any(|p| p.name == note!("B"));
        let has_d = e7_pitches.iter().any(|p| p.name == note!("D"));

        assert!(has_e, "E7 should contain E");
        assert!(has_gs, "E7 should contain G♯");
        assert!(has_b, "E7 should contain B");
        assert!(has_d, "E7 should contain D (♭7)");
    }
}
