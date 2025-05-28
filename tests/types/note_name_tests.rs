use chordy::types::{Accidental, Letter, NoteName};
use chordy::{note, Interval};

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

#[test]
fn test_interval_group_properties() {
    let major_third = Interval::MAJOR_THIRD;
    let minor_third = Interval::MINOR_THIRD;
    let perfect_fifth = Interval::PERFECT_FIFTH;

    // Closure: intervals can be added
    assert_eq!(major_third + minor_third, perfect_fifth);

    // Identity: unison is the identity
    assert_eq!(major_third + Interval::PERFECT_UNISON, major_third);

    // Inverse: every interval has an inverse
    assert_eq!(major_third + (-major_third), Interval::PERFECT_UNISON);

    // Associativity
    let fourth = Interval::PERFECT_FOURTH;
    assert_eq!(
        (major_third + minor_third) + fourth,
        major_third + (minor_third + fourth)
    );
}

#[test]
fn test_torsor_properties() {
    let c = note!("C");
    let e = note!("E");
    let g = note!("G");
    let major_third = Interval::MAJOR_THIRD;

    // Action: note + interval = note
    assert_eq!(c + major_third, e);

    // Difference: note - note = interval
    assert_eq!(e - c, major_third);

    // Torsor property: (p + v) - p = v
    assert_eq!((c + major_third) - c, major_third);

    // Torsor property: p + (q - p) = q
    assert_eq!(c + (g - c), g);
}

#[test]
fn test_transpose_chain() {
    let c = note!("C");

    // Chain of transpositions
    let e = c.transpose(Interval::MAJOR_THIRD);
    let g = e.transpose(Interval::MINOR_THIRD);
    let c_octave = g.transpose(Interval::PERFECT_FOURTH);

    // Should equal direct transposition by octave
    let octave = Interval::MAJOR_THIRD + Interval::MINOR_THIRD + Interval::PERFECT_FOURTH;
    assert_eq!(c.transpose(octave), c_octave);
}

#[test]
fn test_interval_between_notes() {
    let c = note!("C");
    let g = note!("G");

    // The interval from C to G should be a perfect fifth
    assert_eq!(c.interval_to(g), Interval::PERFECT_FIFTH);

    // The interval from G to C should be a perfect fourth (down)
    assert_eq!(g.interval_to(c), -Interval::PERFECT_FIFTH);
}
