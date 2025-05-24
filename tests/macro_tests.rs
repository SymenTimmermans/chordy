use chordy::pitch;
use chordy::note;
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
fn test_pitch_macro_unicode_accidentals() {
    let p = pitch!("G♯4");
    assert_eq!(p, Pitch::new(Letter::G, Accidental::Sharp, 4));
    
    let p = pitch!("E♭7");
    assert_eq!(p, Pitch::new(Letter::E, Accidental::Flat, 7));
}

#[test]
fn test_pitch_macro_double_accidentals() {
    let p = pitch!("A##2");
    assert_eq!(p, Pitch::new(Letter::A, Accidental::DoubleSharp, 2));
    
    let p = pitch!("Bbb5");
    assert_eq!(p, Pitch::new(Letter::B, Accidental::DoubleFlat, 5));
}

#[test]
fn test_pitch_macro_double_unicode_accidentals() {
    let p = pitch!("C♯♯4");
    assert_eq!(p, Pitch::new(Letter::C, Accidental::DoubleSharp, 4));
    
    let p = pitch!("D♭♭7");
    assert_eq!(p, Pitch::new(Letter::D, Accidental::DoubleFlat, 7));
}

#[test]
fn test_pitch_macro_unicode_double_accidentals() {
    let p = pitch!("B𝄪4");
    assert_eq!(p, Pitch::new(Letter::B, Accidental::DoubleSharp, 4));
    
    let p = pitch!("F𝄫7");
    assert_eq!(p, Pitch::new(Letter::F, Accidental::DoubleFlat, 7));
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

#[test]
fn test_note_macro_basic() {
    let p = note!("C");
    assert_eq!(p, NoteName::new(Letter::C, Accidental::Natural));

    let p = note!("D");
    assert_eq!(p, NoteName::new(Letter::D, Accidental::Natural));

    // lowercase should be ok
    let p = note!("e");
    assert_eq!(p, NoteName::new(Letter::E, Accidental::Natural));

    let p = note!("b");
    assert_eq!(p, NoteName::new(Letter::B, Accidental::Natural));
}

#[test]
fn test_note_macro_accidentals() {
    let p = note!("A#");
    assert_eq!(p, NoteName::new(Letter::A, Accidental::Sharp));

    let p = note!("Bb");
    assert_eq!(p, NoteName::new(Letter::B, Accidental::Flat));
}

#[test]
fn test_note_macro_unicode_accidentals() {
    let p = note!("C♯");
    assert_eq!(p, NoteName::new(Letter::C, Accidental::Sharp));

    let p = note!("D♭");
    assert_eq!(p, NoteName::new(Letter::D, Accidental::Flat));
}

#[test]
fn test_note_macro_double_accidentals() {
    let p = note!("G##");
    assert_eq!(p, NoteName::new(Letter::G, Accidental::DoubleSharp));

    let p = note!("Ebb");
    assert_eq!(p, NoteName::new(Letter::E, Accidental::DoubleFlat));
}

#[test]
fn test_note_macro_double_unicode_accidentals() {
    let p = note!("C♯♯");
    assert_eq!(p, NoteName::new(Letter::C, Accidental::DoubleSharp));

    let p = note!("D♭♭");
    assert_eq!(p, NoteName::new(Letter::D, Accidental::DoubleFlat));
}

#[test]
fn test_note_macro_unicode_double_accidentals() {
    let p = note!("C𝄪");
    assert_eq!(p, NoteName::new(Letter::C, Accidental::DoubleSharp));

    let p = note!("D𝄫");
    assert_eq!(p, NoteName::new(Letter::D, Accidental::DoubleFlat));
}
