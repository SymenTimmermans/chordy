use chordy::{key, note, Key};

#[test]
fn test_key_macro_major_keys() {
    assert_eq!(key!("C"), Key::Major(note!("C")));
    assert_eq!(key!("D"), Key::Major(note!("D")));
    assert_eq!(key!("E"), Key::Major(note!("E")));
    assert_eq!(key!("F"), Key::Major(note!("F")));
    assert_eq!(key!("G"), Key::Major(note!("G")));
    assert_eq!(key!("A"), Key::Major(note!("A")));
    assert_eq!(key!("B"), Key::Major(note!("B")));
}

#[test]
fn test_key_macro_minor_keys() {
    assert_eq!(key!("Am"), Key::Minor(note!("A")));
    assert_eq!(key!("Bm"), Key::Minor(note!("B")));
    assert_eq!(key!("Cm"), Key::Minor(note!("C")));
    assert_eq!(key!("Dm"), Key::Minor(note!("D")));
    assert_eq!(key!("Em"), Key::Minor(note!("E")));
    assert_eq!(key!("Fm"), Key::Minor(note!("F")));
    assert_eq!(key!("Gm"), Key::Minor(note!("G")));
}

#[test]
fn test_key_macro_with_accidentals() {
    // Major keys with sharps
    assert_eq!(key!("F#"), Key::Major(note!("F#")));
    assert_eq!(key!("C#"), Key::Major(note!("C#")));
    assert_eq!(key!("G#"), Key::Major(note!("G#")));

    // Major keys with flats
    assert_eq!(key!("Bb"), Key::Major(note!("Bb")));
    assert_eq!(key!("Eb"), Key::Major(note!("Eb")));
    assert_eq!(key!("Ab"), Key::Major(note!("Ab")));
    assert_eq!(key!("Db"), Key::Major(note!("Db")));
    assert_eq!(key!("Gb"), Key::Major(note!("Gb")));

    // Minor keys with sharps
    assert_eq!(key!("F#m"), Key::Minor(note!("F#")));
    assert_eq!(key!("C#m"), Key::Minor(note!("C#")));
    assert_eq!(key!("D#m"), Key::Minor(note!("D#")));
    assert_eq!(key!("G#m"), Key::Minor(note!("G#")));
    assert_eq!(key!("A#m"), Key::Minor(note!("A#")));

    // Minor keys with flats
    assert_eq!(key!("Bbm"), Key::Minor(note!("Bb")));
    assert_eq!(key!("Ebm"), Key::Minor(note!("Eb")));
    assert_eq!(key!("Abm"), Key::Minor(note!("Ab")));
}

#[test]
fn test_key_macro_unicode_accidentals() {
    // Major keys
    assert_eq!(key!("F♯"), Key::Major(note!("F♯")));
    assert_eq!(key!("B♭"), Key::Major(note!("B♭")));

    // Minor keys
    assert_eq!(key!("F♯m"), Key::Minor(note!("F♯")));
    assert_eq!(key!("B♭m"), Key::Minor(note!("B♭")));
}

#[test]
fn test_key_macro_double_accidentals() {
    assert_eq!(key!("F##"), Key::Major(note!("F##")));
    assert_eq!(key!("Bbb"), Key::Major(note!("Bbb")));
    assert_eq!(key!("G##m"), Key::Minor(note!("G##")));
    assert_eq!(key!("Abbm"), Key::Minor(note!("Abb")));
}

// These should fail to compile when not in test mode
// #[test]
// fn test_key_macro_compile_time_validation() {
//     let invalid1 = key!("H");     // Invalid note
//     let invalid2 = key!("Cm#");   // Invalid syntax
//     let invalid3 = key!("");      // Empty string
//     let invalid4 = key!("m");     // Just 'm'
// }
