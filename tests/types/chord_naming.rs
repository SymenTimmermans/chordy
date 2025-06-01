use chordy::prelude::*;

#[test]
fn test_chord_naming_basic_triads() {
    let c_major = Chord::from_notes(&[note!("C"), note!("E"), note!("G")]);
    assert_eq!(c_major.abbreviated_name(), "C");

    let d_minor = Chord::from_notes(&[note!("D"), note!("F"), note!("A")]);
    assert_eq!(d_minor.abbreviated_name(), "Dm");
}
