use chordy::types::*;
use chordy::{note, pitch};

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
    ScaleType::Major,
    note_vec!("C", "D", "E", "F", "G", "A", "B")
);

scale_test!(
    test_major_scale_d,
    note!("D"),
    ScaleType::Major,
    note_vec!("D", "E", "F#", "G", "A", "B", "C#")
);

scale_test!(
    test_major_scale_c_sharp,
    note!("C#"),
    ScaleType::Major,
    note_vec!("C#", "D#", "E#", "F#", "G#", "A#", "B#")
);

scale_test!(
    test_major_scale_f,
    note!("F"),
    ScaleType::Major,
    note_vec!("F", "G", "A", "Bb", "C", "D", "E")
);

// Minor scale tests
scale_test!(
    test_natural_minor_scale_a,
    note!("A"),
    ScaleType::NaturalMinor,
    note_vec!("A", "B", "C", "D", "E", "F", "G")
);

scale_test!(
    test_harmonic_minor_scale_a,
    note!("A"),
    ScaleType::HarmonicMinor,
    note_vec!("A", "B", "C", "D", "E", "F", "G#")
);

// Modes tests
scale_test!(
    test_dorian_mode_d,
    note!("D"),
    ScaleType::Dorian,
    note_vec!("D", "E", "F", "G", "A", "B", "C")
);

scale_test!(
    test_phrygian_mode_e,
    note!("E"),
    ScaleType::Phrygian,
    note_vec!("E", "F", "G", "A", "B", "C", "D")
);

// More comprehensive tests
#[test]
fn test_scale_degree_functions() {
    let c_major = Scale::new(note!("C"), ScaleType::Major);

    // Test degree lookup
    assert_eq!(c_major.degree_of(&note!("C")), Some(1));
    assert_eq!(c_major.degree_of(&note!("G")), Some(5));
    assert_eq!(c_major.degree_of(&note!("F#")), None); // Not in scale
}

#[test]
fn test_chord_functions() {
    use chordy::types::ChordQuality::*;
    use chordy::types::HarmonicFunction::*;

    let c_major = Scale::new(note!("C"), ScaleType::Major);

    let tests: Vec<(NoteName, ChordQuality, HarmonicFunction)> = vec![
        // Diatonic chords
        (note!("C"), Major, Tonic),
        (note!("D"), Minor, Subdominant),
        (note!("E"), Minor, Dominant),
        (note!("F"), Major, Subdominant),
        (note!("G"), Major, Dominant),
        (note!("A"), Minor, Tonic),
        (note!("B"), Diminished, Dominant),
        // Some other chords
        (note!("Db"), Major, Subdominant),
        (note!("Eb"), Minor, Subdominant),
    ];

    tests.iter().for_each(|(root, quality, expected_function)| {
        let chord = Chord::new(*root, *quality, vec![]);
        assert_eq!(
            c_major.harmonic_function(&chord),
            Some(*expected_function),
            "Test failed for chord: {:?} {:?}, expected function: {:?}",
            root,
            quality,
            expected_function
        );
    });
}

#[test]
fn test_scale_transformations() {
    let c_major = Scale::new(note!("C"), ScaleType::Major);

    // Relative minor
    let a_minor = c_major.relative();
    assert_eq!(a_minor.tonic, note!("A"));
    assert_eq!(a_minor.scale_type, ScaleType::NaturalMinor);

    // Parallel minor
    let c_minor = c_major.parallel();
    assert_eq!(c_minor.tonic, note!("C"));
    assert_eq!(c_minor.scale_type, ScaleType::NaturalMinor);

    // Dominant
    let g_major = c_major.dominant();
    assert_eq!(g_major.tonic, note!("G"));
    assert_eq!(g_major.scale_type, ScaleType::Major);
}
