use chordy::Interval;

#[test]
fn test_parse_basic_intervals() {
    assert_eq!("P1".parse::<Interval>().unwrap(), Interval::PERFECT_UNISON);
    assert_eq!("m2".parse::<Interval>().unwrap(), Interval::MINOR_SECOND);
    assert_eq!("M2".parse::<Interval>().unwrap(), Interval::MAJOR_SECOND);
    assert_eq!("m3".parse::<Interval>().unwrap(), Interval::MINOR_THIRD);
    assert_eq!("M3".parse::<Interval>().unwrap(), Interval::MAJOR_THIRD);
    assert_eq!("P4".parse::<Interval>().unwrap(), Interval::PERFECT_FOURTH);
    assert_eq!("P5".parse::<Interval>().unwrap(), Interval::PERFECT_FIFTH);
    assert_eq!("m6".parse::<Interval>().unwrap(), Interval::MINOR_SIXTH);
    assert_eq!("M6".parse::<Interval>().unwrap(), Interval::MAJOR_SIXTH);
    assert_eq!("m7".parse::<Interval>().unwrap(), Interval::MINOR_SEVENTH);
    assert_eq!("M7".parse::<Interval>().unwrap(), Interval::MAJOR_SEVENTH);
    assert_eq!("P8".parse::<Interval>().unwrap(), Interval::OCTAVE);
}

#[test]
fn test_parse_compound_intervals() {
    assert_eq!("M9".parse::<Interval>().unwrap(), Interval::MAJOR_NINTH);
    assert_eq!(
        "P12".parse::<Interval>().unwrap(),
        Interval::PERFECT_TWELFTH
    );
    assert_eq!(
        "M13".parse::<Interval>().unwrap(),
        Interval::MAJOR_THIRTEENTH
    );
}

#[test]
fn test_parse_augmented_diminished() {
    assert_eq!(
        "A4".parse::<Interval>().unwrap(),
        Interval::AUGMENTED_FOURTH
    );
    assert_eq!(
        "d5".parse::<Interval>().unwrap(),
        Interval::DIMINISHED_FIFTH
    );
    assert_eq!(
        "A2".parse::<Interval>().unwrap(),
        Interval::AUGMENTED_SECOND
    );
    assert_eq!(
        "d7".parse::<Interval>().unwrap(),
        Interval::DIMINISHED_SEVENTH
    );
}

#[test]
fn test_parse_doubly_augmented_diminished() {
    assert_eq!(
        "dd1".parse::<Interval>().unwrap(),
        Interval::DOUBLY_DIMINISHED_UNISON
    );
    assert_eq!(
        "AA1".parse::<Interval>().unwrap(),
        Interval::DOUBLY_AUGMENTED_UNISON
    );
    assert_eq!(
        "dd5".parse::<Interval>().unwrap(),
        Interval::DOUBLY_DIMINISHED_FIFTH
    );
    assert_eq!(
        "AA5".parse::<Interval>().unwrap(),
        Interval::DOUBLY_AUGMENTED_FIFTH
    );
    assert_eq!(
        "dd7".parse::<Interval>().unwrap(),
        Interval::DOUBLY_DIMINISHED_SEVENTH
    );
    assert_eq!(
        "AA7".parse::<Interval>().unwrap(),
        Interval::DOUBLY_AUGMENTED_SEVENTH
    );
}

#[test]
fn test_interval_classification_with_double_accidentals() {
    // Test is_fifth()
    assert!(Interval::DOUBLY_DIMINISHED_FIFTH.is_fifth());
    assert!(Interval::DIMINISHED_FIFTH.is_fifth());
    assert!(Interval::PERFECT_FIFTH.is_fifth());
    assert!(Interval::AUGMENTED_FIFTH.is_fifth());
    assert!(Interval::DOUBLY_AUGMENTED_FIFTH.is_fifth());

    // Test is_third()
    assert!(Interval::DOUBLY_DIMINISHED_THIRD.is_third());
    assert!(Interval::DIMINISHED_THIRD.is_third());
    assert!(Interval::MINOR_THIRD.is_third());
    assert!(Interval::MAJOR_THIRD.is_third());
    assert!(Interval::AUGMENTED_THIRD.is_third());
    assert!(Interval::DOUBLY_AUGMENTED_THIRD.is_third());

    // Test is_seventh()
    assert!(Interval::DOUBLY_DIMINISHED_SEVENTH.is_seventh());
    assert!(Interval::DIMINISHED_SEVENTH.is_seventh());
    assert!(Interval::MINOR_SEVENTH.is_seventh());
    assert!(Interval::MAJOR_SEVENTH.is_seventh());
    assert!(Interval::AUGMENTED_SEVENTH.is_seventh());
    assert!(Interval::DOUBLY_AUGMENTED_SEVENTH.is_seventh());
}

#[test]
fn test_display_doubly_modified_intervals() {
    // Doubly diminished
    assert_eq!(Interval::DOUBLY_DIMINISHED_UNISON.to_string(), "dd1");
    assert_eq!(Interval::DOUBLY_DIMINISHED_SECOND.to_string(), "dd2");
    assert_eq!(Interval::DOUBLY_DIMINISHED_THIRD.to_string(), "dd3");
    assert_eq!(Interval::DOUBLY_DIMINISHED_FOURTH.to_string(), "dd4");
    assert_eq!(Interval::DOUBLY_DIMINISHED_FIFTH.to_string(), "dd5");
    assert_eq!(Interval::DOUBLY_DIMINISHED_SIXTH.to_string(), "dd6");
    assert_eq!(Interval::DOUBLY_DIMINISHED_SEVENTH.to_string(), "dd7");
    assert_eq!(Interval::DOUBLY_DIMINISHED_OCTAVE.to_string(), "dd8");

    // Doubly augmented
    assert_eq!(Interval::DOUBLY_AUGMENTED_UNISON.to_string(), "AA1");
    assert_eq!(Interval::DOUBLY_AUGMENTED_SECOND.to_string(), "AA2");
    assert_eq!(Interval::DOUBLY_AUGMENTED_THIRD.to_string(), "AA3");
    assert_eq!(Interval::DOUBLY_AUGMENTED_FOURTH.to_string(), "AA4");
    assert_eq!(Interval::DOUBLY_AUGMENTED_FIFTH.to_string(), "AA5");
    assert_eq!(Interval::DOUBLY_AUGMENTED_SIXTH.to_string(), "AA6");
    assert_eq!(Interval::DOUBLY_AUGMENTED_SEVENTH.to_string(), "AA7");
    assert_eq!(Interval::DOUBLY_AUGMENTED_OCTAVE.to_string(), "AA8");
}

#[test]
fn test_parse_multiple_augmented_diminished() {
    // Test multiple augmented/diminished (if you support AA4, dd5, etc.)
    let aa4 = "AA4".parse::<Interval>().unwrap();
    assert_eq!(aa4.fifths, 6 + 7); // Augmented 4th + another 7 fifths

    let dd5 = "dd5".parse::<Interval>().unwrap();
    assert_eq!(dd5.fifths, 1 - 7 - 7); // Perfect 5th - two diminutions
}

#[test]
fn test_parse_errors() {
    assert!("P2".parse::<Interval>().is_err()); // 2nd can't be perfect
    assert!("m4".parse::<Interval>().is_err()); // 4th can't be minor
    assert!("M5".parse::<Interval>().is_err()); // 5th can't be major
    assert!("X3".parse::<Interval>().is_err()); // Invalid quality
    assert!("M0".parse::<Interval>().is_err()); // Invalid number
    assert!("".parse::<Interval>().is_err()); // Empty string
}

#[test]
fn test_roundtrip() {
    // Test that parsing and displaying are consistent
    let intervals = [
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::AUGMENTED_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_NINTH,
    ];

    for interval in intervals {
        let string_rep = interval.to_string();
        let parsed = string_rep.parse::<Interval>().unwrap();
        assert_eq!(parsed, interval, "Failed roundtrip for {}", string_rep);
    }
}
