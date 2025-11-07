//! Demo program showing piano voicing for extended chords

use chordy::{Chord, note, VoiceLeader, VoiceLeadingStyle, VoicingConfig};

fn main() {
    println!("Testing piano voicing for extended chords...\n");

    // Test various 7th chords that are directly available
    let chords = vec![
        ("A minor 7", Chord::minor_7th(note!("A"))),
        ("F major 7", Chord::major_7th(note!("F"))),
        ("G dominant 7", Chord::dominant_7th(note!("G"))),
        ("C minor 7", Chord::minor_7th(note!("C"))),
        ("D dominant 7", Chord::dominant_7th(note!("D"))),
        ("E minor 7", Chord::minor_7th(note!("E"))),
        ("B flat major 7", Chord::major_7th(note!("Bb"))),
        ("A dominant 7", Chord::dominant_7th(note!("A"))),
        ("C minor-major 7", Chord::minor_major_7th(note!("C"))),
        ("F minor 7 flat 5", Chord::minor_7th_flat_5(note!("F"))),
        // Extended chords - 9ths
        ("C major 9", Chord::major_9th(note!("C"))),
        ("D minor 9", Chord::minor_9th(note!("D"))),
        ("G dominant 9", Chord::dominant_9th(note!("G"))),
        ("F major 9", Chord::major_9th(note!("F"))),
        // Extended chords - 11ths
        ("A minor 11", Chord::minor_11th(note!("A"))),
        ("C major 11", Chord::major_11th(note!("C"))),
        ("G dominant 11", Chord::dominant_11th(note!("G"))),
        // Extended chords - 13ths
        ("F major 13", Chord::major_13th(note!("F"))),
        ("D minor 13", Chord::minor_13th(note!("D"))),
        ("C dominant 13", Chord::dominant_13th(note!("C"))),
        // Altered dominants
        ("G7♭9", Chord::dominant_7th_flat_9(note!("G"))),
        ("G7♯9", Chord::dominant_7th_sharp_9(note!("G"))),
        ("G7♭13", Chord::dominant_7th_flat_13(note!("G"))),
    ];

    for (name, chord) in chords {
        println!("{}:", name);
        println!("  Chord structure: {}", chord);

        // Test block voicing
        if let Ok(block_voiced) = chord.voice_piano_block() {
            println!("  Block voicing: {:?}", block_voiced.pitches);
        }

        // Test spread voicing
        if let Ok(spread_voiced) = chord.voice_piano_spread() {
            println!("  Spread voicing: {:?}", spread_voiced.pitches);
        }

        // Test jazz voicing (shell)
        if let Ok(jazz_voiced) = chord.voice_piano_jazz() {
            println!("  Jazz voicing: {:?}", jazz_voiced.pitches);
        }

        // Test rootless voicing
        if let Ok(rootless_voiced) = chord.voice_piano_rootless() {
            println!("  Rootless voicing: {:?}", rootless_voiced.pitches);
        }

        // Test broken voicing
        if let Ok(broken_voiced) = chord.voice_piano_broken() {
            println!("  Broken voicing: {:?}", broken_voiced.pitches);
        }

        println!();
    }

    // Test voice leading with a progression
    println!("\n=== Voice Leading Demo ===\n");

    let config = VoicingConfig::new().range_from("C3".parse().unwrap(), "C6".parse().unwrap());

    // Test jazz voice leading
    let jazz_leader = VoiceLeader::new(config.clone()).style(VoiceLeadingStyle::Jazz);

    let progression = vec![
        Chord::minor_7th(note!("D")), // ii7
        Chord::dominant_7th(note!("G")), // V7
        Chord::major_7th(note!("C")), // Imaj7
    ];

    println!("Jazz voice leading (ii7-V7-Imaj7):");
    if let Ok(voiced_progression) = jazz_leader.voice_progression(&progression) {
        for (i, voiced) in voiced_progression.iter().enumerate() {
            println!("  {}: {:?}", i + 1, voiced.pitches);
            if let Some(movement) = voiced.info.movement {
                println!("    Voice movement: {} semitones", movement);
            }
        }
    }

    // Test common practice voice leading
    let cp_leader = VoiceLeader::new(config.clone()).style(VoiceLeadingStyle::CommonPractice);

    let simple_progression = vec![
        Chord::major(note!("C")),
        Chord::major(note!("F")),
        Chord::major(note!("G")),
        Chord::major(note!("C")),
    ];

    println!("\nCommon practice voice leading (I-IV-V-I):");
    if let Ok(voiced_progression) = cp_leader.voice_progression(&simple_progression) {
        for (i, voiced) in voiced_progression.iter().enumerate() {
            println!("  {}: {:?}", i + 1, voiced.pitches);
            if let Some(movement) = voiced.info.movement {
                println!("    Voice movement: {} semitones", movement);
            }
        }
    }

    println!("\nExtended chord voicing demo complete!");
}