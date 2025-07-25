use chordy::prelude::*;

#[test]
fn test_roman_degree_creation() {
    // Test roman degree enum (1-7)
    let degrees = [
        RomanDegree::I,
        RomanDegree::II,
        RomanDegree::III,
        RomanDegree::IV,
        RomanDegree::V,
        RomanDegree::VI,
        RomanDegree::VII,
    ];

    // Test that they are distinct
    for (i, &degree) in degrees.iter().enumerate() {
        for (j, &other) in degrees.iter().enumerate() {
            if i != j {
                assert_ne!(degree, other);
            }
        }
    }
}

#[test]
fn test_roman_numeral_creation() {
    // Test roman numeral creation with degree + accidental structure
    let one = RomanNumeral::new(RomanDegree::I, Accidental::Natural);
    let two = RomanNumeral::new(RomanDegree::II, Accidental::Natural);
    let flat_three = RomanNumeral::new(RomanDegree::III, Accidental::Flat);
    let four = RomanNumeral::new(RomanDegree::IV, Accidental::Natural);
    let five = RomanNumeral::new(RomanDegree::V, Accidental::Natural);
    let flat_six = RomanNumeral::new(RomanDegree::VI, Accidental::Flat);
    let flat_seven = RomanNumeral::new(RomanDegree::VII, Accidental::Flat);

    // Test that they are distinct
    assert_ne!(one, two);
    assert_ne!(four, five);
    assert_ne!(
        RomanNumeral::new(RomanDegree::III, Accidental::Natural),
        flat_three
    );
    assert_ne!(flat_six, flat_seven);

    // Test degree access
    assert_eq!(one.degree(), RomanDegree::I);
    assert_eq!(flat_three.degree(), RomanDegree::III);
    assert_eq!(flat_three.accidental(), Accidental::Flat);
    assert_eq!(flat_six.degree(), RomanDegree::VI);
    assert_eq!(flat_six.accidental(), Accidental::Flat);
    assert_eq!(flat_seven.degree(), RomanDegree::VII);
    assert_eq!(flat_seven.accidental(), Accidental::Flat);
}

#[test]
fn test_roman_numeral_display_major_context() {
    // Test display in major key context using chord naming system (uppercase for major/aug, lowercase for minor/dim)
    let major_one = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let minor_two = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let minor_three = RomanChord::minor(RomanNumeral::new(RomanDegree::III, Accidental::Natural));
    let major_four = RomanChord::major(RomanNumeral::new(RomanDegree::IV, Accidental::Natural));
    let major_five = RomanChord::major(RomanNumeral::new(RomanDegree::V, Accidental::Natural));
    let minor_six = RomanChord::minor(RomanNumeral::new(RomanDegree::VI, Accidental::Natural));
    let dim_seven =
        RomanChord::diminished(RomanNumeral::new(RomanDegree::VII, Accidental::Natural));

    // Display depends on chord quality context
    assert_eq!(major_one.to_string(), "I");
    assert_eq!(minor_two.to_string(), "ii");
    assert_eq!(minor_three.to_string(), "iii");
    assert_eq!(major_four.to_string(), "IV");
    assert_eq!(major_five.to_string(), "V");
    assert_eq!(minor_six.to_string(), "vi");
    assert_eq!(dim_seven.to_string(), "vii°");
}

#[test]
fn test_roman_numeral_display_minor_context() {
    // Test display in minor key context using chord naming system with proper accidentals
    let minor_one = RomanChord::minor(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let dim_two = RomanChord::diminished(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let major_flat_three = RomanChord::major(RomanNumeral::new(RomanDegree::III, Accidental::Flat));
    let minor_four = RomanChord::minor(RomanNumeral::new(RomanDegree::IV, Accidental::Natural));
    let minor_five = RomanChord::minor(RomanNumeral::new(RomanDegree::V, Accidental::Natural));
    let major_flat_six = RomanChord::major(RomanNumeral::new(RomanDegree::VI, Accidental::Flat));
    let major_flat_seven = RomanChord::major(RomanNumeral::new(RomanDegree::VII, Accidental::Flat));

    assert_eq!(minor_one.to_string(), "i");
    assert_eq!(dim_two.to_string(), "ii°");
    assert_eq!(major_flat_three.to_string(), "♭III");
    assert_eq!(minor_four.to_string(), "iv");
    assert_eq!(minor_five.to_string(), "v");
    assert_eq!(major_flat_six.to_string(), "♭VI");
    assert_eq!(major_flat_seven.to_string(), "♭VII");
}

#[test]
fn test_roman_numeral_parsing() {
    // Test parsing from strings with proper accidental handling
    assert_eq!(
        "I".parse::<RomanNumeral>().unwrap(),
        RomanNumeral::new(RomanDegree::I, Accidental::Natural)
    );
    assert_eq!(
        "♭III".parse::<RomanNumeral>().unwrap(),
        RomanNumeral::new(RomanDegree::III, Accidental::Flat)
    );
    assert_eq!(
        "♯IV".parse::<RomanNumeral>().unwrap(),
        RomanNumeral::new(RomanDegree::IV, Accidental::Sharp)
    );
    assert_eq!(
        "♭VII".parse::<RomanNumeral>().unwrap(),
        RomanNumeral::new(RomanDegree::VII, Accidental::Flat)
    );
}

#[test]
fn test_roman_chord_creation() {
    // Test basic roman chord creation
    let major_one = RomanChord::new(
        RomanNumeral::new(RomanDegree::I, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ],
    );

    let minor_two = RomanChord::new(
        RomanNumeral::new(RomanDegree::II, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FIFTH,
        ],
    );

    // Test that root access works
    assert_eq!(
        major_one.root(),
        RomanNumeral::new(RomanDegree::I, Accidental::Natural)
    );
    assert_eq!(
        minor_two.root(),
        RomanNumeral::new(RomanDegree::II, Accidental::Natural)
    );
}

#[test]
fn test_roman_chord_helper_methods() {
    // Test common chord creation helpers
    let major_i = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let minor_ii = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let diminished_vii =
        RomanChord::diminished(RomanNumeral::new(RomanDegree::VII, Accidental::Natural));

    // Test intervals are correct
    assert_eq!(
        major_i.intervals(),
        &vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ]
    );

    assert_eq!(
        minor_ii.intervals(),
        &vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::PERFECT_FIFTH,
        ]
    );

    assert_eq!(
        diminished_vii.intervals(),
        &vec![
            Interval::PERFECT_UNISON,
            Interval::MINOR_THIRD,
            Interval::DIMINISHED_FIFTH,
        ]
    );
}

#[test]
fn test_chord_to_roman_conversion_major_key() {
    // Test converting regular chords to roman numerals in a major key
    let c_major_key = Key::Major(note!("C"));

    let c_chord = Chord::major(note!("C"));
    let d_chord = Chord::minor(note!("D"));
    let e_chord = Chord::minor(note!("E"));
    let f_chord = Chord::major(note!("F"));
    let g_chord = Chord::major(note!("G"));
    let a_chord = Chord::minor(note!("A"));
    let b_chord = Chord::diminished(note!("B"));

    // Convert to roman numerals in C major
    let roman_c = c_chord.to_roman(&c_major_key).unwrap();
    let roman_d = d_chord.to_roman(&c_major_key).unwrap();
    let roman_e = e_chord.to_roman(&c_major_key).unwrap();
    let roman_f = f_chord.to_roman(&c_major_key).unwrap();
    let roman_g = g_chord.to_roman(&c_major_key).unwrap();
    let roman_a = a_chord.to_roman(&c_major_key).unwrap();
    let roman_b = b_chord.to_roman(&c_major_key).unwrap();

    // Check degrees (all natural in major key)
    assert_eq!(roman_c.root().degree(), RomanDegree::I);
    assert_eq!(roman_d.root().degree(), RomanDegree::II);
    assert_eq!(roman_e.root().degree(), RomanDegree::III);
    assert_eq!(roman_f.root().degree(), RomanDegree::IV);
    assert_eq!(roman_g.root().degree(), RomanDegree::V);
    assert_eq!(roman_a.root().degree(), RomanDegree::VI);
    assert_eq!(roman_b.root().degree(), RomanDegree::VII);

    // Check accidentals (all natural in C major)
    assert_eq!(roman_c.root().accidental(), Accidental::Natural);
    assert_eq!(roman_d.root().accidental(), Accidental::Natural);
    assert_eq!(roman_e.root().accidental(), Accidental::Natural);
}

#[test]
fn test_chord_to_roman_conversion_minor_key() {
    // Test converting regular chords to roman numerals in a minor key
    let a_minor_key = Key::Minor(note!("A"));

    let a_chord = Chord::minor(note!("A"));
    let b_chord = Chord::diminished(note!("B"));
    let c_chord = Chord::major(note!("C"));
    let d_chord = Chord::minor(note!("D"));
    let e_chord = Chord::minor(note!("E"));
    let f_chord = Chord::major(note!("F"));
    let g_chord = Chord::major(note!("G"));

    // Convert to roman numerals in A minor
    let roman_a = a_chord.to_roman(&a_minor_key).unwrap();
    let roman_b = b_chord.to_roman(&a_minor_key).unwrap();
    let roman_c = c_chord.to_roman(&a_minor_key).unwrap();
    let roman_d = d_chord.to_roman(&a_minor_key).unwrap();
    let roman_e = e_chord.to_roman(&a_minor_key).unwrap();
    let roman_f = f_chord.to_roman(&a_minor_key).unwrap();
    let roman_g = g_chord.to_roman(&a_minor_key).unwrap();

    // Check degrees
    assert_eq!(roman_a.root().degree(), RomanDegree::I);
    assert_eq!(roman_b.root().degree(), RomanDegree::II);
    assert_eq!(roman_c.root().degree(), RomanDegree::III);
    assert_eq!(roman_d.root().degree(), RomanDegree::IV);
    assert_eq!(roman_e.root().degree(), RomanDegree::V);
    assert_eq!(roman_f.root().degree(), RomanDegree::VI);
    assert_eq!(roman_g.root().degree(), RomanDegree::VII);

    // Check accidentals (flat for III, VI, VII in minor relative to major)
    assert_eq!(roman_a.root().accidental(), Accidental::Natural);
    assert_eq!(roman_b.root().accidental(), Accidental::Natural);
    assert_eq!(roman_c.root().accidental(), Accidental::Flat); // ♭III
    assert_eq!(roman_d.root().accidental(), Accidental::Natural);
    assert_eq!(roman_e.root().accidental(), Accidental::Natural);
    assert_eq!(roman_f.root().accidental(), Accidental::Flat); // ♭VI
    assert_eq!(roman_g.root().accidental(), Accidental::Flat); // ♭VII
}

#[test]
fn test_roman_to_chord_conversion_major_key() {
    // Test converting roman numerals to actual chords in a major key
    let c_major_key = Key::Major(note!("C"));

    let roman_i = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let roman_ii = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let roman_v = RomanChord::major(RomanNumeral::new(RomanDegree::V, Accidental::Natural));

    // Convert to actual chords
    let c_chord = roman_i.in_key(&c_major_key);
    let d_chord = roman_ii.in_key(&c_major_key);
    let g_chord = roman_v.in_key(&c_major_key);

    assert_eq!(c_chord.root, note!("C"));
    assert_eq!(d_chord.root, note!("D"));
    assert_eq!(g_chord.root, note!("G"));
}

#[test]
fn test_roman_to_chord_conversion_minor_key() {
    // Test converting roman numerals to actual chords in a minor key
    let a_minor_key = Key::Minor(note!("A"));

    let roman_i = RomanChord::minor(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let roman_biii = RomanChord::major(RomanNumeral::new(RomanDegree::III, Accidental::Flat));
    let roman_v = RomanChord::minor(RomanNumeral::new(RomanDegree::V, Accidental::Natural));

    // Convert to actual chords
    let a_chord = roman_i.in_key(&a_minor_key);
    let c_chord = roman_biii.in_key(&a_minor_key);
    let e_chord = roman_v.in_key(&a_minor_key);

    assert_eq!(a_chord.root, note!("A"));
    assert_eq!(c_chord.root, note!("C"));
    assert_eq!(e_chord.root, note!("E"));
}

#[test]
fn test_roman_chord_display() {
    // Test roman chord display with quality-based case
    let major_i = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let minor_ii = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let dominant_seventh = RomanChord::new(
        RomanNumeral::new(RomanDegree::V, Accidental::Natural),
        vec![
            Interval::PERFECT_UNISON,
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
            Interval::MINOR_SEVENTH,
        ],
    );

    assert_eq!(major_i.to_string(), "I");
    assert_eq!(minor_ii.to_string(), "ii");
    assert_eq!(dominant_seventh.to_string(), "V7");
}

#[test]
fn test_roman_macro() {
    // Test the roman! macro for compile-time creation
    let roman_one = roman!("I");
    let roman_flat_three = roman!("♭III");
    let roman_sharp_four = roman!("♯IV");

    assert_eq!(
        roman_one,
        RomanNumeral::new(RomanDegree::I, Accidental::Natural)
    );
    assert_eq!(
        roman_flat_three,
        RomanNumeral::new(RomanDegree::III, Accidental::Flat)
    );
    assert_eq!(
        roman_sharp_four,
        RomanNumeral::new(RomanDegree::IV, Accidental::Sharp)
    );
}

#[test]
fn test_from_interval_for_scale_degree() {
    // Test From<Interval> for ScaleDegree
    let unison = ScaleDegree::from(Interval::PERFECT_UNISON);
    assert_eq!(unison.step, 1);
    assert_eq!(unison.alteration, None);

    let major_second = ScaleDegree::from(Interval::MAJOR_SECOND);
    assert_eq!(major_second.step, 2);
    assert_eq!(major_second.alteration, None);

    let minor_third = ScaleDegree::from(Interval::MINOR_THIRD);
    assert_eq!(minor_third.step, 3);
    assert_eq!(minor_third.alteration, Some(Accidental::Flat));

    let perfect_fourth = ScaleDegree::from(Interval::PERFECT_FOURTH);
    assert_eq!(perfect_fourth.step, 4);
    assert_eq!(perfect_fourth.alteration, None);

    let augmented_fourth = ScaleDegree::from(Interval::AUGMENTED_FOURTH);
    assert_eq!(augmented_fourth.step, 4);
    assert_eq!(augmented_fourth.alteration, Some(Accidental::Sharp));

    // Test compound intervals reduce to simple
    let major_ninth = ScaleDegree::from(Interval::MAJOR_NINTH);
    assert_eq!(major_ninth.step, 2);
    assert_eq!(major_ninth.alteration, None);
}

#[test]
fn test_from_scale_degree_for_roman_numeral() {
    // Test From<ScaleDegree> for RomanNumeral
    let scale_degree_1 = ScaleDegree::new(1, None);
    let roman_1 = RomanNumeral::from(scale_degree_1);
    assert_eq!(roman_1.degree(), RomanDegree::I);
    assert_eq!(roman_1.accidental(), Accidental::Natural);

    let scale_degree_flat_3 = ScaleDegree::new(3, Some(Accidental::Flat));
    let roman_flat_3 = RomanNumeral::from(scale_degree_flat_3);
    assert_eq!(roman_flat_3.degree(), RomanDegree::III);
    assert_eq!(roman_flat_3.accidental(), Accidental::Flat);

    let scale_degree_sharp_5 = ScaleDegree::new(5, Some(Accidental::Sharp));
    let roman_sharp_5 = RomanNumeral::from(scale_degree_sharp_5);
    assert_eq!(roman_sharp_5.degree(), RomanDegree::V);
    assert_eq!(roman_sharp_5.accidental(), Accidental::Sharp);
}

#[test]
fn test_from_interval_for_roman_numeral() {
    // Test From<Interval> for RomanNumeral (chained conversion)
    let roman_from_unison = RomanNumeral::from(Interval::PERFECT_UNISON);
    assert_eq!(roman_from_unison.degree(), RomanDegree::I);
    assert_eq!(roman_from_unison.accidental(), Accidental::Natural);

    let roman_from_major_third = RomanNumeral::from(Interval::MAJOR_THIRD);
    assert_eq!(roman_from_major_third.degree(), RomanDegree::III);
    assert_eq!(roman_from_major_third.accidental(), Accidental::Natural);

    let roman_from_minor_sixth = RomanNumeral::from(Interval::MINOR_SIXTH);
    assert_eq!(roman_from_minor_sixth.degree(), RomanDegree::VI);
    assert_eq!(roman_from_minor_sixth.accidental(), Accidental::Flat);

    let roman_from_aug_fourth = RomanNumeral::from(Interval::AUGMENTED_FOURTH);
    assert_eq!(roman_from_aug_fourth.degree(), RomanDegree::IV);
    assert_eq!(roman_from_aug_fourth.accidental(), Accidental::Sharp);
}

#[test]
fn test_scale_degree_to_interval_basic() {
    // Test basic scale degrees to interval conversion
    let tonic = ScaleDegree::new(1, None);
    assert_eq!(tonic.to_interval(), Interval::PERFECT_UNISON);

    let supertonic = ScaleDegree::new(2, None);
    assert_eq!(supertonic.to_interval(), Interval::MAJOR_SECOND);

    let mediant = ScaleDegree::new(3, None);
    assert_eq!(mediant.to_interval(), Interval::MAJOR_THIRD);

    let subdominant = ScaleDegree::new(4, None);
    assert_eq!(subdominant.to_interval(), Interval::PERFECT_FOURTH);

    let dominant = ScaleDegree::new(5, None);
    assert_eq!(dominant.to_interval(), Interval::PERFECT_FIFTH);

    let submediant = ScaleDegree::new(6, None);
    assert_eq!(submediant.to_interval(), Interval::MAJOR_SIXTH);

    let leading_tone = ScaleDegree::new(7, None);
    assert_eq!(leading_tone.to_interval(), Interval::MAJOR_SEVENTH);
}

#[test]
fn test_scale_degree_to_interval_flat() {
    // Test flat scale degrees to interval conversion
    let flat_second = ScaleDegree::new(2, Some(Accidental::Flat));
    assert_eq!(flat_second.to_interval(), Interval::MINOR_SECOND);

    let flat_third = ScaleDegree::new(3, Some(Accidental::Flat));
    assert_eq!(flat_third.to_interval(), Interval::MINOR_THIRD);

    let flat_fifth = ScaleDegree::new(5, Some(Accidental::Flat));
    assert_eq!(flat_fifth.to_interval(), Interval::DIMINISHED_FIFTH);

    let flat_sixth = ScaleDegree::new(6, Some(Accidental::Flat));
    assert_eq!(flat_sixth.to_interval(), Interval::MINOR_SIXTH);

    let flat_seventh = ScaleDegree::new(7, Some(Accidental::Flat));
    assert_eq!(flat_seventh.to_interval(), Interval::MINOR_SEVENTH);
}

#[test]
fn test_scale_degree_to_interval_sharp() {
    // Test sharp scale degrees to interval conversion
    let sharp_first = ScaleDegree::new(1, Some(Accidental::Sharp));
    assert_eq!(sharp_first.to_interval(), Interval::AUGMENTED_UNISON);

    let sharp_second = ScaleDegree::new(2, Some(Accidental::Sharp));
    assert_eq!(sharp_second.to_interval(), Interval::AUGMENTED_SECOND);

    let sharp_fourth = ScaleDegree::new(4, Some(Accidental::Sharp));
    assert_eq!(sharp_fourth.to_interval(), Interval::AUGMENTED_FOURTH);

    let sharp_fifth = ScaleDegree::new(5, Some(Accidental::Sharp));
    assert_eq!(sharp_fifth.to_interval(), Interval::AUGMENTED_FIFTH);
}

#[test]
fn test_roman_numeral_to_interval_basic() {
    // Test basic roman numerals to interval conversion
    let i = RomanNumeral::new(RomanDegree::I, Accidental::Natural);
    assert_eq!(i.to_interval(), Interval::PERFECT_UNISON);

    let ii = RomanNumeral::new(RomanDegree::II, Accidental::Natural);
    assert_eq!(ii.to_interval(), Interval::MAJOR_SECOND);

    let iii = RomanNumeral::new(RomanDegree::III, Accidental::Natural);
    assert_eq!(iii.to_interval(), Interval::MAJOR_THIRD);

    let v = RomanNumeral::new(RomanDegree::V, Accidental::Natural);
    assert_eq!(v.to_interval(), Interval::PERFECT_FIFTH);
}

#[test]
fn test_roman_numeral_to_interval_accidentals() {
    // Test roman numerals with accidentals to interval conversion
    let flat_ii = RomanNumeral::new(RomanDegree::II, Accidental::Flat);
    assert_eq!(flat_ii.to_interval(), Interval::MINOR_SECOND);

    let flat_iii = RomanNumeral::new(RomanDegree::III, Accidental::Flat);
    assert_eq!(flat_iii.to_interval(), Interval::MINOR_THIRD);

    let sharp_iv = RomanNumeral::new(RomanDegree::IV, Accidental::Sharp);
    assert_eq!(sharp_iv.to_interval(), Interval::AUGMENTED_FOURTH);

    let flat_vi = RomanNumeral::new(RomanDegree::VI, Accidental::Flat);
    assert_eq!(flat_vi.to_interval(), Interval::MINOR_SIXTH);

    let flat_vii = RomanNumeral::new(RomanDegree::VII, Accidental::Flat);
    assert_eq!(flat_vii.to_interval(), Interval::MINOR_SEVENTH);
}

#[test]
fn test_interval_roundtrip_conversion() {
    // Test that Interval -> ScaleDegree -> Interval roundtrips correctly
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
        Interval::AUGMENTED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_SEVENTH,
    ];

    for interval in intervals.iter() {
        let scale_degree = ScaleDegree::from(*interval);
        let converted_back = scale_degree.to_interval();
        assert_eq!(
            *interval, converted_back,
            "Roundtrip failed for interval: {:?}",
            interval
        );
    }
}

#[test]
fn test_roman_chord_in_key_with_to_interval() {
    // Test that RomanChord::in_key works correctly with the new to_interval method
    let c_major_key = Key::Major(note!("C"));

    // Test basic triads
    let i_chord = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let actual_chord = i_chord.in_key(&c_major_key);
    assert_eq!(actual_chord.root, note!("C"));

    let v_chord = RomanChord::major(RomanNumeral::new(RomanDegree::V, Accidental::Natural));
    let actual_chord = v_chord.in_key(&c_major_key);
    assert_eq!(actual_chord.root, note!("G"));

    // Test with accidentals
    let flat_ii_chord = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Flat));
    let actual_chord = flat_ii_chord.in_key(&c_major_key);
    assert_eq!(actual_chord.root, note!("Db"));

    let sharp_iv_chord = RomanChord::major(RomanNumeral::new(RomanDegree::IV, Accidental::Sharp));
    let actual_chord = sharp_iv_chord.in_key(&c_major_key);
    assert_eq!(actual_chord.root, note!("F#"));
}

macro_rules! roman_naming_test {
    ($name:ident, $chord_str:expr, $key:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let chord: Chord = $chord_str.parse().unwrap();
            let roman = chord.to_roman(&$key).unwrap();
            assert_eq!(
                roman.to_string(),
                $expected,
                "Roman numeral of chord with notes {} in key {} should be named `{}`",
                $chord_str,
                $key,
                $expected
            );
        }
    };
}

// Triads
roman_naming_test!(test_c_major_roman, "C,E,G", Key::Major(note!("C")), "I");
roman_naming_test!(test_d_minor_roman, "D,F,A", Key::Major(note!("C")), "ii");
roman_naming_test!(
    test_e_flat_minor_roman,
    "Eb,Gb,Bb",
    Key::Major(note!("C")),
    "♭iii"
);
roman_naming_test!(
    test_c_sharp_diminished_roman,
    "C#,E,G",
    Key::Major(note!("C")),
    "♯i°"
);
roman_naming_test!(
    test_f_major_augmented_roman,
    "F,A,C#",
    Key::Major(note!("C")),
    "IV+"
);

// Seventh chords
roman_naming_test!(test_g7_roman, "G,B,D,F", Key::Major(note!("C")), "V7");
roman_naming_test!(
    test_a_minor7_roman,
    "A,C,E,G",
    Key::Major(note!("C")),
    "vi7"
);
roman_naming_test!(
    test_b_half_diminished7_roman,
    "B,D,F,A",
    Key::Major(note!("C")),
    "viiø7"
);
roman_naming_test!(
    test_b_fully_diminished7_roman,
    "B,D,F,Ab",
    Key::Major(note!("C")),
    "vii°7"
);
roman_naming_test!(
    test_d_minor7_roman,
    "D,F,A,C",
    Key::Major(note!("C")),
    "ii7"
);
roman_naming_test!(
    test_e_flat_major7_roman,
    "Eb,G,Bb,D",
    Key::Major(note!("C")),
    "♭IIImaj7"
);
roman_naming_test!(
    test_f_sharp_minor7_roman,
    "F#,A,C#,E",
    Key::Major(note!("Ab")),
    "♯vi7"
);
roman_naming_test!(
    test_c_major7_roman,
    "C,E,G,B",
    Key::Major(note!("C")),
    "Imaj7"
);
