use chordy::types::*;
use chordy::pitch;

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
fn test_pitch_from_str() {
    // Test basic cases
    assert_eq!("C4".parse(), Ok(Pitch::new(Letter::C, Accidental::Natural, 4)));
    assert_eq!("D5".parse(), Ok(Pitch::new(Letter::D, Accidental::Natural, 5)));
    
    // Test negative octaves
    assert_eq!("C-2".parse(), Ok(Pitch::new(Letter::C, Accidental::Natural, -2)));
    assert_eq!("F#-1".parse(), Ok(Pitch::new(Letter::F, Accidental::Sharp, -1)));
    
    // Test accidentals
    assert_eq!("Ab3".parse(), Ok(Pitch::new(Letter::A, Accidental::Flat, 3)));
    assert_eq!("G#4".parse(), Ok(Pitch::new(Letter::G, Accidental::Sharp, 4)));
    assert_eq!("Bbb5".parse(), Ok(Pitch::new(Letter::B, Accidental::DoubleFlat, 5)));
    
    // Test invalid cases
    assert!("".parse::<Pitch>().is_err());
    assert!("C".parse::<Pitch>().is_err());
    assert!("4".parse::<Pitch>().is_err());
    assert!("H4".parse::<Pitch>().is_err()); // Invalid letter
    assert!("C#".parse::<Pitch>().is_err()); // Missing octave
    assert!("C4.5".parse::<Pitch>().is_err()); // Invalid octave
}

#[test]
fn test_pitch_transpose() {
    // Basic single semitone transpositions
    assert_eq!(pitch!("C4").transpose(1), pitch!("C#4"));  // C→C#
    assert_eq!(pitch!("C#4").transpose(-1), pitch!("C4")); // C#→C
    assert_eq!(pitch!("D4").transpose(1), pitch!("D#4"));  // D→D#
    assert_eq!(pitch!("Eb4").transpose(-1), pitch!("D4")); // Eb→D

    // Whole step transpositions
    assert_eq!(pitch!("C4").transpose(2), pitch!("D4"));   // C→D
    assert_eq!(pitch!("D4").transpose(-2), pitch!("C4"));  // D→C

    // Enharmonic spelling special cases
    assert_eq!(pitch!("B4").transpose(1), pitch!("C5"));   // B→C not B#
    assert_eq!(pitch!("C4").transpose(-1), pitch!("B3"));  // C→B not Cb
    assert_eq!(pitch!("E4").transpose(1), pitch!("F4"));   // E→F not E#
    assert_eq!(pitch!("F4").transpose(-1), pitch!("E4"));  // F→E not Fb

    // Larger intervals
    assert_eq!(pitch!("C4").transpose(12), pitch!("C5"));  // Octave up
    assert_eq!(pitch!("C5").transpose(-12), pitch!("C4")); // Octave down
    assert_eq!(pitch!("G4").transpose(7), pitch!("D5"));   // Perfect 5th up
    assert_eq!(pitch!("D5").transpose(-7), pitch!("G4"));  // Perfect 5th down

    // Negative octaves
    assert_eq!(pitch!("C-2").transpose(12), pitch!("C-1"));
    assert_eq!(pitch!("C-1").transpose(-12), pitch!("C-2"));

    // Double accidentals
    assert_eq!(pitch!("Cbb4").transpose(2), pitch!("Dbb4")); 
    assert_eq!(pitch!("D##4").transpose(-2), pitch!("C##4"));

    // Chromatic scale tests
    let chromatic_up = [
        pitch!("C4"), pitch!("C#4"), pitch!("D4"), pitch!("D#4"),
        pitch!("E4"), pitch!("F4"), pitch!("F#4"), pitch!("G4"),
        pitch!("G#4"), pitch!("A4"), pitch!("A#4"), pitch!("B4"),
        pitch!("C5")
    ];
    
    let chromatic_down = [
        pitch!("C5"), pitch!("B4"), pitch!("Bb4"), pitch!("A4"),
        pitch!("Ab4"), pitch!("G4"), pitch!("Gb4"), pitch!("F4"),
        pitch!("E4"), pitch!("Eb4"), pitch!("D4"), pitch!("Db4"),
        pitch!("C4")
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
    assert_eq!(pitch!("C4").transpose(24), pitch!("C6"));  // 2 octaves up
    assert_eq!(pitch!("C4").transpose(-24), pitch!("C2")); // 2 octaves down

    // Edge cases with accidentals
    assert_eq!(pitch!("F#4").transpose(1), pitch!("G4"));  // F#→G not F##
    assert_eq!(pitch!("Gb4").transpose(-1), pitch!("F4")); // Gb→F not Gbb
    assert_eq!(pitch!("B#3").transpose(1), pitch!("C#4")); // B#→C#
    assert_eq!(pitch!("Cb4").transpose(-1), pitch!("Bb3")); // Cb→Bb
    //
    assert_eq!(pitch!("C4").transpose(0), pitch!("C4"));  // No change
    //
    assert_eq!(pitch!("G#4").transpose(-2), pitch!("F#4"));  // C→C#
}
