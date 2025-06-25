use chordy::prelude::*;

macro_rules! scale_test {
    ($name:ident, $root:expr, $scale_type:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let scale = Scale::from_definition($root, $scale_type);
            let notes = scale.notes();
            assert_eq!(notes, $expected);
        }
    };
}


// Major scale tests
scale_test!(
    test_major_scale_c,
    note!("C"),
    scales::IONIAN,
    note_vec!["C", "D", "E", "F", "G", "A", "B"]
);

scale_test!(
    test_major_scale_d,
    note!("D"),
    scales::IONIAN,
    note_vec!("D", "E", "F#", "G", "A", "B", "C#")
);

scale_test!(
    test_major_scale_c_sharp,
    note!("C#"),
    scales::IONIAN,
    note_vec!("C#", "D#", "E#", "F#", "G#", "A#", "B#")
);

scale_test!(
    test_major_scale_f,
    note!("F"),
    scales::IONIAN,
    note_vec!("F", "G", "A", "Bb", "C", "D", "E")
);

// Minor scale tests
scale_test!(
    test_natural_minor_scale_a,
    note!("A"),
    scales::AEOLIAN,
    note_vec!("A", "B", "C", "D", "E", "F", "G")
);

scale_test!(
    test_harmonic_minor_scale_a,
    note!("A"),
    scales::HARMONIC_MINOR,
    note_vec!("A", "B", "C", "D", "E", "F", "G#")
);

// Modes tests
scale_test!(
    test_dorian_mode_d,
    note!("D"),
    scales::DORIAN,
    note_vec!("D", "E", "F", "G", "A", "B", "C")
);

scale_test!(
    test_phrygian_mode_e,
    note!("E"),
    scales::PHRYGIAN,
    note_vec!("E", "F", "G", "A", "B", "C", "D")
);

scale_test!(
    test_lydian_mode_f,
    note!("F#"),
    scales::LYDIAN,
    note_vec!("F#", "G#", "A#", "B#", "C#", "D#", "E#")
);

scale_test!(
    test_altered_scale_c,
    note!("C"),
    scales::ALTERED,
    note_vec!("C", "Db", "Eb", "Fb", "Gb", "Ab", "Bb")
);

// More comprehensive tests
#[test]
fn test_scale_degree_functions() {
    // --- C Major Scale ---
    let c_major = Scale::from_definition(note!("C"), scales::IONIAN);
    // Test basic degrees of c major
    assert_eq!(c_major.degree_of(&note!("C")), Some(ScaleDegree::TONIC));
    assert_eq!(
        c_major.degree_of(&note!("D")),
        Some(ScaleDegree::SUPERTONIC)
    );
    assert_eq!(c_major.degree_of(&note!("E")), Some(ScaleDegree::MEDIANT));
    assert_eq!(
        c_major.degree_of(&note!("F")),
        Some(ScaleDegree::SUBDOMINANT)
    );
    assert_eq!(c_major.degree_of(&note!("G")), Some(ScaleDegree::DOMINANT));
    assert_eq!(
        c_major.degree_of(&note!("A")),
        Some(ScaleDegree::SUBMEDIANT)
    );
    assert_eq!(
        c_major.degree_of(&note!("B")),
        Some(ScaleDegree::LEADING_TONE)
    );

    // Some notes in c major can be altered to be sharp or flat without clashing with scale tones
    assert_eq!(
        c_major.degree_of(&note!("C#")),
        Some(ScaleDegree::new(1, Some(Accidental::Sharp)))
    );
    assert_eq!(
        c_major.degree_of(&note!("D#")),
        Some(ScaleDegree::new(2, Some(Accidental::Sharp)))
    );
    assert_eq!(
        c_major.degree_of(&note!("F#")),
        Some(ScaleDegree::new(4, Some(Accidental::Sharp)))
    );
    assert_eq!(
        c_major.degree_of(&note!("G#")),
        Some(ScaleDegree::new(5, Some(Accidental::Sharp)))
    );
    assert_eq!(
        c_major.degree_of(&note!("A#")),
        Some(ScaleDegree::new(6, Some(Accidental::Sharp)))
    );

    assert_eq!(
        c_major.degree_of(&note!("Db")),
        Some(ScaleDegree::new(2, Some(Accidental::Flat)))
    );
    assert_eq!(
        c_major.degree_of(&note!("Eb")),
        Some(ScaleDegree::new(3, Some(Accidental::Flat)))
    );
    assert_eq!(
        c_major.degree_of(&note!("Gb")),
        Some(ScaleDegree::new(5, Some(Accidental::Flat)))
    );
    assert_eq!(
        c_major.degree_of(&note!("Ab")),
        Some(ScaleDegree::new(6, Some(Accidental::Flat)))
    );
    assert_eq!(
        c_major.degree_of(&note!("Bb")),
        Some(ScaleDegree::new(7, Some(Accidental::Flat)))
    );

    // some notes will clash with scale tones
    // this function will return the scale tones instead
    assert_eq!(
        c_major.degree_of(&note!("E#")),
        Some(ScaleDegree::new(4, None))
    );
    assert_eq!(
        c_major.degree_of(&note!("B#")),
        Some(ScaleDegree::new(1, None))
    );
    assert_eq!(
        c_major.degree_of(&note!("Cb")),
        Some(ScaleDegree::new(7, None))
    );
    assert_eq!(
        c_major.degree_of(&note!("Fb")),
        Some(ScaleDegree::new(3, None))
    );

    // --- E flat minor scale ---
    let eb_minor = Scale::from_definition(note!("Eb"), scales::AEOLIAN);
    // test basic degrees of eb minor
    assert_eq!(eb_minor.degree_of(&note!("Eb")), Some(ScaleDegree::TONIC));
    assert_eq!(
        eb_minor.degree_of(&note!("F")),
        Some(ScaleDegree::SUPERTONIC)
    );
    assert_eq!(eb_minor.degree_of(&note!("Gb")), Some(ScaleDegree::MEDIANT));
    assert_eq!(
        eb_minor.degree_of(&note!("Ab")),
        Some(ScaleDegree::SUBDOMINANT)
    );
    assert_eq!(
        eb_minor.degree_of(&note!("Bb")),
        Some(ScaleDegree::DOMINANT)
    );
    assert_eq!(
        eb_minor.degree_of(&note!("Cb")),
        Some(ScaleDegree::SUBMEDIANT)
    );
    assert_eq!(
        eb_minor.degree_of(&note!("Db")),
        Some(ScaleDegree::SUBTONIC)
    );

    // some notes in eb minor can be altered to be sharp or flat without clashing with scale tones
    assert_eq!(
        eb_minor.degree_of(&note!("E")),
        Some(ScaleDegree::new(1, Some(Accidental::Sharp)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("Fb")),
        Some(ScaleDegree::new(2, Some(Accidental::Flat)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("G")),
        Some(ScaleDegree::new(3, Some(Accidental::Sharp)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("A")),
        Some(ScaleDegree::new(4, Some(Accidental::Sharp)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("C")),
        Some(ScaleDegree::new(6, Some(Accidental::Sharp)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("D")),
        Some(ScaleDegree::new(7, Some(Accidental::Sharp)))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("B#")),
        Some(ScaleDegree::new(6, Some(Accidental::Sharp)))
    );

    // clashing notes
    assert_eq!(
        eb_minor.degree_of(&note!("D#")),
        Some(ScaleDegree::new(1, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("E#")),
        Some(ScaleDegree::new(2, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("F#")),
        Some(ScaleDegree::new(3, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("G#")),
        Some(ScaleDegree::new(4, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("A#")),
        Some(ScaleDegree::new(5, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("B")),
        Some(ScaleDegree::new(6, None))
    );
    assert_eq!(
        eb_minor.degree_of(&note!("C#")),
        Some(ScaleDegree::new(7, None))
    );

    // Double accidentals with name preservation
    assert_eq!(
        eb_minor.degree_of(&note!("Abb")),
        Some(ScaleDegree::new(4, Some(Accidental::Flat)))
    );
}

#[test]
fn test_chord_functions() {
    use chordy::types::HarmonicFunction::*;

    let c_major = Scale::from_definition(note!("C"), scales::IONIAN);

    let tests: Vec<(Chord, HarmonicFunction)> = vec![
        // Diatonic chords
        (Chord::major(note!("C")), Tonic),
        (Chord::minor(note!("D")), Subdominant),
        (Chord::minor(note!("E")), Dominant),
        (Chord::major(note!("F")), Subdominant),
        (Chord::major(note!("G")), Dominant),
        (Chord::minor(note!("A")), Tonic),
        (Chord::diminished(note!("B")), Dominant),
        // Some other chords
        (Chord::major(note!("Db")), Subdominant),
        (Chord::minor(note!("Eb")), Dominant),
    ];

    tests.iter().for_each(|(chord, expected_function)| {
        assert_eq!(
            c_major.harmonic_function(chord),
            Some(*expected_function),
            "Test failed for chord: {:?}, expected function: {:?}",
            chord.root,
            expected_function
        );
    });
}

#[test]
fn test_scale_transformations() {
    let c_major = Scale::from_definition(note!("C"), scales::IONIAN);

    // Relative minor
    let a_minor = c_major
        .relative()
        .expect("Could not transform to relative scale");
    assert_eq!(a_minor.tonic, note!("A"));
    assert_eq!(a_minor, scales::AEOLIAN);

    // Parallel minor
    let c_minor = c_major
        .parallel()
        .expect("Could not transform to parallel scale");
    assert_eq!(c_minor.tonic, note!("C"));
    assert_eq!(c_minor, scales::AEOLIAN);
}

#[test]
fn test_scale_degree_to_interval_with_double_accidentals() {
    // Test double flat scale degrees
    let double_flat_2 = ScaleDegree::new(2, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_2.to_interval(), Interval::DIMINISHED_SECOND);
    
    let double_flat_3 = ScaleDegree::new(3, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_3.to_interval(), Interval::DIMINISHED_THIRD);
    
    let double_flat_4 = ScaleDegree::new(4, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_4.to_interval(), Interval::DOUBLY_DIMINISHED_FOURTH);
    
    let double_flat_5 = ScaleDegree::new(5, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_5.to_interval(), Interval::DOUBLY_DIMINISHED_FIFTH);
    
    let double_flat_6 = ScaleDegree::new(6, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_6.to_interval(), Interval::DIMINISHED_SIXTH);
    
    let double_flat_7 = ScaleDegree::new(7, Some(Accidental::DoubleFlat));
    assert_eq!(double_flat_7.to_interval(), Interval::DIMINISHED_SEVENTH);
    
    // Test double sharp scale degrees
    let double_sharp_1 = ScaleDegree::new(1, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_1.to_interval(), Interval::DOUBLY_AUGMENTED_UNISON);
    
    let double_sharp_2 = ScaleDegree::new(2, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_2.to_interval(), Interval::DOUBLY_AUGMENTED_SECOND);
    
    let double_sharp_3 = ScaleDegree::new(3, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_3.to_interval(), Interval::DOUBLY_AUGMENTED_THIRD);
    
    let double_sharp_4 = ScaleDegree::new(4, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_4.to_interval(), Interval::DOUBLY_AUGMENTED_FOURTH);
    
    let double_sharp_5 = ScaleDegree::new(5, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_5.to_interval(), Interval::DOUBLY_AUGMENTED_FIFTH);
    
    let double_sharp_6 = ScaleDegree::new(6, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_6.to_interval(), Interval::DOUBLY_AUGMENTED_SIXTH);
    
    let double_sharp_7 = ScaleDegree::new(7, Some(Accidental::DoubleSharp));
    assert_eq!(double_sharp_7.to_interval(), Interval::DOUBLY_AUGMENTED_SEVENTH);
}

#[test]
fn test_interval_to_scale_degree_with_double_accidentals() {
    // Test doubly diminished intervals
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_UNISON),
        ScaleDegree::new(1, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_SECOND),
        ScaleDegree::new(2, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_THIRD),
        ScaleDegree::new(3, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_FOURTH),
        ScaleDegree::new(4, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_FIFTH),
        ScaleDegree::new(5, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_SIXTH),
        ScaleDegree::new(6, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_SEVENTH),
        ScaleDegree::new(7, Some(Accidental::DoubleFlat))
    );
    
    // Test doubly augmented intervals
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_UNISON),
        ScaleDegree::new(1, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_SECOND),
        ScaleDegree::new(2, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_THIRD),
        ScaleDegree::new(3, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_FOURTH),
        ScaleDegree::new(4, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_FIFTH),
        ScaleDegree::new(5, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_SIXTH),
        ScaleDegree::new(6, Some(Accidental::DoubleSharp))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_SEVENTH),
        ScaleDegree::new(7, Some(Accidental::DoubleSharp))
    );
    
    // Test compound intervals with double accidentals
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_DIMINISHED_NINTH),
        ScaleDegree::new(2, Some(Accidental::DoubleFlat))
    );
    
    assert_eq!(
        ScaleDegree::from(Interval::DOUBLY_AUGMENTED_TENTH),
        ScaleDegree::new(3, Some(Accidental::DoubleSharp))
    );
}
