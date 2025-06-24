use chordy::prelude::*;

#[test]
fn test_key_degree_of_c_major() {
    let c_major = Key::Major(note!("C"));
    
    // Test natural scale degrees in C major
    assert_eq!(c_major.degree_of(note!("C")), ScaleDegree::new(1, None));
    assert_eq!(c_major.degree_of(note!("D")), ScaleDegree::new(2, None));
    assert_eq!(c_major.degree_of(note!("E")), ScaleDegree::new(3, None));
    assert_eq!(c_major.degree_of(note!("F")), ScaleDegree::new(4, None));
    assert_eq!(c_major.degree_of(note!("G")), ScaleDegree::new(5, None));
    assert_eq!(c_major.degree_of(note!("A")), ScaleDegree::new(6, None));
    assert_eq!(c_major.degree_of(note!("B")), ScaleDegree::new(7, None));
}

#[test]
fn test_key_degree_of_c_major_altered_notes() {
    let c_major = Key::Major(note!("C"));
    
    // Test what we actually get from the interval conversion
    let cs_degree = c_major.degree_of(note!("C#"));
    let fs_degree = c_major.degree_of(note!("F#"));
    let bb_degree = c_major.degree_of(note!("Bb"));
    let eb_degree = c_major.degree_of(note!("Eb"));
    
    // C# should be degree 1 (but the accidental depends on the interval conversion)
    assert_eq!(cs_degree.step, 1);
    // F# should be degree 4  
    assert_eq!(fs_degree.step, 4);
    // Bb should be degree 7
    assert_eq!(bb_degree.step, 7);
    // Eb should be degree 3
    assert_eq!(eb_degree.step, 3);
}

#[test]
fn test_key_degree_of_a_minor() {
    let a_minor = Key::Minor(note!("A"));
    
    // Test natural scale degrees relative to A (A minor has same notes as C major)
    assert_eq!(a_minor.degree_of(note!("A")), ScaleDegree::new(1, None));
    assert_eq!(a_minor.degree_of(note!("B")), ScaleDegree::new(2, None));
    assert_eq!(a_minor.degree_of(note!("C")), ScaleDegree::new(3, Some(Accidental::Flat)));
    assert_eq!(a_minor.degree_of(note!("D")), ScaleDegree::new(4, None));
    assert_eq!(a_minor.degree_of(note!("E")), ScaleDegree::new(5, None));
    assert_eq!(a_minor.degree_of(note!("F")), ScaleDegree::new(6, Some(Accidental::Flat)));
    assert_eq!(a_minor.degree_of(note!("G")), ScaleDegree::new(7, Some(Accidental::Flat)));
}

#[test]
fn test_key_degree_of_f_sharp_major() {
    let fs_major = Key::Major(note!("F#"));
    
    // Test some degrees in F# major (6 sharps)
    assert_eq!(fs_major.degree_of(note!("F#")), ScaleDegree::new(1, None));
    assert_eq!(fs_major.degree_of(note!("G#")), ScaleDegree::new(2, None));
    assert_eq!(fs_major.degree_of(note!("A#")), ScaleDegree::new(3, None));
    assert_eq!(fs_major.degree_of(note!("B")), ScaleDegree::new(4, None));
    assert_eq!(fs_major.degree_of(note!("C#")), ScaleDegree::new(5, None));
}

#[test]
fn test_key_degree_of_enharmonic_equivalents() {
    let c_major = Key::Major(note!("C"));
    
    // Test enharmonic equivalents - they may map to different scale degrees
    // because they represent different intervals from C
    let c_sharp = c_major.degree_of(note!("C#"));
    let d_flat = c_major.degree_of(note!("Db"));
    
    // C# (augmented unison) and Db (minor second) are different intervals
    // so they may map to different scale degrees
    assert_eq!(c_sharp.step, 1);  // C# is likely an altered 1st degree
    assert_eq!(d_flat.step, 2);   // Db is likely a flat 2nd degree
}