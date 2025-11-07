use chordy::{
    note, Chord, PianoHandPosition, PianoHandSpan, PianoVoicer, PianoVoicingConfig, PianoVoicingType,
};

#[test]
fn test_piano_hand_span_default() {
    let span = PianoHandSpan::default();
    assert_eq!(span.left_hand_max_span, 10);
    assert_eq!(span.right_hand_max_span, 12);
    assert_eq!(span.left_hand_max_notes, 4);
    assert_eq!(span.right_hand_max_notes, 5);
    assert_eq!(span.min_finger_spacing, 2);
}

#[test]
fn test_piano_hand_span_small_hands() {
    let span = PianoHandSpan::small_hands();
    assert_eq!(span.left_hand_max_span, 8);
    assert_eq!(span.right_hand_max_span, 10);
    assert_eq!(span.left_hand_max_notes, 3);
    assert_eq!(span.right_hand_max_notes, 4);
}

#[test]
fn test_piano_hand_span_large_hands() {
    let span = PianoHandSpan::large_hands();
    assert_eq!(span.left_hand_max_span, 12);
    assert_eq!(span.right_hand_max_span, 14);
    assert_eq!(span.left_hand_max_notes, 5);
    assert_eq!(span.right_hand_max_notes, 6);
}

#[test]
fn test_piano_hand_span_comfortable_checks() {
    let span = PianoHandSpan::default();

    // Left hand checks
    assert!(span.is_left_hand_comfortable(8));  // Octave
    assert!(span.is_left_hand_comfortable(10)); // Max span
    assert!(!span.is_left_hand_comfortable(11)); // Too wide

    // Right hand checks
    assert!(span.is_right_hand_comfortable(10)); // Octave + minor 3rd
    assert!(span.is_right_hand_comfortable(12)); // Max span
    assert!(!span.is_right_hand_comfortable(13)); // Too wide

    // Note count checks
    assert!(span.is_left_hand_note_count_comfortable(3));
    assert!(span.is_left_hand_note_count_comfortable(4)); // Max
    assert!(!span.is_left_hand_note_count_comfortable(5)); // Too many

    assert!(span.is_right_hand_note_count_comfortable(4));
    assert!(span.is_right_hand_note_count_comfortable(5)); // Max
    assert!(!span.is_right_hand_note_count_comfortable(6)); // Too many
}

#[test]
fn test_piano_voicing_config_default() {
    let config = PianoVoicingConfig::default();
    assert_eq!(config.hand_position, PianoHandPosition::BothHands);
    assert_eq!(config.voicing_type, PianoVoicingType::Block);
    assert!(config.prefer_root_position);
    assert!(config.avoid_voice_crossing);
    assert_eq!(config.max_total_span, 24);
}

#[test]
fn test_piano_voicing_config_presets() {
    // Left hand config
    let left_config = PianoVoicingConfig::left_hand();
    assert_eq!(left_config.hand_position, PianoHandPosition::LeftHand);

    // Right hand config
    let right_config = PianoVoicingConfig::right_hand();
    assert_eq!(right_config.hand_position, PianoHandPosition::RightHand);

    // Jazz config
    let jazz_config = PianoVoicingConfig::jazz();
    assert_eq!(jazz_config.voicing_type, PianoVoicingType::Shell);
    assert!(!jazz_config.prefer_root_position);

    // Classical config
    let classical_config = PianoVoicingConfig::classical();
    assert_eq!(classical_config.voicing_type, PianoVoicingType::Block);
    assert!(classical_config.prefer_root_position);
    assert!(classical_config.avoid_voice_crossing);

    // Broken config
    let broken_config = PianoVoicingConfig::broken();
    assert_eq!(broken_config.voicing_type, PianoVoicingType::Broken);
}

#[test]
fn test_piano_voicer_ergonomic_score() {
    let voicer = PianoVoicer::default();

    // Test with C major chord pitches
    let c4 = "C4".parse().unwrap();
    let e4 = "E4".parse().unwrap();
    let g4 = "G4".parse().unwrap();

    let pitches = vec![c4, e4, g4];
    let score = voicer.ergonomic_score(&pitches);

    // Should be highly ergonomic (small span, few notes, even spacing)
    assert!(score > 0.8);

    // Test with wider spacing
    let c3 = "C3".parse().unwrap();
    let g4 = "G4".parse().unwrap();
    let c5 = "C5".parse().unwrap();

    let wide_pitches = vec![c3, g4, c5];
    let wide_score = voicer.ergonomic_score(&wide_pitches);

    // Should be less ergonomic due to wide span
    assert!(wide_score < score);
}

#[test]
fn test_piano_voicer_is_ergonomic() {
    let voicer = PianoVoicer::default();

    // Comfortable chord within octave
    let c4 = "C4".parse().unwrap();
    let e4 = "E4".parse().unwrap();
    let g4 = "G4".parse().unwrap();
    let comfortable_pitches = vec![c4, e4, g4];
    assert!(voicer.is_ergonomic(&comfortable_pitches));

    // Too wide span for both hands (more than 2 octaves)
    let c3 = "C3".parse().unwrap();
    let c6 = "C6".parse().unwrap();
    let wide_pitches = vec![c3, c6];
    assert!(!voicer.is_ergonomic(&wide_pitches));
}

#[test]
fn test_chord_piano_voicing_methods() {
    let chord = Chord::major(note!("C"));

    // Test block voicing
    let block_voiced = chord.voice_piano_block().unwrap();
    assert!(block_voiced.is_piano_voicing());
    assert_eq!(block_voiced.piano_voicing_type(), Some(&PianoVoicingType::Block));

    // Test spread voicing
    let spread_voiced = chord.voice_piano_spread().unwrap();
    assert!(spread_voiced.is_piano_voicing());
    assert_eq!(spread_voiced.piano_voicing_type(), Some(&PianoVoicingType::Spread));

    // Test jazz voicing (shell)
    let jazz_chord = Chord::dominant_7th(note!("C"));
    let jazz_voiced = jazz_chord.voice_piano_jazz().unwrap();
    assert!(jazz_voiced.is_piano_voicing());
    assert_eq!(jazz_voiced.piano_voicing_type(), Some(&PianoVoicingType::Shell));

    // Test rootless voicing
    let rootless_voiced = jazz_chord.voice_piano_rootless().unwrap();
    assert!(rootless_voiced.is_piano_voicing());
    assert_eq!(rootless_voiced.piano_voicing_type(), Some(&PianoVoicingType::Rootless));

    // Test broken voicing
    let broken_voiced = chord.voice_piano_broken().unwrap();
    assert!(broken_voiced.is_piano_voicing());
    assert_eq!(broken_voiced.piano_voicing_type(), Some(&PianoVoicingType::Broken));
}

#[test]
fn test_chord_piano_voicing_with_config() {
    let chord = Chord::major(note!("C"));

    // Test left hand configuration
    let left_config = PianoVoicingConfig::left_hand();
    let left_voiced = chord.voice_piano_with_config(&left_config).unwrap();
    assert!(left_voiced.is_piano_voicing());
    assert_eq!(left_voiced.piano_hand_position(), Some(&PianoHandPosition::LeftHand));

    // Test right hand configuration
    let right_config = PianoVoicingConfig::right_hand();
    let right_voiced = chord.voice_piano_with_config(&right_config).unwrap();
    assert!(right_voiced.is_piano_voicing());
    assert_eq!(right_voiced.piano_hand_position(), Some(&PianoHandPosition::RightHand));

}

#[test]
fn test_piano_voicing_details() {
    let chord = Chord::major(note!("C"));
    let voiced = chord.voice_piano_block().unwrap();

    // Check that piano-specific details are present
    assert!(voiced.is_piano_voicing());
    assert!(voiced.piano_hand_position().is_some());
    assert!(voiced.piano_voicing_type().is_some());

    // Check that guitar methods return None for piano voicing
    assert!(voiced.guitar_fingering().is_none());
    assert!(voiced.guitar_tuning().is_none());
}

#[test]
fn test_piano_voicing_ergonomic_adjustments() {
    let chord = Chord::major(note!("C"));
    let _config = PianoVoicingConfig::left_hand();

    // Create a voicer with small hand span
    let mut small_config = PianoVoicingConfig::left_hand();
    small_config.hand_span = PianoHandSpan::small_hands();

    let voiced = chord.voice_piano_with_config(&small_config).unwrap();

    // The voicing should be adjusted to be ergonomic for small hands
    let pitches = &voiced.pitches;
    assert!(pitches.len() <= 3); // Small hands max notes

    let span = (pitches.last().unwrap().midi_number() - pitches.first().unwrap().midi_number()) as u8;
    assert!(span <= 8); // Small hands max span
}

#[test]
fn test_piano_voicing_types_display() {
    assert_eq!(PianoVoicingType::Block.to_string(), "Block");
    assert_eq!(PianoVoicingType::Broken.to_string(), "Broken");
    assert_eq!(PianoVoicingType::Arpeggiated.to_string(), "Arpeggiated");
    assert_eq!(PianoVoicingType::Spread.to_string(), "Spread");
    assert_eq!(PianoVoicingType::Shell.to_string(), "Shell");
    assert_eq!(PianoVoicingType::Rootless.to_string(), "Rootless");
}

#[test]
fn test_piano_hand_positions_display() {
    assert_eq!(PianoHandPosition::LeftHand.to_string(), "Left Hand");
    assert_eq!(PianoHandPosition::RightHand.to_string(), "Right Hand");
    assert_eq!(PianoHandPosition::BothHands.to_string(), "Both Hands");
}

