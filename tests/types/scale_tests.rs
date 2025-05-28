use chordy::types::*;
use chordy::note;

macro_rules! scale_test {
    ($name:ident, $root:expr, $scale_type:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let scale = Scale::new($root, $scale_type);
            let notes = scale.notes();
            assert_eq!(notes, $expected);
        }
    };
}

macro_rules! note_vec {
    ($($note:expr),*) => {
        vec![$(note!($note)),*]
    };
}

// Major scale tests
scale_test!(
    test_major_scale_c,
    note!("C"),
    scales::IONIAN,
    note_vec!("C", "D", "E", "F", "G", "A", "B")
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
    let c_major = Scale::new(note!("C"), scales::IONIAN);
    // Test basic degrees of c major
    assert_eq!(c_major.degree_of(&note!("C")), Some(ScaleDegree::TONIC));
    assert_eq!(c_major.degree_of(&note!("D")), Some(ScaleDegree::SUPERTONIC));
    assert_eq!(c_major.degree_of(&note!("E")), Some(ScaleDegree::MEDIANT));
    assert_eq!(c_major.degree_of(&note!("F")), Some(ScaleDegree::SUBDOMINANT));
    assert_eq!(c_major.degree_of(&note!("G")), Some(ScaleDegree::DOMINANT));
    assert_eq!(c_major.degree_of(&note!("A")), Some(ScaleDegree::SUBMEDIANT));
    assert_eq!(c_major.degree_of(&note!("B")), Some(ScaleDegree::LEADING_TONE));

    // Some notes in c major can be altered to be sharp or flat without clashing with scale tones
    assert_eq!(c_major.degree_of(&note!("C#")), Some(ScaleDegree::new(1, Some(Accidental::Sharp))));
    assert_eq!(c_major.degree_of(&note!("D#")), Some(ScaleDegree::new(2, Some(Accidental::Sharp))));
    assert_eq!(c_major.degree_of(&note!("F#")), Some(ScaleDegree::new(4, Some(Accidental::Sharp))));
    assert_eq!(c_major.degree_of(&note!("G#")), Some(ScaleDegree::new(5, Some(Accidental::Sharp))));
    assert_eq!(c_major.degree_of(&note!("A#")), Some(ScaleDegree::new(6, Some(Accidental::Sharp))));

    assert_eq!(c_major.degree_of(&note!("Db")), Some(ScaleDegree::new(2, Some(Accidental::Flat))));
    assert_eq!(c_major.degree_of(&note!("Eb")), Some(ScaleDegree::new(3, Some(Accidental::Flat))));
    assert_eq!(c_major.degree_of(&note!("Gb")), Some(ScaleDegree::new(5, Some(Accidental::Flat))));
    assert_eq!(c_major.degree_of(&note!("Ab")), Some(ScaleDegree::new(6, Some(Accidental::Flat))));
    assert_eq!(c_major.degree_of(&note!("Bb")), Some(ScaleDegree::new(7, Some(Accidental::Flat))));

    // some notes will clash with scale tones
    // this function will return the scale tones instead
    assert_eq!(c_major.degree_of(&note!("E#")), Some(ScaleDegree::new(4, None)));
    assert_eq!(c_major.degree_of(&note!("B#")), Some(ScaleDegree::new(1, None)));
    assert_eq!(c_major.degree_of(&note!("Cb")), Some(ScaleDegree::new(7, None)));
    assert_eq!(c_major.degree_of(&note!("Fb")), Some(ScaleDegree::new(3, None)));

    // --- E flat minor scale ---
    let eb_minor = Scale::new(note!("Eb"), scales::AEOLIAN);
    // test basic degrees of eb minor
    assert_eq!(eb_minor.degree_of(&note!("Eb")), Some(ScaleDegree::TONIC));
    assert_eq!(eb_minor.degree_of(&note!("F")), Some(ScaleDegree::SUPERTONIC));
    assert_eq!(eb_minor.degree_of(&note!("Gb")), Some(ScaleDegree::MEDIANT));
    assert_eq!(eb_minor.degree_of(&note!("Ab")), Some(ScaleDegree::SUBDOMINANT));
    assert_eq!(eb_minor.degree_of(&note!("Bb")), Some(ScaleDegree::DOMINANT));
    assert_eq!(eb_minor.degree_of(&note!("Cb")), Some(ScaleDegree::SUBMEDIANT));
    assert_eq!(eb_minor.degree_of(&note!("Db")), Some(ScaleDegree::SUBTONIC));
    
    // some notes in eb minor can be altered to be sharp or flat without clashing with scale tones
    assert_eq!(eb_minor.degree_of(&note!("E")), Some(ScaleDegree::new(1, Some(Accidental::Sharp))));
    assert_eq!(eb_minor.degree_of(&note!("Fb")), Some(ScaleDegree::new(2, Some(Accidental::Flat))));
    assert_eq!(eb_minor.degree_of(&note!("G")), Some(ScaleDegree::new(3, Some(Accidental::Sharp))));
    assert_eq!(eb_minor.degree_of(&note!("A")), Some(ScaleDegree::new(4, Some(Accidental::Sharp))));
    assert_eq!(eb_minor.degree_of(&note!("C")), Some(ScaleDegree::new(6, Some(Accidental::Sharp))));
    assert_eq!(eb_minor.degree_of(&note!("D")), Some(ScaleDegree::new(7, Some(Accidental::Sharp))));
    assert_eq!(eb_minor.degree_of(&note!("B#")), Some(ScaleDegree::new(6, Some(Accidental::Sharp))));

    // clashing notes
    assert_eq!(eb_minor.degree_of(&note!("D#")), Some(ScaleDegree::new(1, None)));
    assert_eq!(eb_minor.degree_of(&note!("E#")), Some(ScaleDegree::new(2, None)));
    assert_eq!(eb_minor.degree_of(&note!("F#")), Some(ScaleDegree::new(3, None)));
    assert_eq!(eb_minor.degree_of(&note!("G#")), Some(ScaleDegree::new(4, None)));
    assert_eq!(eb_minor.degree_of(&note!("A#")), Some(ScaleDegree::new(5, None)));
    assert_eq!(eb_minor.degree_of(&note!("B")), Some(ScaleDegree::new(6, None)));
    assert_eq!(eb_minor.degree_of(&note!("C#")), Some(ScaleDegree::new(7, None)));

    // Double accidentals with name preservation
    assert_eq!(eb_minor.degree_of(&note!("Abb")), Some(ScaleDegree::new(4, Some(Accidental::Flat))));
}

#[test]
fn test_chord_functions() {
    use chordy::types::HarmonicFunction::*;

    let c_major = Scale::new(note!("C"), scales::IONIAN);

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

    tests.iter().for_each(|(chord,  expected_function)| {
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
    let c_major = Scale::new(note!("C"), scales::IONIAN);

    // Relative minor
    let a_minor = c_major.relative().expect("Could not transform to relative scale");
    assert_eq!(a_minor.tonic, note!("A"));
    assert_eq!(a_minor.definition, scales::AEOLIAN);

    // Parallel minor
    let c_minor = c_major.parallel().expect("Could not transform to parallel scale");
    assert_eq!(c_minor.tonic, note!("C"));
    assert_eq!(c_minor.definition, scales::AEOLIAN);
}
