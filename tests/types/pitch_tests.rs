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
    let pitch = Pitch::new(Letter::C, Accidental::Natural, 4);
    let transposed = pitch.transpose(2);
    assert_eq!(transposed, Pitch::new(Letter::D, Accidental::Natural, 4));

    let pitch = Pitch::new(Letter::A, Accidental::Sharp, 3);
    let transposed = pitch.transpose(1);
    assert_eq!(transposed, Pitch::new(Letter::B, Accidental::Natural, 3));

    // B up 1 semitone should be C natural, not B#                                                                                                                                                                   
    let b = Pitch::new(Letter::B, Accidental::Natural, 4);                                                                                                                                                           
    assert_eq!(b.transpose(1), Pitch::new(Letter::C, Accidental::Natural, 5));                                                                                                                                       
                                                                                                                                                                                                                     
    // C down 1 semitone should be B natural, not Cb                                                                                                                                                                 
    let c = Pitch::new(Letter::C, Accidental::Natural, 4);                                                                                                                                                           
    // assert_eq!(c.transpose(-1), Pitch::new(Letter::B, Accidental::Natural, 3));                                                                                                                                      
                                                                                                                                                                                                                     
    // F# up 1 semitone should be G natural, not F##                                                                                                                                                                 
    let f_sharp = Pitch::new(Letter::F, Accidental::Sharp, 4);                                                                                                                                                       
    assert_eq!(f_sharp.transpose(1), Pitch::new(Letter::G, Accidental::Natural, 4));
}
