use chordy::prelude::*;

#[test]
fn test_chord_naming_basic_triads() {
    let c_major = Chord::from_notes(&[note!("C"), note!("E"), note!("G")]);
    assert_eq!(c_major.abbreviated_name(), "C");

    let d_minor = Chord::from_notes(&[note!("D"), note!("F"), note!("A")]);
    assert_eq!(d_minor.abbreviated_name(), "Dm");

    let e_flat_aug = Chord::from_notes(&[note!("Eb"), note!("G"), note!("B")]);
    assert_eq!(e_flat_aug.abbreviated_name(), "E♭aug");
}

#[test]
fn test_chord_naming_seventh_chords() {
    let g7 = Chord::from_notes(&[note!("G"), note!("B"), note!("D"), note!("F")]);
    assert_eq!(g7.abbreviated_name(), "G7");

    let a_minor_7 = Chord::from_notes(&[note!("A"), note!("C"), note!("E"), note!("G")]);
    assert_eq!(a_minor_7.abbreviated_name(), "Am7");

    let c_major_7 = Chord::from_notes(&[note!("C"), note!("E"), note!("G"), note!("B")]);
    assert_eq!(c_major_7.abbreviated_name(), "Cmaj7");
}
