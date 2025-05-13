use chordy::types::*;

#[test]
fn test_pitch_creation() {
    let pitch = Pitch::new(NoteName::new(Letter::C, Accidental::Natural), 4);
    assert_eq!(pitch.to_string(), "C4");

    let pitch = Pitch::new(NoteName::new(Letter::A, Accidental::Sharp), 3);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(pitch.to_string(), "A♯3");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "A#3");

    let pitch = Pitch::new(NoteName::new(Letter::B, Accidental::Flat), 5);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(pitch.to_string(), "B♭5");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "Bb5");

}

#[test]
fn test_midi_number() {
    let pitch = Pitch::new(NoteName::new(Letter::C, Accidental::Natural), 3);
    assert_eq!(pitch.midi_number(), 60);

    let pitch = Pitch::new(NoteName::new(Letter::G, Accidental::Sharp), 5);
    assert_eq!(pitch.midi_number(), 92);
}

#[test]
fn test_pitch_enharmonic() {
    let p1 = Pitch::new(NoteName::new(Letter::C, Accidental::Natural), 4);
    let p2 = Pitch::new(NoteName::new(Letter::B, Accidental::Sharp), 3);

    assert!(p1.is_enharmonic_with(&p2));
}

#[test]
fn test_pitch_transpose() {
    let pitch = Pitch::new(NoteName::new(Letter::C, Accidental::Natural), 4);
    let transposed = pitch.transpose(2);
    let expected = Pitch::new(NoteName::new(Letter::D, Accidental::Natural), 4);

    assert_eq!(transposed, expected);
}
