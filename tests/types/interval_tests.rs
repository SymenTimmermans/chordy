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
