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
fn test_chord_pitches_ascending_order() {
    use chordy::Pitch;

    // Test basic triad - should stay in same octave
    let c_major = Chord::major(note!("C"));
    let pitches = c_major.pitches(4);
    let expected = vec![
        Pitch::new(Letter::C, Accidental::Natural, 4),
        Pitch::new(Letter::E, Accidental::Natural, 4),
        Pitch::new(Letter::G, Accidental::Natural, 4),
    ];
    assert_eq!(pitches, expected);

    // Test Dm7 - C should cross to next octave (D3, F3, A3, C4)
    let d_minor_7 = Chord::minor_7th(note!("D"));
    let pitches = d_minor_7.pitches(3);
    let expected = vec![
        Pitch::new(Letter::D, Accidental::Natural, 3),
        Pitch::new(Letter::F, Accidental::Natural, 3),
        Pitch::new(Letter::A, Accidental::Natural, 3),
        Pitch::new(Letter::C, Accidental::Natural, 4), // Should cross to octave 4
    ];
    assert_eq!(pitches, expected);

    // Test extended chord with 9th
    let c_maj9 = Chord::new(
        note!("C"),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SEVENTH,
            Interval::MAJOR_NINTH,
        ],
    );
    let pitches = c_maj9.pitches(4);
    let expected = vec![
        Pitch::new(Letter::C, Accidental::Natural, 4),
        Pitch::new(Letter::E, Accidental::Natural, 4),
        Pitch::new(Letter::G, Accidental::Natural, 4),
        Pitch::new(Letter::B, Accidental::Natural, 4),
        Pitch::new(Letter::D, Accidental::Natural, 5), // 9th in next octave
    ];
    assert_eq!(pitches, expected);

    // Test chord with intervals that create octave crossings
    let f_chord = Chord::new(
        note!("F"),
        vec![
            Interval::PERFECT_UNISON, // F
            Interval::MAJOR_SEVENTH,  // E - would be lower than F in same octave
            Interval::MAJOR_NINTH,    // G - already in next octave
        ],
    );
    let pitches = f_chord.pitches(4);
    let expected = vec![
        Pitch::new(Letter::F, Accidental::Natural, 4),
        Pitch::new(Letter::E, Accidental::Natural, 5), // E should bump to next octave
        Pitch::new(Letter::G, Accidental::Natural, 5), // 9th already in next octave
    ];
    assert_eq!(pitches, expected);
}

#[test]
fn test_chord_pitches_midi_ordering() {
    // Test that MIDI numbers are always ascending
    let d_minor_7 = Chord::minor_7th(note!("D"));
    let pitches = d_minor_7.pitches(3);

    let midi_numbers: Vec<i8> = pitches.iter().map(|p| p.midi_number()).collect();

    // Check that each MIDI number is greater than the previous
    for window in midi_numbers.windows(2) {
        assert!(
            window[1] > window[0],
            "MIDI numbers should be ascending, but {} <= {}",
            window[1],
            window[0]
        );
    }
}

#[test]
fn test_chord_dissonance_levels() {
    use chordy::prelude::HasIntervals;

    // Test perfect consonances
    let power_chord = Chord::new(
        note!("C"),
        vec![Interval::PERFECT_UNISON, Interval::PERFECT_FIFTH],
    );
    assert!(
        power_chord.dissonance_level() < 0.1,
        "Power chord should be very consonant"
    );

    // Test major triad (consonant)
    let c_major = Chord::major(note!("C"));
    let major_dissonance = c_major.dissonance_level();
    assert!(
        major_dissonance < 0.2,
        "Major triad should be consonant: {}",
        major_dissonance
    );

    // Test minor triad (consonant)
    let c_minor = Chord::minor(note!("C"));
    let minor_dissonance = c_minor.dissonance_level();
    assert!(
        minor_dissonance < 0.2,
        "Minor triad should be consonant: {}",
        minor_dissonance
    );

    // Test diminished triad (dissonant due to tritone)
    let c_dim = Chord::diminished(note!("C"));
    let dim_dissonance = c_dim.dissonance_level();
    assert!(
        dim_dissonance > 0.5,
        "Diminished triad should be quite dissonant: {}",
        dim_dissonance
    );

    // Test augmented triad (moderately dissonant)
    let c_aug = Chord::augmented(note!("C"));
    let aug_dissonance = c_aug.dissonance_level();
    assert!(
        aug_dissonance > 0.25 && aug_dissonance < 0.4,
        "Augmented triad should be moderately dissonant: {}",
        aug_dissonance
    );

    // Test seventh chords
    let c_maj7 = Chord::major_7th(note!("C"));
    let maj7_dissonance = c_maj7.dissonance_level();
    assert!(
        maj7_dissonance > 0.2 && maj7_dissonance < 0.4,
        "Major 7th should have mild dissonance: {}",
        maj7_dissonance
    );

    let c7 = Chord::dominant_7th(note!("C"));
    let dom7_dissonance = c7.dissonance_level();
    assert!(
        dom7_dissonance > 0.2 && dom7_dissonance < 0.4,
        "Dominant 7th should have mild dissonance: {}",
        dom7_dissonance
    );

    // Test extended chords
    let c_maj9 = Chord::new(
        note!("C"),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MAJOR_SEVENTH,
            Interval::MAJOR_NINTH,
        ],
    );
    let maj9_dissonance = c_maj9.dissonance_level();
    assert!(
        maj9_dissonance > 0.25 && maj9_dissonance < 0.4,
        "Major 9th should have moderate dissonance: {}",
        maj9_dissonance
    );

    // Test cluster chord (very dissonant)
    let cluster = Chord::new(
        note!("C"),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_SECOND,
            Interval::MAJOR_SECOND,
        ],
    );
    let cluster_dissonance = cluster.dissonance_level();
    assert!(
        cluster_dissonance > 0.5,
        "Cluster chord should be very dissonant: {}",
        cluster_dissonance
    );
}

#[test]
fn test_triads_from_scale_c_major() {
    // start with a scale
    let scale = Scale::from_definition(note!("C"), scales::IONIAN);

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
    let scale = Scale::from_definition(note!("Db"), scales::IONIAN);

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
    let scale = Scale::from_definition(note!("F#"), scales::HARMONIC_MINOR);

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
    let scale = Scale::from_definition(note!("C"), scales::IONIAN);

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
