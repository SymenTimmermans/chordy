use chordy::pitch;
use chordy::types::*;

#[test]
fn test_pitch_macro_basic() {
    let p = pitch!("C4");
    assert_eq!(p, Pitch::new(Letter::C, Accidental::Natural, 4));
}

#[test]
fn test_pitch_macro_accidentals() {
    let p = pitch!("A#3");
    assert_eq!(p, Pitch::new(Letter::A, Accidental::Sharp, 3));
    
    let p = pitch!("Bb4");
    assert_eq!(p, Pitch::new(Letter::B, Accidental::Flat, 4));
}

#[test]
fn test_pitch_macro_negative_octave() {
    let p = pitch!("C-2");
    assert_eq!(p, Pitch::new(Letter::C, Accidental::Natural, -2));
}

#[test]
#[should_panic]
fn test_pitch_macro_invalid() {
    let _ = pitch!("H4"); // Should panic on invalid note
}
