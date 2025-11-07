use chordy::{Chord, note, PianoVoicingConfig};

#[test]
fn test_left_hand_voicing_generates_low_notes() {
    let chord = Chord::major(note!("C"));

    // Test left hand configuration
    let left_config = PianoVoicingConfig::left_hand();
    let left_voiced = chord.voice_piano_with_config(&left_config).unwrap();

    // Left hand should have notes in lower octaves
    let has_low_notes = left_voiced.pitches.iter().any(|p| p.midi_number() <= 60);
    assert!(has_low_notes, "Left-hand voicing should include low notes");

    // Left hand should have appropriate note count
    let note_count = left_voiced.pitches.len();
    assert!(note_count <= 4, "Left-hand voicing should have <= 4 notes, got {}", note_count);
}

#[test]
fn test_both_hands_voicing_splits_notes() {
    let chord = Chord::major(note!("C"));

    // Test both hands configuration
    let both_config = PianoVoicingConfig::default();
    let both_voiced = chord.voice_piano_with_config(&both_config).unwrap();

    // Both hands should have both low and high notes
    let has_low_notes = both_voiced.pitches.iter().any(|p| p.midi_number() <= 60);
    let has_high_notes = both_voiced.pitches.iter().any(|p| p.midi_number() > 60);

    assert!(has_low_notes, "Both-hands voicing should include low notes");
    assert!(has_high_notes, "Both-hands voicing should include high notes");

    // Note repetition is common in piano voicing
    let unique_pitches: std::collections::HashSet<_> = both_voiced.pitches.iter().map(|p| p.midi_number()).collect();
    let has_repetition = both_voiced.pitches.len() > unique_pitches.len();

    // It's okay to have repetition in piano voicing
    println!("Both-hands voicing: {:?}", both_voiced.pitches);
    println!("Has note repetition: {}", has_repetition);
}

#[test]
fn test_right_hand_voicing_generates_high_notes() {
    let chord = Chord::major(note!("C"));

    // Test right hand configuration
    let right_config = PianoVoicingConfig::right_hand();
    let right_voiced = chord.voice_piano_with_config(&right_config).unwrap();

    // Right hand should have notes in higher octaves
    let has_high_notes = right_voiced.pitches.iter().any(|p| p.midi_number() > 60);
    assert!(has_high_notes, "Right-hand voicing should include high notes");

    // Right hand can have more notes than left hand
    let note_count = right_voiced.pitches.len();
    assert!(note_count <= 5, "Right-hand voicing should have <= 5 notes, got {}", note_count);
}