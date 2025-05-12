use chordy::types::{NoteName, Letter, Accidental};

#[test]
fn test_note_display() {
    let note = NoteName::new(Letter::F, Accidental::Sharp);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(note.to_string(), "F♯");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "F#");


    let note = NoteName::new(Letter::B, Accidental::Flat);
    #[cfg(feature = "utf8_symbols")]
    assert_eq!(note.to_string(), "B♭");
    #[cfg(not(feature = "utf8_symbols"))]
    assert_eq!(note.to_string(), "Bb");

    let note = NoteName::new(Letter::C, Accidental::Natural);
    assert_eq!(note.to_string(), "C");
}

#[test]
fn test_accidental_display() {
    assert_eq!(Accidental::Flat.to_string(), "♭");
    assert_eq!(Accidental::DoubleSharp.to_string(), "𝄪");
}
