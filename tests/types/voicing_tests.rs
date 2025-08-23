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
    "B2,D♯3,A3,B3,F#4"
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

guitar_voicing_test!(
    test_g7_guitar_voicing,
    Chord::dominant_7th(note!("G")),
    "G2,B2,D3,G3,B3,F4"
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

guitar_voicing_test!(
    test_d_major_over_f_sharp,
    Chord::major(note!("D")).with_slash_bass(note!("F♯")),
    "F#2,A2,D3,A3,D4,F#4"
);

guitar_voicing_test!(
    test_e_minor_over_b,
    Chord::minor(note!("E")).with_slash_bass(note!("B")),
    "B2,E3,G3,B3,E4"
);

guitar_voicing_test!(
    test_a_minor_over_c,
    Chord::minor(note!("A")).with_slash_bass(note!("C")),
    "C3,E3,A3,C4,E4"
);

guitar_voicing_test!(
    test_g_major_over_b,
    Chord::major(note!("G")).with_slash_bass(note!("B")),
    "B2,D3,G3,B3,G4"
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

// Guitar component unit tests
// These test the guitar voicing components in isolation

#[test]
fn test_string_state_parsing() {
    use chordy::StringState;
    
    assert_eq!("X".parse::<StringState>().unwrap(), StringState::Muted);
    assert_eq!("x".parse::<StringState>().unwrap(), StringState::Muted);
    assert_eq!("0".parse::<StringState>().unwrap(), StringState::Open);
    assert_eq!("1".parse::<StringState>().unwrap(), StringState::Fretted(1));
    assert_eq!(
        "12".parse::<StringState>().unwrap(),
        StringState::Fretted(12)
    );

    // Test invalid fret numbers
    assert!("25".parse::<StringState>().is_err());
    assert!("abc".parse::<StringState>().is_err());
}

#[test]
fn test_string_state_display() {
    use chordy::StringState;
    
    assert_eq!(format!("{}", StringState::Muted), "X");
    assert_eq!(format!("{}", StringState::Open), "0");
    assert_eq!(format!("{}", StringState::Fretted(5)), "5");
    assert_eq!(format!("{}", StringState::Fretted(12)), "12");
}

#[test]
fn test_string_state_methods() {
    use chordy::StringState;
    
    let muted = StringState::Muted;
    let open = StringState::Open;
    let fretted = StringState::Fretted(3);

    assert!(muted.is_muted());
    assert!(!muted.is_open());
    assert!(!muted.is_fretted());
    assert_eq!(muted.fret_number(), None);

    assert!(open.is_open());
    assert!(!open.is_muted());
    assert!(!open.is_fretted());
    assert_eq!(open.fret_number(), None);

    assert!(fretted.is_fretted());
    assert!(!fretted.is_muted());
    assert!(!fretted.is_open());
    assert_eq!(fretted.fret_number(), Some(3));
}

#[test]
fn test_guitar_tuning() {
    use chordy::GuitarTuning;
    
    let standard = GuitarTuning::standard();
    assert_eq!(standard.strings[0], "E2".parse().unwrap()); // Low E
    assert_eq!(standard.strings[1], "A2".parse().unwrap()); // A
    assert_eq!(standard.strings[2], "D3".parse().unwrap()); // D
    assert_eq!(standard.strings[3], "G3".parse().unwrap()); // G
    assert_eq!(standard.strings[4], "B3".parse().unwrap()); // B
    assert_eq!(standard.strings[5], "E4".parse().unwrap()); // High E

    let drop_d = GuitarTuning::drop_d();
    assert_eq!(drop_d.strings[0], "D2".parse().unwrap()); // Low D
    assert_eq!(drop_d.strings[1], "A2".parse().unwrap()); // A (same as standard)
}

#[test]
fn test_guitar_fingering_parsing() {
    use chordy::{GuitarFingering, StringState};
    
    // Test C major open chord (032010,2)
    let c_major = "032010,2".parse::<GuitarFingering>().unwrap();
    assert_eq!(c_major.root_string, 2);
    assert_eq!(c_major.frets[0], StringState::Open); // Low E open
    assert_eq!(c_major.frets[1], StringState::Fretted(3)); // A string 3rd fret
    assert_eq!(c_major.frets[2], StringState::Fretted(2)); // D string 2nd fret
    assert_eq!(c_major.frets[3], StringState::Open); // G string open
    assert_eq!(c_major.frets[4], StringState::Fretted(1)); // B string 1st fret
    assert_eq!(c_major.frets[5], StringState::Open); // High E open

    // Test barre chord (133211,0)
    let f_major_barre = "133211,0".parse::<GuitarFingering>().unwrap();
    assert_eq!(f_major_barre.root_string, 0);
    assert!(f_major_barre.is_barre());
    assert_eq!(f_major_barre.barre_fret(), Some(1));

    // Test invalid formats
    assert!("12345,2".parse::<GuitarFingering>().is_err()); // Too short
    assert!("1234567,2".parse::<GuitarFingering>().is_err()); // Too long
    assert!("123456".parse::<GuitarFingering>().is_err()); // No root string
    assert!("123456,6".parse::<GuitarFingering>().is_err()); // Invalid root string
}

#[test]
fn test_guitar_fingering_display() {
    use chordy::{GuitarFingering, StringState};
    
    let fingering = GuitarFingering::new(
        [
            StringState::Open,
            StringState::Fretted(3),
            StringState::Fretted(2),
            StringState::Open,
            StringState::Fretted(1),
            StringState::Open,
        ],
        2,
    )
    .unwrap();

    assert_eq!(format!("{}", fingering), "032010");
}

#[test]
fn test_guitar_fingering_barre_detection() {
    use chordy::GuitarFingering;
    
    // Open chord - not a barre
    let open_chord = "032010,2".parse::<GuitarFingering>().unwrap();
    assert!(!open_chord.is_barre());
    assert_eq!(open_chord.barre_fret(), None);

    // Barre chord - is a barre
    let barre_chord = "133211,0".parse::<GuitarFingering>().unwrap();
    assert!(barre_chord.is_barre());
    assert_eq!(barre_chord.barre_fret(), Some(1));

    // Mixed fretted/muted (no open) - is a barre
    let partial_barre = "1X321X,0".parse::<GuitarFingering>().unwrap();
    assert!(partial_barre.is_barre());
    assert_eq!(partial_barre.barre_fret(), Some(1));
}

#[test]
fn test_guitar_fingering_to_pitches() {
    use chordy::{GuitarFingering, GuitarTuning};
    
    let tuning = GuitarTuning::standard();

    // Test C major open chord (032010)
    let c_major = "032010,2".parse::<GuitarFingering>().unwrap();
    let pitches = c_major.to_pitches(&tuning);

    // Should have 6 pitches (no muted strings)
    assert_eq!(pitches.len(), 6);

    // Check the actual pitches produced
    assert_eq!(pitches[0], "E2".parse().unwrap()); // Low E open
    assert_eq!(pitches[1], "C3".parse().unwrap()); // A string 3rd fret = C
    assert_eq!(pitches[2], "E3".parse().unwrap()); // D string 2nd fret = E
    assert_eq!(pitches[3], "G3".parse().unwrap()); // G string open
    assert_eq!(pitches[4], "C4".parse().unwrap()); // B string 1st fret = C
    assert_eq!(pitches[5], "E4".parse().unwrap()); // High E open
}

#[test]
fn test_guitar_fingering_root_pitch() {
    use chordy::{GuitarFingering, GuitarTuning};
    
    let tuning = GuitarTuning::standard();

    // C major with root on D string (index 2)
    let c_major = "032010,2".parse::<GuitarFingering>().unwrap();
    let root_pitch = c_major.root_pitch(&tuning).unwrap();
    assert_eq!(root_pitch, "E3".parse().unwrap()); // D string 2nd fret = E

    // Wait, that's not right. Let me fix this - C major root should be C
    // The root string should be A string (index 1) for C major chord
    let c_major_correct = "032010,1".parse::<GuitarFingering>().unwrap();
    let root_pitch_correct = c_major_correct.root_pitch(&tuning).unwrap();
    assert_eq!(root_pitch_correct, "C3".parse().unwrap()); // A string 3rd fret = C
}

#[test]
fn test_guitar_fingering_transposition() {
    use chordy::GuitarFingering;
    
    // Test transposing a barre chord - use F major (133211) as base
    let f_major_barre = "133211,0".parse::<GuitarFingering>().unwrap();
    assert!(f_major_barre.is_barre()); // All fretted, no open strings
    assert_eq!(f_major_barre.barre_fret(), Some(1));

    // Transpose to 3rd fret (should be G major)
    let g_major = f_major_barre.transpose_to_fret(3).unwrap();
    assert_eq!(format!("{}", g_major), "355433");
    assert_eq!(g_major.root_string, 0);
    assert_eq!(g_major.barre_fret(), Some(3));

    // Transpose to 5th fret (should be A major)
    let a_major = f_major_barre.transpose_to_fret(5).unwrap();
    assert_eq!(format!("{}", a_major), "577655");

    // Try to transpose an open chord (should fail)
    let c_major_open = "032010,1".parse::<GuitarFingering>().unwrap();
    assert!(!c_major_open.is_barre());
    assert!(c_major_open.transpose_to_fret(3).is_err());

    // Test transposing to fret 0 (should work - becomes E major open)
    let e_major = f_major_barre.transpose_to_fret(0).unwrap();
    assert_eq!(format!("{}", e_major), "022100");
}

#[test]
fn test_guitar_chord_analyzer_basic() {
    use chordy::IntervalFirstGuitarFinder;
    use std::collections::HashSet;

    let finder = IntervalFirstGuitarFinder::new();

    // Test C major chord analysis
    let c_major = Chord::major(note!("C"));
    let voicings = finder.find_voicings(&c_major);

    // Should find at least one voicing
    assert!(
        !voicings.is_empty(),
        "Should find voicings for C major chord"
    );

    // The best voicing should have a reasonable score (lower is better)
    let (best_fingering, best_score) = &voicings[0];
    assert!(
        *best_score < 50.0,
        "Best voicing score should be reasonable: {}",
        best_score
    );

    // Should produce actual pitches
    let pitches = best_fingering.to_pitches(&chordy::GuitarTuning::standard());
    assert!(!pitches.is_empty(), "Should produce actual pitches");

    // Should contain the chord tones (C, E, G)
    let notes: HashSet<_> = pitches.iter().map(|p| p.name).collect();
    assert!(notes.contains(&note!("C")), "Should contain C");
    assert!(notes.contains(&note!("E")), "Should contain E");
    assert!(notes.contains(&note!("G")), "Should contain G");
}

#[test]
fn test_guitar_chord_analyzer_different_chords() {
    use chordy::IntervalFirstGuitarFinder;

    let finder = IntervalFirstGuitarFinder::new();

    // Test various chord types
    let test_chords = [
        Chord::major(note!("G")),
        Chord::minor(note!("A")),
        Chord::major(note!("D")),
        Chord::minor(note!("E")),
    ];

    for chord in &test_chords {
        let voicings = finder.find_voicings(chord);
        assert!(
            !voicings.is_empty(),
            "Should find voicings for chord: {}",
            chord
        );

        // Check that the best voicing contains the chord root
        let (best_fingering, _score) = &voicings[0];
        let pitches = best_fingering.to_pitches(&chordy::GuitarTuning::standard());
        let has_root = pitches.iter().any(|p| p.name == chord.root());
        assert!(has_root, "Should contain root note for chord: {}", chord);
    }
}

#[test]
fn test_interval_first_guitar_finder_score_ordering() {
    use chordy::IntervalFirstGuitarFinder;

    let finder = IntervalFirstGuitarFinder::new();
    let c_major = Chord::major(note!("C"));
    let voicings = finder.find_voicings(&c_major);

    // Should be sorted by score (ascending - lower is better)
    for i in 1..voicings.len() {
        assert!(
            voicings[i - 1].1 <= voicings[i].1,
            "Voicings should be sorted by score (lower is better)"
        );
    }
}
