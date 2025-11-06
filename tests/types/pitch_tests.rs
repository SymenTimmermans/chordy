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

#[test]
fn test_pitch_cents_from() {
    // Test same pitch (0 cents)
    let a440 = Pitch::new(Letter::A, Accidental::Natural, 4);
    assert!((a440.cents_from(&a440)).abs() < 0.01);

    // Test octave (1200 cents)
    let a3 = Pitch::new(Letter::A, Accidental::Natural, 3);
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);
    assert!((a4.cents_from(&a3) - 1200.0).abs() < 0.01);

    // Test semitone (100 cents)
    let c4 = Pitch::new(Letter::C, Accidental::Natural, 4);
    let c_sharp4 = Pitch::new(Letter::C, Accidental::Sharp, 4);
    assert!((c_sharp4.cents_from(&c4) - 100.0).abs() < 0.01);

    // Test microtonal differences
    // Since Pitch only represents equal-tempered pitches, we need to test with actual different pitches
    // and calculate the expected cents difference between them
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);
    let a_sharp4 = Pitch::new(Letter::A, Accidental::Sharp, 4);

    // A#4 should be 100 cents above A4
    let cents_diff = a_sharp4.cents_from(&a4);
    assert!((cents_diff - 100.0).abs() < 0.01);

    // Test negative cents
    let g_sharp4 = Pitch::new(Letter::G, Accidental::Sharp, 4);
    let cents_diff = g_sharp4.cents_from(&a4);
    assert!((cents_diff + 100.0).abs() < 0.01); // G#4 is 100 cents below A4

    // Test enharmonic equivalence
    let c_sharp4 = Pitch::new(Letter::C, Accidental::Sharp, 4);
    let d_flat4 = Pitch::new(Letter::D, Accidental::Flat, 4);
    assert!((c_sharp4.cents_from(&d_flat4)).abs() < 0.01);
}

#[test]
fn test_pitch_transpose_cents() {
    let a440 = Pitch::new(Letter::A, Accidental::Natural, 4);

    // Test zero cents (no change)
    assert_eq!(a440.transpose_cents(0.0), a440);

    // Test semitone (100 cents)
    let a_sharp4 = a440.transpose_cents(100.0);
    assert_eq!(a_sharp4, Pitch::new(Letter::A, Accidental::Sharp, 4));

    // Test octave (1200 cents)
    let a5 = a440.transpose_cents(1200.0);
    assert_eq!(a5, Pitch::new(Letter::A, Accidental::Natural, 5));

    // Test negative cents
    let g_sharp4 = a440.transpose_cents(-100.0);
    assert_eq!(g_sharp4, Pitch::new(Letter::G, Accidental::Sharp, 4));

    // Test microtonal transposition (should snap to nearest equal-tempered pitch)
    let slightly_sharp_a = a440.transpose_cents(15.0);
    // Should still be A4 since 15 cents is closer to A4 than A#4
    assert_eq!(slightly_sharp_a, a440);

    let slightly_flat_a_sharp = a440.transpose_cents(85.0);
    // Should be A#4 since 85 cents is closer to A#4 than A4
    assert_eq!(slightly_flat_a_sharp, Pitch::new(Letter::A, Accidental::Sharp, 4));
}

#[test]
fn test_interval_cents() {
    // Test basic intervals
    assert_eq!(Interval::PERFECT_UNISON.cents(), 0.0);
    assert_eq!(Interval::MINOR_SECOND.cents(), 100.0);
    assert_eq!(Interval::MAJOR_SECOND.cents(), 200.0);
    assert_eq!(Interval::MINOR_THIRD.cents(), 300.0);
    assert_eq!(Interval::MAJOR_THIRD.cents(), 400.0);
    assert_eq!(Interval::PERFECT_FOURTH.cents(), 500.0);
    assert_eq!(Interval::AUGMENTED_FOURTH.cents(), 600.0);
    assert_eq!(Interval::PERFECT_FIFTH.cents(), 700.0);
    assert_eq!(Interval::MINOR_SIXTH.cents(), 800.0);
    assert_eq!(Interval::MAJOR_SIXTH.cents(), 900.0);
    assert_eq!(Interval::MINOR_SEVENTH.cents(), 1000.0);
    assert_eq!(Interval::MAJOR_SEVENTH.cents(), 1100.0);
    assert_eq!(Interval::OCTAVE.cents(), 1200.0);

    // Test compound intervals
    assert_eq!(Interval::MAJOR_NINTH.cents(), 1400.0);
    assert_eq!(Interval::PERFECT_ELEVENTH.cents(), 1700.0);
    assert_eq!(Interval::MAJOR_THIRTEENTH.cents(), 2100.0); // Octave + major 6th = 12 + 9 = 21 semitones

    // Note: Extreme intervals like DOUBLY_DIMINISHED_UNISON and DOUBLY_AUGMENTED_UNISON
    // may not work correctly with the current interval system as it's designed for
    // practical music intervals. These theoretical intervals are not commonly used
    // in standard music practice.
}

#[test]
fn test_pitch_harmonic() {
    // Test C2 fundamental
    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);

    // 1st harmonic = fundamental
    assert_eq!(c2.harmonic(1), c2);

    // 2nd harmonic = octave above
    assert_eq!(c2.harmonic(2), pitch!("C3"));

    // 3rd harmonic = perfect fifth above (G3)
    assert_eq!(c2.harmonic(3), pitch!("G3"));

    // 4th harmonic = two octaves above
    assert_eq!(c2.harmonic(4), pitch!("C4"));

    // 5th harmonic = major third above (E4)
    assert_eq!(c2.harmonic(5), pitch!("E4"));

    // 6th harmonic = perfect fifth above (G4)
    assert_eq!(c2.harmonic(6), pitch!("G4"));

    // 7th harmonic = minor seventh above (Bb4/A#4)
    let seventh_harmonic = c2.harmonic(7);
    assert!(seventh_harmonic.is_enharmonic_with(&pitch!("Bb4")));

    // 8th harmonic = three octaves above
    assert_eq!(c2.harmonic(8), pitch!("C5"));

    // Test A4 fundamental
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);

    // 2nd harmonic = octave above
    assert_eq!(a4.harmonic(2), pitch!("A5"));

    // 3rd harmonic = perfect fifth above (E6)
    assert_eq!(a4.harmonic(3), pitch!("E6"));
}

#[test]
fn test_pitch_harmonics() {
    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);

    // Test harmonics up to 8
    let harmonics = c2.harmonics(8);
    assert_eq!(harmonics.len(), 8);

    // Verify the harmonic series: [C2, C3, G3, C4, E4, G4, Bb4/A#4, C5]
    assert_eq!(harmonics[0], pitch!("C2")); // 1st harmonic
    assert_eq!(harmonics[1], pitch!("C3")); // 2nd harmonic
    assert_eq!(harmonics[2], pitch!("G3")); // 3rd harmonic
    assert_eq!(harmonics[3], pitch!("C4")); // 4th harmonic
    assert_eq!(harmonics[4], pitch!("E4")); // 5th harmonic
    assert_eq!(harmonics[5], pitch!("G4")); // 6th harmonic
    assert!(harmonics[6].is_enharmonic_with(&pitch!("Bb4"))); // 7th harmonic
    assert_eq!(harmonics[7], pitch!("C5")); // 8th harmonic

    // Test empty harmonics
    let empty = c2.harmonics(0);
    assert!(empty.is_empty());

    // Test single harmonic
    let single = c2.harmonics(1);
    assert_eq!(single.len(), 1);
    assert_eq!(single[0], c2);
}

#[test]
fn test_pitch_subharmonic() {
    // Test C4 fundamental
    let c4 = Pitch::new(Letter::C, Accidental::Natural, 4);

    // 1st subharmonic = fundamental
    assert_eq!(c4.subharmonic(1), c4);

    // 2nd subharmonic = octave below
    assert_eq!(c4.subharmonic(2), pitch!("C3"));

    // 3rd subharmonic = major third below (F2)
    assert_eq!(c4.subharmonic(3), pitch!("F2"));

    // 4th subharmonic = two octaves below
    assert_eq!(c4.subharmonic(4), pitch!("C2"));

    // 5th subharmonic = major third below (G#1)
    assert_eq!(c4.subharmonic(5), pitch!("G#1"));

    // Test A4 fundamental
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);

    // 2nd subharmonic = octave below
    assert_eq!(a4.subharmonic(2), pitch!("A3"));

    // 3rd subharmonic = perfect fifth below (D3)
    assert_eq!(a4.subharmonic(3), pitch!("D3"));
}

#[test]
fn test_pitch_fundamental_of_harmonic() {
    // Test finding fundamental from G3 (3rd harmonic of C2)
    let g3 = Pitch::new(Letter::G, Accidental::Natural, 3);
    let fundamental = g3.fundamental_of_harmonic(3);
    assert_eq!(fundamental, pitch!("C2"));

    // Test finding fundamental from C5 (8th harmonic of C2)
    let c5 = Pitch::new(Letter::C, Accidental::Natural, 5);
    let fundamental = c5.fundamental_of_harmonic(8);
    assert_eq!(fundamental, pitch!("C2"));

    // Test finding fundamental from E4 (5th harmonic of C2)
    let e4 = Pitch::new(Letter::E, Accidental::Natural, 4);
    let fundamental = e4.fundamental_of_harmonic(5);
    assert_eq!(fundamental, pitch!("C2"));

    // Test finding fundamental from Bb4 (7th harmonic of C2)
    let bb4 = Pitch::new(Letter::B, Accidental::Flat, 4);
    let fundamental = bb4.fundamental_of_harmonic(7);
    assert_eq!(fundamental, pitch!("C2"));

    // Test finding fundamental from A5 (2nd harmonic of A4)
    let a5 = Pitch::new(Letter::A, Accidental::Natural, 5);
    let fundamental = a5.fundamental_of_harmonic(2);
    assert_eq!(fundamental, pitch!("A4"));

    // Test finding fundamental from E6 (3rd harmonic of A4)
    let e6 = Pitch::new(Letter::E, Accidental::Natural, 6);
    let fundamental = e6.fundamental_of_harmonic(3);
    assert_eq!(fundamental, pitch!("A4"));
}

#[test]
#[should_panic(expected = "Harmonic number must be positive")]
fn test_pitch_harmonic_zero_panics() {
    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);
    c2.harmonic(0);
}

#[test]
#[should_panic(expected = "Subharmonic number must be positive")]
fn test_pitch_subharmonic_zero_panics() {
    let c4 = Pitch::new(Letter::C, Accidental::Natural, 4);
    c4.subharmonic(0);
}

#[test]
#[should_panic(expected = "Harmonic number must be positive")]
fn test_pitch_fundamental_of_harmonic_zero_panics() {
    let g3 = Pitch::new(Letter::G, Accidental::Natural, 3);
    g3.fundamental_of_harmonic(0);
}

#[test]
fn test_harmonic_series_relationship() {
    // Test that harmonic and fundamental_of_harmonic are inverse operations
    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);

    for n in 1..=8 {
        let harmonic = c2.harmonic(n);
        let recovered_fundamental = harmonic.fundamental_of_harmonic(n);

        // The recovered fundamental should be enharmonically equivalent to the original
        assert!(recovered_fundamental.is_enharmonic_with(&c2),
                "Failed for harmonic {}: {} -> {} -> {}", n, c2, harmonic, recovered_fundamental);
    }
}
