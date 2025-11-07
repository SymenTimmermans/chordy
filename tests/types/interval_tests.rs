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

#[test]
fn test_interval_frequency_ratio() {
    // Test equal temperament frequency ratios
    let tolerance = 0.001;

    // Perfect unison (1:1)
    assert!((Interval::PERFECT_UNISON.frequency_ratio() - 1.0).abs() < tolerance);

    // Octave (2:1)
    assert!((Interval::OCTAVE.frequency_ratio() - 2.0).abs() < tolerance);

    // Perfect fifth (≈1.4983:1)
    let fifth_ratio = Interval::PERFECT_FIFTH.frequency_ratio();
    assert!((fifth_ratio - 1.4983).abs() < tolerance);

    // Perfect fourth (≈1.3348:1)
    let fourth_ratio = Interval::PERFECT_FOURTH.frequency_ratio();
    assert!((fourth_ratio - 1.3348).abs() < tolerance);

    // Major third (≈1.2599:1)
    let major_third_ratio = Interval::MAJOR_THIRD.frequency_ratio();
    assert!((major_third_ratio - 1.2599).abs() < tolerance);

    // Minor third (≈1.1892:1)
    let minor_third_ratio = Interval::MINOR_THIRD.frequency_ratio();
    assert!((minor_third_ratio - 1.1892).abs() < tolerance);

    // Major second (≈1.1225:1)
    let major_second_ratio = Interval::MAJOR_SECOND.frequency_ratio();
    assert!((major_second_ratio - 1.1225).abs() < tolerance);

    // Minor second (≈1.0595:1)
    let minor_second_ratio = Interval::MINOR_SECOND.frequency_ratio();
    assert!((minor_second_ratio - 1.0595).abs() < tolerance);
}

#[test]
fn test_interval_just_intonation_ratio() {
    // Test just intonation ratios (pure intervals)

    // Perfect unison (1:1)
    assert_eq!(Interval::PERFECT_UNISON.just_intonation_ratio(), (1, 1));

    // Octave (2:1)
    assert_eq!(Interval::OCTAVE.just_intonation_ratio(), (2, 1));

    // Perfect fifth (3:2)
    assert_eq!(Interval::PERFECT_FIFTH.just_intonation_ratio(), (3, 2));

    // Perfect fourth (4:3)
    assert_eq!(Interval::PERFECT_FOURTH.just_intonation_ratio(), (4, 3));

    // Major third (5:4)
    assert_eq!(Interval::MAJOR_THIRD.just_intonation_ratio(), (5, 4));

    // Minor third (6:5)
    assert_eq!(Interval::MINOR_THIRD.just_intonation_ratio(), (6, 5));

    // Major second (9:8)
    assert_eq!(Interval::MAJOR_SECOND.just_intonation_ratio(), (9, 8));

    // Minor second (16:15)
    assert_eq!(Interval::MINOR_SECOND.just_intonation_ratio(), (16, 15));

    // Tritone (45:32)
    assert_eq!(Interval::AUGMENTED_FOURTH.just_intonation_ratio(), (45, 32));
    assert_eq!(Interval::DIMINISHED_FIFTH.just_intonation_ratio(), (45, 32));

    // Major sixth (5:3)
    assert_eq!(Interval::MAJOR_SIXTH.just_intonation_ratio(), (5, 3));

    // Minor sixth (8:5)
    assert_eq!(Interval::MINOR_SIXTH.just_intonation_ratio(), (8, 5));

    // Major seventh (15:8)
    assert_eq!(Interval::MAJOR_SEVENTH.just_intonation_ratio(), (15, 8));

    // Minor seventh (9:5)
    assert_eq!(Interval::MINOR_SEVENTH.just_intonation_ratio(), (9, 5));
}

#[test]
fn test_interval_pythagorean_ratio() {
    // Test Pythagorean tuning ratios

    // Perfect unison (1:1)
    assert_eq!(Interval::PERFECT_UNISON.pythagorean_ratio(), (1, 1));

    // Octave (2:1)
    assert_eq!(Interval::OCTAVE.pythagorean_ratio(), (2, 1));

    // Perfect fifth (3:2)
    assert_eq!(Interval::PERFECT_FIFTH.pythagorean_ratio(), (3, 2));

    // Perfect fourth (4:3)
    assert_eq!(Interval::PERFECT_FOURTH.pythagorean_ratio(), (4, 3));

    // Major third (81:64)
    assert_eq!(Interval::MAJOR_THIRD.pythagorean_ratio(), (81, 64));

    // Minor third (32:27)
    assert_eq!(Interval::MINOR_THIRD.pythagorean_ratio(), (32, 27));

    // Major second (9:8)
    assert_eq!(Interval::MAJOR_SECOND.pythagorean_ratio(), (9, 8));

    // Minor second (256:243) - Pythagorean limma
    assert_eq!(Interval::MINOR_SECOND.pythagorean_ratio(), (256, 243));

    // Tritone (729:512)
    assert_eq!(Interval::AUGMENTED_FOURTH.pythagorean_ratio(), (729, 512));
    assert_eq!(Interval::DIMINISHED_FIFTH.pythagorean_ratio(), (729, 512));

    // Major sixth (27:16)
    assert_eq!(Interval::MAJOR_SIXTH.pythagorean_ratio(), (27, 16));

    // Minor sixth (128:81)
    assert_eq!(Interval::MINOR_SIXTH.pythagorean_ratio(), (128, 81));

    // Major seventh (243:128)
    assert_eq!(Interval::MAJOR_SEVENTH.pythagorean_ratio(), (243, 128));

    // Minor seventh (16:9)
    assert_eq!(Interval::MINOR_SEVENTH.pythagorean_ratio(), (16, 9));
}

#[test]
fn test_interval_from_ratio() {
    // Test identifying intervals from frequency ratios

    // Perfect unison (1:1)
    assert_eq!(Interval::from_ratio(1.0), Some(Interval::PERFECT_UNISON));

    // Octave (2:1)
    assert_eq!(Interval::from_ratio(2.0), Some(Interval::OCTAVE));

    // Perfect fifth (1.5:1)
    assert_eq!(Interval::from_ratio(1.5), Some(Interval::PERFECT_FIFTH));

    // Perfect fourth (1.333:1)
    assert_eq!(Interval::from_ratio(1.333), Some(Interval::PERFECT_FOURTH));

    // Major third (1.25:1)
    assert_eq!(Interval::from_ratio(1.25), Some(Interval::MAJOR_THIRD));

    // Minor third (1.2:1)
    assert_eq!(Interval::from_ratio(1.2), Some(Interval::MINOR_THIRD));

    // Major second (1.125:1)
    assert_eq!(Interval::from_ratio(1.125), Some(Interval::MAJOR_SECOND));

    // Equal temperament fifth (≈1.4983:1)
    assert_eq!(Interval::from_ratio(1.4983), Some(Interval::PERFECT_FIFTH));

    // Equal temperament fourth (≈1.3348:1)
    assert_eq!(Interval::from_ratio(1.3348), Some(Interval::PERFECT_FOURTH));

    // Equal temperament major third (≈1.2599:1)
    assert_eq!(Interval::from_ratio(1.2599), Some(Interval::MAJOR_THIRD));

    // Equal temperament minor third (≈1.1892:1)
    assert_eq!(Interval::from_ratio(1.1892), Some(Interval::MINOR_THIRD));

    // Equal temperament major second (≈1.1225:1)
    assert_eq!(Interval::from_ratio(1.1225), Some(Interval::MAJOR_SECOND));

    // Equal temperament minor second (≈1.0595:1)
    assert_eq!(Interval::from_ratio(1.0595), Some(Interval::MINOR_SECOND));

    // Test compound intervals
    assert_eq!(Interval::from_ratio(2.5), Some(Interval::MAJOR_TENTH)); // Octave + major third (2.5 ≈ 2.5198)
    assert_eq!(Interval::from_ratio(3.0), Some(Interval::PERFECT_TWELFTH)); // Octave + perfect fifth (3.0 ≈ 2.9966)

    // Test no match for invalid ratios
    assert_eq!(Interval::from_ratio(0.5), None); // Below unison
    assert_eq!(Interval::from_ratio(4.5), None); // Above supported range
    assert_eq!(Interval::from_ratio(1.1), Some(Interval::MINOR_SECOND)); // 1.1 ≈ 1.0595 (within 5% tolerance)
}

#[test]
fn test_compound_interval_ratios() {
    // Test that compound intervals work correctly

    // Major ninth (octave + major second)
    assert_eq!(Interval::MAJOR_NINTH.just_intonation_ratio(), (9, 4)); // 9:4
    assert_eq!(Interval::MAJOR_NINTH.pythagorean_ratio(), (9, 4)); // 9:4

    // Major tenth (octave + major third)
    assert_eq!(Interval::MAJOR_TENTH.just_intonation_ratio(), (5, 2)); // 5:2
    assert_eq!(Interval::MAJOR_TENTH.pythagorean_ratio(), (81, 32)); // 81:32

    // Perfect eleventh (octave + perfect fourth)
    assert_eq!(Interval::PERFECT_ELEVENTH.just_intonation_ratio(), (8, 3)); // 8:3
    assert_eq!(Interval::PERFECT_ELEVENTH.pythagorean_ratio(), (8, 3)); // 8:3

    // Perfect twelfth (octave + perfect fifth)
    assert_eq!(Interval::PERFECT_TWELFTH.just_intonation_ratio(), (3, 1)); // 3:1
    assert_eq!(Interval::PERFECT_TWELFTH.pythagorean_ratio(), (3, 1)); // 3:1
}
