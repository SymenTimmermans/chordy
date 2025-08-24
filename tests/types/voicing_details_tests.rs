//! Tests for the VoicingDetails system that includes instrument-specific metadata

use chordy::prelude::*;
use chordy::{VoicingDetails, VoicingStyle, GuitarTuning};

#[test]
fn test_guitar_voicing_includes_fingering_details() {
    let c_major = Chord::major(note!("C"));
    
    // Voice the chord for guitar
    let voiced = c_major.voice_for_guitar().unwrap();
    
    // Should be a guitar voicing
    assert!(voiced.is_guitar_voicing());
    assert!(voiced.has_voicing_details());
    assert_eq!(voiced.info.style, VoicingStyle::Guitar);
    
    // Should have guitar fingering information
    let fingering = voiced.guitar_fingering().expect("Should have guitar fingering");
    let tuning = voiced.guitar_tuning().expect("Should have guitar tuning");
    
    // Verify fingering produces the correct pitches
    let fingering_pitches = fingering.to_pitches(tuning);
    assert_eq!(fingering_pitches, voiced.pitches, "Fingering should produce the voiced pitches");
    
    // Should use standard tuning
    assert_eq!(*tuning, GuitarTuning::standard());
    
    // Fingering should be a valid guitar fingering (6 strings)
    assert_eq!(fingering.frets.len(), 6, "Should have 6 strings");
    
    println!("C major guitar voicing:");
    println!("  Fingering: {}", fingering);
    println!("  Pitches: {:?}", voiced.pitches);
    println!("  Details: {:?}", voiced.info.details);
}

#[test]
fn test_non_guitar_voicing_has_no_fingering_details() {
    let c_major = Chord::major(note!("C"));
    
    // Voice the chord for piano (closed voicing)
    let voiced = c_major.voice_closed("C4".parse().unwrap()).unwrap();
    
    // Should not be a guitar voicing
    assert!(!voiced.is_guitar_voicing());
    assert!(!voiced.has_voicing_details());
    assert_eq!(voiced.info.style, VoicingStyle::Closed);
    
    // Should have no guitar fingering information
    assert!(voiced.guitar_fingering().is_none());
    assert!(voiced.guitar_tuning().is_none());
    
    // Details should be None
    assert!(voiced.info.details.is_none());
}

#[test]
fn test_multiple_guitar_chords_have_different_fingerings() {
    let chords = [
        Chord::major(note!("C")),
        Chord::major(note!("G")),
        Chord::minor(note!("A")),
        Chord::major(note!("D")),
    ];
    
    let mut fingerings = Vec::new();
    
    for chord in &chords {
        let voiced = chord.voice_for_guitar().unwrap();
        
        // Each should be a guitar voicing
        assert!(voiced.is_guitar_voicing());
        
        let fingering = voiced.guitar_fingering().unwrap();
        fingerings.push(fingering.clone());
        
        println!("{} guitar voicing: {}", chord, fingering);
        
        // Verify the fingering produces the correct pitches
        let tuning = voiced.guitar_tuning().unwrap();
        let fingering_pitches = fingering.to_pitches(tuning);
        assert_eq!(fingering_pitches, voiced.pitches);
    }
    
    // All fingerings should be different (different chord shapes)
    for (i, fingering1) in fingerings.iter().enumerate() {
        for (j, fingering2) in fingerings.iter().enumerate() {
            if i != j {
                assert_ne!(fingering1, fingering2, "Different chords should have different fingerings");
            }
        }
    }
}

#[test]
fn test_voicing_details_enum_variants() {
    // Test that we can construct different types of voicing details
    let guitar_details = VoicingDetails::Guitar {
        fingering: "032010,1".parse().unwrap(),
        tuning: GuitarTuning::standard(),
    };
    
    let piano_details = VoicingDetails::Piano {
        hand_position: "RH: C4-E4-G4, LH: C3".to_string(),
    };
    
    let generic_details = VoicingDetails::Generic;
    
    // Test pattern matching
    match guitar_details {
        VoicingDetails::Guitar { fingering, tuning } => {
            assert_eq!(fingering.root_string, 1);
            assert_eq!(tuning, GuitarTuning::standard());
        }
        _ => panic!("Expected guitar details"),
    }
    
    match piano_details {
        VoicingDetails::Piano { hand_position } => {
            assert!(hand_position.contains("RH:"));
        }
        _ => panic!("Expected piano details"),
    }
    
    match generic_details {
        VoicingDetails::Generic => {
            // Success
        }
        _ => panic!("Expected generic details"),
    }
}

#[test]
fn test_voicing_info_with_details() {
    use chordy::{VoicingInfo, VoicingStyle, PitchRange};
    
    // Test creating VoicingInfo with details
    let details = VoicingDetails::Guitar {
        fingering: "032010,1".parse().unwrap(),
        tuning: GuitarTuning::standard(),
    };
    
    let info = VoicingInfo::new_with_details(
        VoicingStyle::Guitar,
        PitchRange::guitar(),
        0,
        details.clone(),
    );
    
    assert_eq!(info.style, VoicingStyle::Guitar);
    assert_eq!(info.details, Some(details));
    
    // Test adding details to existing VoicingInfo
    let mut info2 = VoicingInfo::new(VoicingStyle::Guitar, PitchRange::guitar(), 0);
    assert!(info2.details.is_none());
    
    info2 = info2.with_details(VoicingDetails::Generic);
    assert_eq!(info2.details, Some(VoicingDetails::Generic));
}

#[test]
fn test_slash_chord_guitar_voicing_details() {
    // Test that slash chords also get proper fingering details
    let c_over_e = Chord::major(note!("C")).with_slash_bass(note!("E"));
    
    let voiced = c_over_e.voice_for_guitar().unwrap();
    
    assert!(voiced.is_guitar_voicing());
    
    let fingering = voiced.guitar_fingering().unwrap();
    let tuning = voiced.guitar_tuning().unwrap();
    
    // Verify the bass note is E
    let pitches = fingering.to_pitches(tuning);
    assert_eq!(voiced.chord.bass_note(), note!("E"));
    
    // The lowest pitch should be E (bass note)
    let bass_pitch = voiced.bass_pitch().unwrap();
    assert_eq!(bass_pitch.name, note!("E"));
    
    println!("C/E guitar voicing: {}", fingering);
    println!("Pitches: {:?}", pitches);
}