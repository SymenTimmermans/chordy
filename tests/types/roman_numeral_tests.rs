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
    assert_ne!(RomanNumeral::new(RomanDegree::III, Accidental::Natural), flat_three);
    
    // Test degree access
    assert_eq!(one.degree(), RomanDegree::I);
    assert_eq!(flat_three.degree(), RomanDegree::III);
    assert_eq!(flat_three.accidental(), Accidental::Flat);
}

#[test]
fn test_roman_numeral_display_major_context() {
    // Test display in major key context (uppercase for major/aug, lowercase for minor/dim)
    let major_one = RomanNumeral::new(RomanDegree::I, Accidental::Natural);
    let minor_two = RomanNumeral::new(RomanDegree::II, Accidental::Natural);
    let minor_three = RomanNumeral::new(RomanDegree::III, Accidental::Natural);
    let major_four = RomanNumeral::new(RomanDegree::IV, Accidental::Natural);
    let major_five = RomanNumeral::new(RomanDegree::V, Accidental::Natural);
    let minor_six = RomanNumeral::new(RomanDegree::VI, Accidental::Natural);
    let dim_seven = RomanNumeral::new(RomanDegree::VII, Accidental::Natural);
    
    // Display depends on chord quality context
    assert_eq!(major_one.display_for_quality(ChordQuality::Major), "I");
    assert_eq!(minor_two.display_for_quality(ChordQuality::Minor), "ii");
    assert_eq!(minor_three.display_for_quality(ChordQuality::Minor), "iii");
    assert_eq!(major_four.display_for_quality(ChordQuality::Major), "IV");
    assert_eq!(major_five.display_for_quality(ChordQuality::Major), "V");
    assert_eq!(minor_six.display_for_quality(ChordQuality::Minor), "vi");
    assert_eq!(dim_seven.display_for_quality(ChordQuality::Diminished), "vii°");
}

#[test]
fn test_roman_numeral_display_minor_context() {
    // Test display in minor key context with proper accidentals
    let minor_one = RomanNumeral::new(RomanDegree::I, Accidental::Natural);
    let dim_two = RomanNumeral::new(RomanDegree::II, Accidental::Natural);
    let major_flat_three = RomanNumeral::new(RomanDegree::III, Accidental::Flat);
    let minor_four = RomanNumeral::new(RomanDegree::IV, Accidental::Natural);
    let minor_five = RomanNumeral::new(RomanDegree::V, Accidental::Natural);
    let major_flat_six = RomanNumeral::new(RomanDegree::VI, Accidental::Flat);
    let major_flat_seven = RomanNumeral::new(RomanDegree::VII, Accidental::Flat);
    
    assert_eq!(minor_one.display_for_quality(ChordQuality::Minor), "i");
    assert_eq!(dim_two.display_for_quality(ChordQuality::Diminished), "ii°");
    assert_eq!(major_flat_three.display_for_quality(ChordQuality::Major), "♭III");
    assert_eq!(minor_four.display_for_quality(ChordQuality::Minor), "iv");
    assert_eq!(minor_five.display_for_quality(ChordQuality::Minor), "v");
    assert_eq!(major_flat_six.display_for_quality(ChordQuality::Major), "♭VI");
    assert_eq!(major_flat_seven.display_for_quality(ChordQuality::Major), "♭VII");
}

#[test]
fn test_roman_numeral_parsing() {
    // Test parsing from strings with proper accidental handling
    assert_eq!("I".parse::<RomanNumeral>().unwrap(), 
               RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    assert_eq!("♭III".parse::<RomanNumeral>().unwrap(), 
               RomanNumeral::new(RomanDegree::III, Accidental::Flat));
    assert_eq!("♯IV".parse::<RomanNumeral>().unwrap(), 
               RomanNumeral::new(RomanDegree::IV, Accidental::Sharp));
    assert_eq!("♭VII".parse::<RomanNumeral>().unwrap(), 
               RomanNumeral::new(RomanDegree::VII, Accidental::Flat));
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
    assert_eq!(major_one.root(), RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    assert_eq!(minor_two.root(), RomanNumeral::new(RomanDegree::II, Accidental::Natural));
}

#[test]
fn test_roman_chord_helper_methods() {
    // Test common chord creation helpers
    let major_i = RomanChord::major(RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    let minor_ii = RomanChord::minor(RomanNumeral::new(RomanDegree::II, Accidental::Natural));
    let diminished_vii = RomanChord::diminished(RomanNumeral::new(RomanDegree::VII, Accidental::Natural));

    // Test intervals are correct
    assert_eq!(major_i.intervals(), &vec![
        Interval::PERFECT_UNISON,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
    ]);
    
    assert_eq!(minor_ii.intervals(), &vec![
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FIFTH,
    ]);
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
    assert_eq!(roman_c.root().accidental(), Accidental::Flat);  // ♭III
    assert_eq!(roman_d.root().accidental(), Accidental::Natural);
    assert_eq!(roman_e.root().accidental(), Accidental::Natural);
    assert_eq!(roman_f.root().accidental(), Accidental::Flat);  // ♭VI
    assert_eq!(roman_g.root().accidental(), Accidental::Flat);  // ♭VII
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

    assert_eq!(roman_one, RomanNumeral::new(RomanDegree::I, Accidental::Natural));
    assert_eq!(roman_flat_three, RomanNumeral::new(RomanDegree::III, Accidental::Flat));
    assert_eq!(roman_sharp_four, RomanNumeral::new(RomanDegree::IV, Accidental::Sharp));
}


