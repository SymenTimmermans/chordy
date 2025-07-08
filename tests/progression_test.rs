//! Basic tests for the chord progression system

use chordy::prelude::*;

#[test]
fn test_chord_api() {
    let c_major_key = Key::Major(note!("C"));
    let g_major_chord = Chord::major(note!("G"));

    let options = c_major_key.progression_options(&g_major_chord).unwrap();

    // there should be a strong progression towards the C (I) chord from G (V)
    let c_major_chord = Chord::major(note!("C"));
    assert!(!options.strong.is_empty(), "There should be strong progression options from G to C");

    assert!(options.strong.contains(&c_major_chord),
        "There should be a strong progression to I (C) from V (G)");

    
}

#[test]
fn test_major_key_progression_options() {
    let c_major = Key::Major(note!("C"));
    
    // Test I chord progressions
    let c_chord = Chord::major(note!("C")); // I chord
    let options = c_major.progression_options(&c_chord).unwrap();
    
    // I should have strong progressions to IV, V, and vi
    assert!(!options.strong.is_empty(), "I should have strong progression options");
    assert!(!options.moderate.is_empty(), "I should have moderate progression options");
    
    // Find specific progressions
    let has_f = options.strong.iter().any(|chord| chord.root() == note!("F")); // IV
    let has_g = options.strong.iter().any(|chord| chord.root() == note!("G")); // V
    let has_a = options.strong.iter().any(|chord| chord.root() == note!("A")); // vi
    
    assert!(has_f, "I should have strong progression to IV (F)");
    assert!(has_g, "I should have strong progression to V (G)");
    assert!(has_a, "I should have strong progression to vi (A)");
}

#[test]
fn test_minor_key_progression_options() {
    let a_minor = Key::Minor(note!("A"));
    
    // Test i chord progressions in minor
    let a_chord = Chord::minor(note!("A")); // i chord
    let options = a_minor.progression_options(&a_chord).unwrap();
    
    assert!(!options.strong.is_empty(), "i should have strong progression options");
    
    // Find specific progressions common in minor
    let has_d = options.strong.iter().any(|chord| chord.root == note!("D")); // iv
    let has_e = options.strong.iter().any(|chord| chord.root == note!("E")); // V
    
    assert!(has_d, "i should have strong progression to iv (D)");
    assert!(has_e, "i should have strong progression to V (E)");
}


#[test]
fn test_progression_strength_categories() {
    let c_major = Key::Major(note!("C"));
    let g_chord = Chord::major(note!("G")); // V chord
    let options = c_major.progression_options(&g_chord).unwrap();
    
    // V should have strong progressions (explicit arrows)
    assert!(!options.strong.is_empty(), "V should have strong progressions");
    
    // Should have at least I as a strong target
    let has_c = options.strong.iter().any(|chord| chord.root == note!("C")); // I
    assert!(has_c, "V should have strong progression to I (C)");
    
    // Should also have moderate and weak options
    assert!(!options.moderate.is_empty(), "V should have moderate progression options");
    assert!(!options.weak.is_empty(), "V should have weak progression options");
}

