use chordy::types::{NoteName, Letter, Accidental};
use chordy::note;

#[test]
fn test_note_name_creation() {
    let note = NoteName::new(Letter::C, Accidental::Natural);
    assert_eq!(note.to_string(), "C");
}

#[test]
fn test_base_midi_number() {
    let note = NoteName::new(Letter::C, Accidental::Natural);
    assert_eq!(note.base_midi_number(), 0);
    
    let note = NoteName::new(Letter::A, Accidental::Flat);
    assert_eq!(note.base_midi_number(), 8); // A (9) + Flat (-1)
}

#[test]
fn test_enharmonic_equivalence() {
    let c = NoteName::new(Letter::C, Accidental::Natural);
    let b_sharp = NoteName::new(Letter::B, Accidental::Sharp);
    assert!(c.is_enharmonic_with(&b_sharp));

    assert!(note!("Cb").is_enharmonic_with(&note!("B")));
}
