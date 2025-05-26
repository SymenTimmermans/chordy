use chordy::{note, Chord, Interval};

#[test]
fn test_chord_notes() {
    // Basic triads
    let c_major = Chord::major(note!("C"));
    assert_eq!(c_major.notes(), vec![note!("C"), note!("E"), note!("G")]);
    
    let d_minor = Chord::minor(note!("D"));
    assert_eq!(d_minor.notes(), vec![note!("D"), note!("F"), note!("A")]);
    
    // Challenging spellings
    let f_sharp_major = Chord::major(note!("F#"));
    assert_eq!(f_sharp_major.notes(), vec![note!("F#"), note!("A#"), note!("C#")]);
    
    let g_flat_major = Chord::major(note!("Gb"));
    assert_eq!(g_flat_major.notes(), vec![note!("Gb"), note!("Bb"), note!("Db")]);
    
    // Tricky augmented/diminished cases
    let c_sharp_diminished = Chord::diminished(note!("C#"));
    assert_eq!(c_sharp_diminished.notes(), vec![note!("C#"), note!("E"), note!("G")]);
    
    let d_flat_augmented = Chord::augmented(note!("Db"));
    assert_eq!(d_flat_augmented.notes(), vec![note!("Db"), note!("F"), note!("A")]);
    
    // Test custom interval chords
    let custom = Chord::new(note!("C"), vec![
        Interval::MAJOR_SECOND,
        Interval::PERFECT_FOURTH,
        Interval::MAJOR_SIXTH,
    ]);
    assert_eq!(custom.notes(), vec![note!("D"), note!("F"), note!("A")]);
}
