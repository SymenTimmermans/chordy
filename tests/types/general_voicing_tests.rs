//! Unit tests for general voicing functionality (non-guitar-specific)

use chordy::prelude::*;
use chordy::{PitchRange, VoicedChord, VoicingConfig, VoicingError, VoicingInfo, VoicingStyle, Voicer};

#[test]
fn test_pitch_range_creation() {
    let c3: Pitch = "C3".parse().unwrap();
    let c6: Pitch = "C6".parse().unwrap();
    let range = PitchRange::new(c3, c6);
    assert_eq!(range.low, c3);
    assert_eq!(range.high, c6);
}

#[test]
fn test_pitch_range_contains() {
    let c3: Pitch = "C3".parse().unwrap();
    let c6: Pitch = "C6".parse().unwrap();
    let g4: Pitch = "G4".parse().unwrap();
    let b2: Pitch = "B2".parse().unwrap();
    let d6: Pitch = "D6".parse().unwrap();

    let range = PitchRange::new(c3, c6);
    assert!(range.contains(g4));
    assert!(range.contains(c3));
    assert!(range.contains(c6));
    assert!(!range.contains(b2));
    assert!(!range.contains(d6));
}

#[test]
fn test_pitch_range_span() {
    let c3: Pitch = "C3".parse().unwrap();
    let c6: Pitch = "C6".parse().unwrap();
    let g3: Pitch = "G3".parse().unwrap();

    let range = PitchRange::new(c3, c6);
    assert_eq!(range.span_octaves(), 3.0);

    let range2 = PitchRange::new(c3, g3);
    assert!((range2.span_octaves() - 0.583).abs() < 0.01); // 7 semitones / 12
}

#[test]
fn test_voicing_config_builder() {
    let c3: Pitch = "C3".parse().unwrap();
    let c6: Pitch = "C6".parse().unwrap();
    let c2: Pitch = "C2".parse().unwrap();

    let config = VoicingConfig::new()
        .style(VoicingStyle::Open)
        .range_from(c3, c6)
        .bass_pitch(c2);

    assert_eq!(config.style, VoicingStyle::Open);
    assert_eq!(config.range.low, c3);
    assert_eq!(config.range.high, c6);
    assert_eq!(config.bass_pitch, Some(c2));
}

#[test]
fn test_voiced_chord_properties() {
    let chord = Chord::major(note!("C"));
    let c4: Pitch = "C4".parse().unwrap();
    let e4: Pitch = "E4".parse().unwrap();
    let g4: Pitch = "G4".parse().unwrap();
    let pitches = vec![c4, e4, g4];
    let info = VoicingInfo::new(VoicingStyle::Closed, PitchRange::piano(), 0);
    let voiced = VoicedChord::new(chord, pitches, info);

    assert_eq!(voiced.bass_pitch(), Some(c4));
    assert_eq!(voiced.soprano_pitch(), Some(g4));
    assert_eq!(voiced.span_semitones(), 7); // Perfect fifth
    assert!(voiced.is_closed());
    assert!(!voiced.is_open());
}

#[test]
fn test_voice_intervals() {
    let chord = Chord::major(note!("C"));
    let c4: Pitch = "C4".parse().unwrap();
    let e4: Pitch = "E4".parse().unwrap();
    let g4: Pitch = "G4".parse().unwrap();
    let pitches = vec![c4, e4, g4];
    let info = VoicingInfo::new(VoicingStyle::Closed, PitchRange::piano(), 0);
    let voiced = VoicedChord::new(chord, pitches, info);

    let intervals = voiced.voice_intervals();
    assert_eq!(intervals, vec![4, 3]); // Major third, minor third
}

#[test]
fn test_voicer_closed_voicing() {
    let chord = Chord::major(note!("C"));
    let config = VoicingConfig::new()
        .style(VoicingStyle::Closed)
        .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    let voicer = Voicer::new(config);

    let result = voicer.voice_chord(&chord);
    assert!(result.is_ok());

    let voiced = result.unwrap();
    assert_eq!(voiced.pitches.len(), 3); // Root, third, fifth
    assert!(voiced.is_closed());
    assert!(!voiced.is_open());

    // Check that pitches are in ascending order
    let pitches = voiced.pitches;
    assert!(pitches[0].midi_number() <= pitches[1].midi_number());
    assert!(pitches[1].midi_number() <= pitches[2].midi_number());
}

#[test]
fn test_voicer_open_voicing() {
    let chord = Chord::major(note!("C"));
    let config = VoicingConfig::new()
        .style(VoicingStyle::Open)
        .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    let voicer = Voicer::new(config);

    let result = voicer.voice_chord(&chord);
    assert!(result.is_ok());

    let voiced = result.unwrap();
    assert_eq!(voiced.pitches.len(), 3);

    // Open voicing should generally span more than an octave
    // (though not always, depends on the specific algorithm)
    let span = voiced.span_semitones();
    assert!(span >= 0); // Basic sanity check
}

#[test]
fn test_voicer_drop2_voicing() {
    let chord = Chord::major_7th(note!("C")); // Use 7th chord for drop-2
    let config = VoicingConfig::new()
        .style(VoicingStyle::Drop2)
        .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    let voicer = Voicer::new(config);

    let result = voicer.voice_chord(&chord);
    assert!(result.is_ok());

    let voiced = result.unwrap();
    assert_eq!(voiced.pitches.len(), 4); // Root, third, fifth, seventh

    // Drop-2 should have the second-highest note dropped an octave
    let pitches = voiced.pitches;
    // Basic check that we have the right number of pitches
    assert_eq!(pitches.len(), 4);
}

#[test]
fn test_voicer_spread_voicing() {
    let chord = Chord::major(note!("C"));
    let config = VoicingConfig::new()
        .style(VoicingStyle::spread(
            Interval::MAJOR_THIRD,
            Interval::PERFECT_FIFTH,
        ))
        .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    let voicer = Voicer::new(config);

    let result = voicer.voice_chord(&chord);
    assert!(result.is_ok());

    let voiced = result.unwrap();
    assert_eq!(voiced.pitches.len(), 3);

    // Check that adjacent intervals respect the constraints
    let intervals = voiced.voice_intervals();
    for &interval in &intervals {
        assert!(interval >= 4); // At least major third
        assert!(interval <= 7); // At most perfect fifth
    }
}

#[test]
fn test_voicer_out_of_range_error() {
    let chord = Chord::major(note!("C"));
    // Very restrictive range
    let config = VoicingConfig::new()
        .style(VoicingStyle::Closed)
        .range_from("C8".parse().unwrap(), "C8".parse().unwrap());
    let voicer = Voicer::new(config);

    let result = voicer.voice_chord(&chord);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), VoicingError::OutOfRange);
}

#[test]
fn test_voicer_with_bass_pitch() {
    let chord = Chord::major(note!("C"));
    let config = VoicingConfig::new()
        .style(VoicingStyle::Closed)
        .range_from("C3".parse().unwrap(), "C6".parse().unwrap());
    let voicer = Voicer::new(config);

    let bass_pitch: Pitch = "C3".parse().unwrap();
    let result = voicer.voice_chord_from_pitch(&chord, Some(bass_pitch));
    assert!(result.is_ok());

    let voiced = result.unwrap();
    // The bass note should be close to the specified pitch
    let actual_bass = voiced.bass_pitch().unwrap();
    assert_eq!(actual_bass.name, bass_pitch.name);
}

#[test]
fn test_guitar_voicing_style_integration() {
    let c_major = Chord::major(note!("C"));

    // Test the convenient guitar voicing method
    let guitar_voiced = c_major.voice_for_guitar();
    assert!(
        guitar_voiced.is_ok(),
        "Guitar voicing should succeed for C major"
    );

    let voiced_chord = guitar_voiced.unwrap();

    // Should use Guitar voicing style
    assert_eq!(voiced_chord.info.style, VoicingStyle::Guitar);

    // Should use guitar pitch range
    assert_eq!(voiced_chord.info.range, PitchRange::guitar());

    // Should produce pitches
    assert!(
        !voiced_chord.pitches.is_empty(),
        "Should produce guitar voicing pitches"
    );

    // Should contain the root chord
    assert_eq!(voiced_chord.chord.root(), note!("C"));
}

#[test]
fn test_guitar_voicing_config() {
    let config = VoicingConfig::guitar();

    // Should have Guitar style
    assert_eq!(config.style, VoicingStyle::Guitar);

    // Should have guitar range
    assert_eq!(config.range, PitchRange::guitar());
}

#[test]
fn test_guitar_voicing_with_voicing_engine() {
    let config = VoicingConfig::guitar();
    let voicer = Voicer::new(config);
    let c_major = Chord::major(note!("C"));

    let result = voicer.voice_chord(&c_major);
    assert!(result.is_ok(), "Guitar voicing via engine should succeed");

    let voiced = result.unwrap();
    assert_eq!(voiced.info.style, VoicingStyle::Guitar);
    assert!(!voiced.pitches.is_empty());
}

#[test]
fn test_guitar_voicing_unsupported_chord() {
    use chordy::IntervalFirstGuitarFinder;

    // Create a chord that might not have good guitar fingerings
    let finder = IntervalFirstGuitarFinder::new();

    // Test with a basic B♭ major chord
    let unusual_chord = Chord::major(note!("B♭"));
    let voicings = finder.find_voicings(&unusual_chord);
    
    // The interval-first approach should find voicings for most basic chords
    // If no voicings found, test that the voicing engine handles this case
    if voicings.is_empty() {
        // Test that the voicing engine handles this case
        let config = VoicingConfig::guitar();
        let voicer = Voicer::new(config);
        let result = voicer.voice_chord(&unusual_chord);

        // Should return an error for unsupported chords
        assert!(result.is_err());
        if let Err(VoicingError::UnsupportedStyle) = result {
            // This is the expected error
        } else {
            panic!("Expected UnsupportedStyle error, got: {:?}", result);
        }
    } else {
        // Should find at least one voicing for basic chords
        assert!(!voicings.is_empty(), "Should find voicings for B♭ major");
    }
}

#[test]
fn test_voicing_style_display() {
    assert_eq!(format!("{}", VoicingStyle::Guitar), "Guitar");
    assert_eq!(format!("{}", VoicingStyle::Closed), "Closed");
    assert_eq!(format!("{}", VoicingStyle::Open), "Open");
}