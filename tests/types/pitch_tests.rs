use chordy::pitch;
use chordy::types::*;

#[test]
fn test_pitch_creation() {
    let pitch = Pitch::new(Letter::C, Accidental::Natural, 4);
    assert_eq!(pitch.to_string(), "C4");

    let pitch = Pitch::new(Letter::A, Accidental::Sharp, 3);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(pitch.to_string(), "A♯3");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "A#3");

    let pitch = Pitch::new(Letter::B, Accidental::Flat, 5);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(pitch.to_string(), "B♭5");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "Bb5");
}

#[test]
fn test_midi_number() {
    assert_eq!(pitch!("B#2").midi_number(), 60);
    assert_eq!(pitch!("C3").midi_number(), 60);
    assert_eq!(pitch!("G#5").midi_number(), 92);
    assert_eq!(pitch!("Ab-1").midi_number(), 20);
    assert_eq!(pitch!("Cbb3").midi_number(), 58);
    assert_eq!(pitch!("Dbb3").midi_number(), 60);
    assert_eq!(pitch!("Dbb4").midi_number(), 72);
}

#[test]
fn test_pitch_from_str() {
    // Test basic cases
    assert_eq!(
        "C4".parse(),
        Ok(Pitch::new(Letter::C, Accidental::Natural, 4))
    );
    assert_eq!(
        "D5".parse(),
        Ok(Pitch::new(Letter::D, Accidental::Natural, 5))
    );

    // Test negative octaves
    assert_eq!(
        "C-2".parse(),
        Ok(Pitch::new(Letter::C, Accidental::Natural, -2))
    );
    assert_eq!(
        "F#-1".parse(),
        Ok(Pitch::new(Letter::F, Accidental::Sharp, -1))
    );

    // Test accidentals
    assert_eq!(
        "Ab3".parse(),
        Ok(Pitch::new(Letter::A, Accidental::Flat, 3))
    );
    assert_eq!(
        "G#4".parse(),
        Ok(Pitch::new(Letter::G, Accidental::Sharp, 4))
    );
    assert_eq!(
        "Bbb5".parse(),
        Ok(Pitch::new(Letter::B, Accidental::DoubleFlat, 5))
    );

    // Test unicode accidentals
    assert_eq!(
        "A♭3".parse(),
        Ok(Pitch::new(Letter::A, Accidental::Flat, 3))
    );
    assert_eq!(
        "G♯4".parse(),
        Ok(Pitch::new(Letter::G, Accidental::Sharp, 4))
    );

    // Test double unicode accidentals
    assert_eq!(
        "G♯♯4".parse(),
        Ok(Pitch::new(Letter::G, Accidental::DoubleSharp, 4))
    );
    assert_eq!(
        "B♭♭5".parse(),
        Ok(Pitch::new(Letter::B, Accidental::DoubleFlat, 5))
    );
    //
    // Test double unicode accidentals
    assert_eq!(
        "G𝄪4".parse(),
        Ok(Pitch::new(Letter::G, Accidental::DoubleSharp, 4))
    );
    assert_eq!(
        "B𝄫5".parse(),
        Ok(Pitch::new(Letter::B, Accidental::DoubleFlat, 5))
    );

    // Test invalid cases
    assert!("".parse::<Pitch>().is_err());
    assert!("C".parse::<Pitch>().is_err());
    assert!("4".parse::<Pitch>().is_err());
    assert!("H4".parse::<Pitch>().is_err()); // Invalid letter
    assert!("C#".parse::<Pitch>().is_err()); // Missing octave
    assert!("C4.5".parse::<Pitch>().is_err()); // Invalid octave
}

#[test]
fn test_pitch_double_flat_transpose() {
    assert_eq!(pitch!("Cbb4").transpose(2), pitch!("Dbb4"));
}

#[test]
fn test_pitch_b_sharp_to_c_sharp_transpose() {
    assert_eq!(pitch!("B#3").transpose(1), pitch!("C#4")); // B#→C#
}

#[test]
fn test_pitch_transpose() {
    // Basic single semitone transpositions
    assert_eq!(pitch!("C4").transpose(1), pitch!("C#4")); // C→C#
    assert_eq!(pitch!("C#4").transpose(-1), pitch!("C4")); // C#→C
    assert_eq!(pitch!("D4").transpose(1), pitch!("D#4")); // D→D#
    assert_eq!(pitch!("Eb4").transpose(-1), pitch!("D4")); // Eb→D

    // Whole step transpositions
    assert_eq!(pitch!("C4").transpose(2), pitch!("D4")); // C→D
    assert_eq!(pitch!("D4").transpose(-2), pitch!("C4")); // D→C

    // Enharmonic spelling special cases
    assert_eq!(pitch!("B4").transpose(1), pitch!("C5")); // B→C not B#
    assert_eq!(pitch!("C4").transpose(-1), pitch!("B3")); // C→B not Cb
    assert_eq!(pitch!("E4").transpose(1), pitch!("F4")); // E→F not E#
    assert_eq!(pitch!("F4").transpose(-1), pitch!("E4")); // F→E not Fb

    // Larger intervals
    assert_eq!(pitch!("C4").transpose(12), pitch!("C5")); // Octave up
    assert_eq!(pitch!("C5").transpose(-12), pitch!("C4")); // Octave down
    assert_eq!(pitch!("G4").transpose(7), pitch!("D5")); // Perfect 5th up
    assert_eq!(pitch!("D5").transpose(-7), pitch!("G4")); // Perfect 5th down

    // Negative octaves
    assert_eq!(pitch!("C-2").transpose(12), pitch!("C-1"));
    assert_eq!(pitch!("C-1").transpose(-12), pitch!("C-2"));

    // Double accidentals
    assert_eq!(pitch!("Cbb4").transpose(2), pitch!("Dbb4"));
    assert_eq!(pitch!("D##4").transpose(-2), pitch!("C##4"));

    // Chromatic scale tests
    let chromatic_up = [
        pitch!("C4"),
        pitch!("C#4"),
        pitch!("D4"),
        pitch!("D#4"),
        pitch!("E4"),
        pitch!("F4"),
        pitch!("F#4"),
        pitch!("G4"),
        pitch!("G#4"),
        pitch!("A4"),
        pitch!("A#4"),
        pitch!("B4"),
        pitch!("C5"),
    ];

    let chromatic_down = [
        pitch!("C5"),
        pitch!("B4"),
        pitch!("Bb4"),
        pitch!("A4"),
        pitch!("Ab4"),
        pitch!("G4"),
        pitch!("Gb4"),
        pitch!("F4"),
        pitch!("E4"),
        pitch!("Eb4"),
        pitch!("D4"),
        pitch!("Db4"),
        pitch!("C4"),
    ];

    // Test ascending chromatic
    let mut pitch = pitch!("C4");
    for (i, expected) in chromatic_up.iter().enumerate() {
        assert_eq!(pitch, *expected, "{} is expected to be {}", pitch, expected);
        pitch = pitch!("C4").transpose(i as i8 + 1);
    }

    // Test descending chromatic
    let mut pitch = pitch!("C5");
    for (i, expected) in chromatic_down.iter().enumerate() {
        assert_eq!(pitch, *expected, "{} is expected to be {}", pitch, expected);
        pitch = pitch!("C5").transpose(-(i as i8 + 1));
    }

    // Extreme transpositions
    assert_eq!(pitch!("C4").transpose(24), pitch!("C6")); // 2 octaves up
    assert_eq!(pitch!("C4").transpose(-24), pitch!("C2")); // 2 octaves down

    // Edge cases with accidentals
    assert_eq!(pitch!("F#4").transpose(1), pitch!("G4")); // F#→G not F##
    assert_eq!(pitch!("Gb4").transpose(-1), pitch!("F4")); // Gb→F not Gbb
    assert_eq!(pitch!("B#3").transpose(1), pitch!("C#4")); // B#→C#
    assert_eq!(pitch!("Cb4").transpose(-1), pitch!("Bb3")); // Cb→Bb
                                                            //
    assert_eq!(pitch!("C4").transpose(0), pitch!("C4")); // No change
                                                         //
    assert_eq!(pitch!("G#4").transpose(-2), pitch!("F#4")); // G#-F# not Gb
}

#[test]
fn test_pitch_frequency_conversion() {
    // Test standard concert pitch
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);
    assert!((a4.to_frequency() - 440.0).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(440.0), a4);

    // Test C4 (middle C)
    let c4 = Pitch::new(Letter::C, Accidental::Natural, 4);
    assert!((c4.to_frequency() - 261.63).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(261.63), c4);

    // Test G4
    let g4 = Pitch::new(Letter::G, Accidental::Natural, 4);
    assert!((g4.to_frequency() - 392.0).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(392.0), g4);

    // Test E4
    let e4 = Pitch::new(Letter::E, Accidental::Natural, 4);
    assert!((e4.to_frequency() - 329.63).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(329.63), e4);

    // Test A3 (octave below A4)
    let a3 = Pitch::new(Letter::A, Accidental::Natural, 3);
    assert!((a3.to_frequency() - 220.0).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(220.0), a3);

    // Test A5 (octave above A4)
    let a5 = Pitch::new(Letter::A, Accidental::Natural, 5);
    assert!((a5.to_frequency() - 880.0).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(880.0), a5);

    // Test sharp notes
    let c_sharp4 = Pitch::new(Letter::C, Accidental::Sharp, 4);
    assert!((c_sharp4.to_frequency() - 277.18).abs() < 0.01);
    // Note: from_frequency may return either C♯4 or D♭4 due to enharmonic equivalence
    let freq_pitch = Pitch::from_frequency(277.18);
    assert!(freq_pitch.is_enharmonic_with(&c_sharp4));

    // Test flat notes
    let d_flat4 = Pitch::new(Letter::D, Accidental::Flat, 4);
    assert!((d_flat4.to_frequency() - 277.18).abs() < 0.01);
    assert!(freq_pitch.is_enharmonic_with(&d_flat4));

    // Test enharmonic equivalence in frequency space
    assert!(c_sharp4.is_enharmonic_with(&d_flat4));
    assert!((c_sharp4.to_frequency() - d_flat4.to_frequency()).abs() < 0.01);

    // Test extreme frequencies
    let c0 = Pitch::new(Letter::C, Accidental::Natural, 0);
    assert!((c0.to_frequency() - 16.35).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(16.35), c0);

    let c8 = Pitch::new(Letter::C, Accidental::Natural, 8);
    assert!((c8.to_frequency() - 4186.01).abs() < 0.01);
    assert_eq!(Pitch::from_frequency(4186.01), c8);
}

#[test]
fn test_pitch_midi_conversion() {
    // Test standard MIDI notes
    assert_eq!(Pitch::from_midi_number(60), pitch!("C3"));
    assert_eq!(Pitch::from_midi_number(69), pitch!("A3"));
    assert_eq!(Pitch::from_midi_number(72), pitch!("C4"));

    // Test that MIDI conversion is consistent with existing midi_number method
    for midi in 0..=127 {
        let pitch = Pitch::from_midi_number(midi);
        assert_eq!(pitch.midi_number(), midi as i8, "MIDI conversion failed for note {}", midi);
    }

    // Test edge cases
    assert_eq!(Pitch::from_midi_number(0), pitch!("C-2"));
    assert_eq!(Pitch::from_midi_number(127), pitch!("G8"));
}

#[test]
fn test_pitch_midi_prefer_flats() {
    // Test black keys with flat preference
    assert_eq!(Pitch::from_midi_number_prefer_flats(61), pitch!("Db3")); // C#3 -> Db3
    assert_eq!(Pitch::from_midi_number_prefer_flats(63), pitch!("Eb3")); // D#3 -> Eb3
    assert_eq!(Pitch::from_midi_number_prefer_flats(66), pitch!("Gb3")); // F#3 -> Gb3
    assert_eq!(Pitch::from_midi_number_prefer_flats(68), pitch!("Ab3")); // G#3 -> Ab3
    assert_eq!(Pitch::from_midi_number_prefer_flats(70), pitch!("Bb3")); // A#3 -> Bb3

    // Test white keys (should be the same as default)
    assert_eq!(Pitch::from_midi_number_prefer_flats(60), pitch!("C3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(62), pitch!("D3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(64), pitch!("E3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(65), pitch!("F3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(67), pitch!("G3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(69), pitch!("A3"));
    assert_eq!(Pitch::from_midi_number_prefer_flats(71), pitch!("B3"));

    // Test that all MIDI conversions are enharmonically equivalent
    for midi in 0..=127 {
        let pitch_default = Pitch::from_midi_number(midi);
        let pitch_flat = Pitch::from_midi_number_prefer_flats(midi);
        assert!(pitch_default.is_enharmonic_with(&pitch_flat),
                "MIDI note {}: {} and {} are not enharmonic", midi, pitch_default, pitch_flat);
    }
}

#[test]
fn test_pitch_midi_in_key() {
    use chordy::Key;

    // Test sharp keys
    let g_major = Key::Major(chordy::note!("G")); // 1 sharp
    let d_major = Key::Major(chordy::note!("D")); // 2 sharps
    let a_major = Key::Major(chordy::note!("A")); // 3 sharps

    // In sharp keys, black keys should use sharps
    assert_eq!(Pitch::from_midi_number_in_key(61, &g_major), pitch!("C#3")); // C# not Db
    assert_eq!(Pitch::from_midi_number_in_key(66, &g_major), pitch!("F#3")); // F# not Gb
    assert_eq!(Pitch::from_midi_number_in_key(68, &g_major), pitch!("G#3")); // G# not Ab

    // Test flat keys
    let f_major = Key::Major(chordy::note!("F")); // 1 flat
    let bb_major = Key::Major(chordy::note!("Bb")); // 2 flats
    let eb_major = Key::Major(chordy::note!("Eb")); // 3 flats

    // In flat keys, black keys should use flats
    assert_eq!(Pitch::from_midi_number_in_key(61, &f_major), pitch!("Db3")); // Db not C#
    assert_eq!(Pitch::from_midi_number_in_key(66, &f_major), pitch!("Gb3")); // Gb not F#
    assert_eq!(Pitch::from_midi_number_in_key(70, &f_major), pitch!("Bb3")); // Bb not A#

    // Test natural keys (C major, A minor)
    let c_major = Key::Major(chordy::note!("C")); // 0 sharps/flats
    let a_minor = Key::Minor(chordy::note!("A")); // 0 sharps/flats

    // In natural keys, default to sharps (consistent with from_midi_number)
    assert_eq!(Pitch::from_midi_number_in_key(61, &c_major), pitch!("C#3"));
    assert_eq!(Pitch::from_midi_number_in_key(66, &c_major), pitch!("F#3"));
    assert_eq!(Pitch::from_midi_number_in_key(61, &a_minor), pitch!("C#3"));
    assert_eq!(Pitch::from_midi_number_in_key(66, &a_minor), pitch!("F#3"));

    // Test that all MIDI conversions are enharmonically equivalent
    for midi in 0..=127 {
        let pitch_default = Pitch::from_midi_number(midi);
        let pitch_in_key = Pitch::from_midi_number_in_key(midi, &c_major);
        assert!(pitch_default.is_enharmonic_with(&pitch_in_key),
                "MIDI note {}: {} and {} are not enharmonic", midi, pitch_default, pitch_in_key);
    }
}

#[test]
fn test_pitch_midi_enharmonic_equivalence() {
    // Test that all MIDI construction methods produce enharmonically equivalent results
    use chordy::Key;

    let c_major = Key::Major(chordy::note!("C"));

    for midi in 0..=127 {
        let pitch1 = Pitch::from_midi_number(midi);
        let pitch2 = Pitch::from_midi_number_prefer_flats(midi);
        let pitch3 = Pitch::from_midi_number_in_key(midi, &c_major);

        assert!(pitch1.is_enharmonic_with(&pitch2),
                "MIDI note {}: {} and {} are not enharmonic", midi, pitch1, pitch2);
        assert!(pitch1.is_enharmonic_with(&pitch3),
                "MIDI note {}: {} and {} are not enharmonic", midi, pitch1, pitch3);
        assert!(pitch2.is_enharmonic_with(&pitch3),
                "MIDI note {}: {} and {} are not enharmonic", midi, pitch2, pitch3);
    }
}

#[test]
fn test_pitch_constants() {
    assert_eq!(Pitch::A440, 440.0);
    assert_eq!(Pitch::C0_FREQUENCY, 16.3516);
}
