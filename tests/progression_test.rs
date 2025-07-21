//! Basic tests for the chord progression system

use chordy::prelude::*;

#[test]
fn test_chord_api() {
    let c_major_key = Key::Major(note!("C"));
    let g_major_chord = Chord::major(note!("G"));

    let options = c_major_key.progressions_from(&g_major_chord).unwrap();

    // there should be a strong progression towards the C (I) chord from G (V)
    let c_major_chord = Chord::major(note!("C"));
    assert!(!options.strong.is_empty(), "There should be strong progression options from G to C");

    // print out all the options for debugging
    println!("Progression options from G in C:");
    for opt in options.strong.iter() {
        println!("Strong: {}", opt.to_html());
    }

    assert!(options.strong.contains(&c_major_chord),
        "There should be a strong progression to I (C) from V (G)");

    
}

#[test]
fn test_major_key_progressions_from() {
    let c_major = Key::Major(note!("C"));
    
    // Test I chord progressions
    let c_chord = Chord::major(note!("C")); // I chord
    let options = c_major.progressions_from(&c_chord).unwrap();
    
    // Based on new progression map, I should have strong progressions
    assert!(!options.strong.is_empty(), "I should have strong progression options");
    
    // The algorithm may match different I variants, accept valid strong progressions
    let has_f = options.strong.iter().any(|chord| chord.root() == note!("F")); // IV
    let has_g = options.strong.iter().any(|chord| chord.root() == note!("G")); // V
    
    assert!(has_f || has_g, "I should have strong progressions to IV or V");
    
    // Should have moderate progressions too
    assert!(!options.moderate.is_empty(), "I should have moderate progression options");
}

#[test]
fn test_minor_key_progressions_from() {
    let a_minor = Key::Minor(note!("A"));
    
    // Test i chord progressions in minor
    let a_chord = Chord::minor(note!("A")); // i chord
    let options = a_minor.progressions_from(&a_chord).unwrap();
    
    // In minor keys, i might not have strong progressions (no explicit edges)
    // but should have moderate progressions (jumps to primary nodes)
    let total_progressions = options.strong.len() + options.moderate.len() + options.weak.len();
    assert!(total_progressions > 0, "i should have some progression options");
    
    // If no strong progressions, should at least have moderate ones
    if options.strong.is_empty() {
        assert!(!options.moderate.is_empty(), "i should have moderate progression options");
    } else {
        // If there are strong progressions, they should include expected chords
        let has_d = options.strong.iter().any(|chord| chord.root == note!("D")); // iv
        let has_e = options.strong.iter().any(|chord| chord.root == note!("E")); // V
        assert!(has_d || has_e, "i should have strong progressions to iv or V");
    }
}


#[test]
fn test_progression_strength_categories() {
    let c_major = Key::Major(note!("C"));
    let g_chord = Chord::major(note!("G")); // V chord
    let options = c_major.progressions_from(&g_chord).unwrap();
    
    // V should have strong progressions (explicit arrows)
    assert!(!options.strong.is_empty(), "V should have strong progressions");
    
    // Based on new progression map: V -> iii, vi, I
    let has_c = options.strong.iter().any(|chord| chord.root == note!("C")); // I
    let has_e = options.strong.iter().any(|chord| chord.root == note!("E")); // iii
    let has_a = options.strong.iter().any(|chord| chord.root == note!("A")); // vi
    

    println!("Progression options from V in C:");
    for opt in options.strong.iter() {
        println!("Strong: {}", opt.to_html());
    }

    // V should connect to at least one of these targets
    assert!(has_c || has_e || has_a, "V should have strong progression to I, iii, or vi");
    
    // Should also have moderate and weak options
    assert!(!options.moderate.is_empty(), "V should have moderate progression options");
    assert!(!options.weak.is_empty(), "V should have weak progression options");
}

