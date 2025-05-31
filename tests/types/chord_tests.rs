use chordy::prelude::*;

#[test]
fn test_chord_notes() {
    // Basic triads
    let c_major = Chord::major(note!("C"));
    assert_eq!(c_major.notes(), vec![note!("C"), note!("E"), note!("G")]);

    let d_minor = Chord::minor(note!("D"));
    assert_eq!(d_minor.notes(), vec![note!("D"), note!("F"), note!("A")]);

    // Challenging spellings
    let f_sharp_major = Chord::major(note!("F#"));
    assert_eq!(
        f_sharp_major.notes(),
        vec![note!("F#"), note!("A#"), note!("C#")]
    );

    let g_flat_major = Chord::major(note!("Gb"));
    assert_eq!(
        g_flat_major.notes(),
        vec![note!("Gb"), note!("Bb"), note!("Db")]
    );

    // Tricky augmented/diminished cases
    let c_sharp_diminished = Chord::diminished(note!("C#"));
    assert_eq!(
        c_sharp_diminished.notes(),
        vec![note!("C#"), note!("E"), note!("G")]
    );

    let d_flat_augmented = Chord::augmented(note!("Db"));
    assert_eq!(
        d_flat_augmented.notes(),
        vec![note!("Db"), note!("F"), note!("A")]
    );

    // Test custom interval chords
    let custom = Chord::new(
        note!("C"),
        vec![
            Interval::MAJOR_SECOND,
            Interval::PERFECT_FOURTH,
            Interval::MAJOR_SIXTH,
        ],
    );
    assert_eq!(custom.notes(), vec![note!("D"), note!("F"), note!("A")]);
}

#[test]
fn test_triads_from_scale_c_major() {
    // start with a scale
    let scale = Scale::new(note!("C"), scales::IONIAN);

    // get the triads that are in this scale
    let triads: Vec<Chord> = scale.triads().collect();

    // triads contains C major
    assert!(triads.contains(&Chord::major(note!("C"))));
    assert!(triads.contains(&Chord::minor(note!("D"))));
    assert!(triads.contains(&Chord::minor(note!("E"))));
    assert!(triads.contains(&Chord::major(note!("F"))));
    assert!(triads.contains(&Chord::major(note!("G"))));
    assert!(triads.contains(&Chord::minor(note!("A"))));
    assert!(triads.contains(&Chord::diminished(note!("B"))));
}

#[test]
fn test_triads_from_scale_d_flat_major() {
    // start with a scale
    let scale = Scale::new(note!("Db"), scales::IONIAN);

    // get the triads that are in this scale
    let triads: Vec<Chord> = scale.triads().collect();

    assert!(triads.contains(&Chord::major(note!("Db"))));
    assert!(triads.contains(&Chord::minor(note!("Eb"))));
    assert!(triads.contains(&Chord::minor(note!("F"))));
    assert!(triads.contains(&Chord::major(note!("Gb"))));
    assert!(triads.contains(&Chord::major(note!("Ab"))));
    assert!(triads.contains(&Chord::minor(note!("Bb"))));
    assert!(triads.contains(&Chord::diminished(note!("C"))));
}

#[test]
fn test_triads_from_scale_f_sharp_harmonic_minor() {
    // start with a scale
    let scale = Scale::new(note!("F#"), scales::HARMONIC_MINOR);

    // get the triads that are in this scale
    let triads: Vec<Chord> = scale.triads().collect();

    assert!(triads.contains(&Chord::minor(note!("F#"))));
    assert!(triads.contains(&Chord::diminished(note!("G#"))));
    assert!(triads.contains(&Chord::augmented(note!("A"))));
    assert!(triads.contains(&Chord::minor(note!("B"))));
    assert!(triads.contains(&Chord::major(note!("C#"))));
    assert!(triads.contains(&Chord::major(note!("D"))));
    assert!(triads.contains(&Chord::diminished(note!("E#"))));
}

#[test]
fn test_sevenths_from_scale_c_major() {
    // start with a scale
    let scale = Scale::new(note!("C"), scales::IONIAN);

    // get the triads that are in this scale
    let sevenths: Vec<Chord> = scale.sevenths().collect();

    assert!(sevenths.contains(&Chord::major_7th(note!("C"))));
    assert!(sevenths.contains(&Chord::minor_7th(note!("D"))));
    assert!(sevenths.contains(&Chord::minor_7th(note!("E"))));
    assert!(sevenths.contains(&Chord::major_7th(note!("F"))));
    assert!(sevenths.contains(&Chord::dominant_7th(note!("G"))));
    assert!(sevenths.contains(&Chord::minor_7th(note!("A"))));
    assert!(sevenths.contains(&Chord::minor_7th_flat_5(note!("B"))));
}
