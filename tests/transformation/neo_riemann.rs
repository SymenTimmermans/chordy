use chordy::{Chord, ChordQuality};
use chordy::note;

#[test]
fn test_p_transformations() {
    let c_major = Chord::new(note!("C"), ChordQuality::Major, vec![]);
    let c_minor = chordy::transformation::neo_riemann::transform_p(&c_major);
    assert_eq!(c_minor.notes(), vec![note!("C"), note!("Eb"), note!("G")]);

    let a_minor = Chord::new(note!("A"), ChordQuality::Minor, vec![]);
    let a_major = chordy::transformation::neo_riemann::transform_p(&a_minor);
    assert_eq!(a_major.notes(), vec![note!("A"), note!("C#"), note!("E")]);
}

#[test]
fn test_r_transformations() {
    let c_major = Chord::new(note!("C"), ChordQuality::Major, vec![]);
    let a_minor = chordy::transformation::neo_riemann::transform_r(&c_major);
    assert_eq!(a_minor.notes(), vec![note!("A"), note!("C"), note!("E")]);

    let f_minor = Chord::new(note!("F"), ChordQuality::Minor, vec![]);
    let ab_major = chordy::transformation::neo_riemann::transform_r(&f_minor);
    assert_eq!(ab_major.notes(), vec![note!("Ab"), note!("C"), note!("Eb")]);
}
