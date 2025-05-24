use chordy::{Chord, ChordQuality};
use chordy::note;

#[test]
fn test_chord_notes() {
    // Basic triads
    let c_major = Chord::new(note!("C"), ChordQuality::Major, vec![]);
    assert_eq!(c_major.notes(), vec![note!("C"), note!("E"), note!("G")]);
    
    let d_minor = Chord::new(note!("D"), ChordQuality::Minor, vec![]);
    assert_eq!(d_minor.notes(), vec![note!("D"), note!("F"), note!("A")]);
    
    // Challenging spellings
    let f_sharp_major = Chord::new(note!("F#"), ChordQuality::Major, vec![]);
    assert_eq!(f_sharp_major.notes(), vec![note!("F#"), note!("A#"), note!("C#")]);
    
    let g_flat_major = Chord::new(note!("Gb"), ChordQuality::Major, vec![]);
    assert_eq!(g_flat_major.notes(), vec![note!("Gb"), note!("Bb"), note!("Db")]);
    
    // Tricky augmented/diminished cases
    let c_sharp_diminished = Chord::new(note!("C#"), ChordQuality::Diminished, vec![]);
    assert_eq!(c_sharp_diminished.notes(), vec![note!("C#"), note!("E"), note!("G")]);
    
    let d_flat_augmented = Chord::new(note!("Db"), ChordQuality::Augmented, vec![]);
    assert_eq!(d_flat_augmented.notes(), vec![note!("Db"), note!("F"), note!("A")]);
}
