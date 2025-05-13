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
    // Whole step transposition
    assert_eq!(pitch!("C4").transpose(2), pitch!("D4"));
    
    // Half step transposition
    assert_eq!(pitch!("A#3").transpose(1), pitch!("B3"));
    
    // Special cases for proper enharmonic spelling
    assert_eq!(pitch!("B4").transpose(1), pitch!("C5"));  // B→C not B#
    // assert_eq!(pitch!("C4").transpose(-1), pitch!("B3")); // C→B not Cb
    assert_eq!(pitch!("F#4").transpose(1), pitch!("G4")); // F#→G not F##
    
    // Larger intervals
    assert_eq!(pitch!("C4").transpose(12), pitch!("C5")); // Octave up
    assert_eq!(pitch!("E5").transpose(-12), pitch!("E4")); // Octave down
    
    // With accidentals
    assert_eq!(pitch!("Bb4").transpose(2), pitch!("C5"));
    assert_eq!(pitch!("D#4").transpose(-1), pitch!("D4"));
    
    // Negative octaves
    let p1 = pitch!("C-2").transpose(12);
    let p2 = pitch!("C-1");
    assert_eq!(p1, p2);
}
