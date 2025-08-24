//! Example demonstrating how to access guitar fingering details from voiced chords
//! 
//! This example shows how composition apps can extract guitar chord diagram 
//! information from the chordy library's voicing results.

use chordy::prelude::*;
use chordy::{GuitarFingering, StringState};

/// Generate an ASCII chord diagram for guitar fingering
fn generate_chord_diagram(fingering: &GuitarFingering, chord_name: &str) -> String {
    let mut diagram = String::new();
    
    // Get the fret range to display
    let fretted_positions: Vec<u8> = fingering.frets.iter()
        .filter_map(|s| match s { StringState::Fretted(f) => Some(*f), _ => None })
        .collect();
    
    let (start_fret, end_fret) = if fretted_positions.is_empty() {
        (0, 4) // Show first 4 frets for open chords
    } else {
        let min_fret = *fretted_positions.iter().min().unwrap();
        let max_fret = *fretted_positions.iter().max().unwrap();
        
        if min_fret == 0 {
            (0, (max_fret + 1).max(4)) // Include open strings, show at least 4 frets
        } else {
            // For barre chords, show from min_fret to max_fret + 1
            (min_fret.saturating_sub(1).max(1), max_fret + 1)
        }
    };
    
    diagram.push_str(&format!("    {} Guitar Chord Diagram\n", chord_name));
    
    // String names (from high E to low E for standard display)
    let string_names = ["e", "B", "G", "D", "A", "E"];
    
    // Draw each string (from high E to low E)
    for string_idx in (0..6).rev() {
        let string_name = string_names[5 - string_idx];
        
        // Handle different string states at the nut
        match &fingering.frets[string_idx] {
            StringState::Muted => {
                diagram.push_str(&format!("{} x||", string_name));
            }
            StringState::Open => {
                diagram.push_str(&format!("{} O||", string_name));
            }
            StringState::Fretted(_) => {
                diagram.push_str(&format!("{} -||", string_name));
            }
        }
        
        // Draw frets
        for fret in start_fret..=end_fret {
            match &fingering.frets[string_idx] {
                StringState::Muted => diagram.push_str("---|"),
                StringState::Open => diagram.push_str("---|"),
                StringState::Fretted(f) if *f == fret => diagram.push_str("-O-|"),
                _ => diagram.push_str("---|"),
            }
        }
        
        diagram.push('\n');
    }
    
    // Add fret numbers at bottom - align with fret positions
    diagram.push_str("     ");  // Align with string start position
    for fret in start_fret..=end_fret {
        diagram.push_str(&format!("{:^4}", fret));  // Center fret number in 4-char width to match "---|" spacing
    }
    
    diagram
}

fn main() {
    println!("=== Guitar Voicing Details Example ===\n");
    
    // Create some chords to voice
    let chords = [
        ("C Major", Chord::major(note!("C"))),
        ("G Major", Chord::major(note!("G"))),
        ("A Minor", Chord::minor(note!("A"))),
        ("D Major", Chord::major(note!("D"))),
        ("E7", Chord::dominant_7th(note!("E"))),
        ("F Major", Chord::major(note!("F"))),
        ("C/E (slash chord)", Chord::major(note!("C")).with_slash_bass(note!("E"))),
    ];
    
    for (name, chord) in &chords {
        println!("--- {} ---", name);
        
        // Voice the chord for guitar
        match chord.voice_for_guitar() {
            Ok(voiced) => {
                // Check if this voicing has guitar-specific details
                if voiced.is_guitar_voicing() {
                    // Extract the guitar fingering information
                    let fingering = voiced.guitar_fingering().unwrap();
                    let tuning = voiced.guitar_tuning().unwrap();
                    
                    println!("  Chord: {}", chord);
                    println!("  Fingering: {}", fingering);
                    println!("  Root string: {} (0=low E, 1=A, 2=D, 3=G, 4=B, 5=high E)", fingering.root_string);
                    println!("  Tuning: {} {} {} {} {} {}", 
                        tuning.strings[0], tuning.strings[1], tuning.strings[2],
                        tuning.strings[3], tuning.strings[4], tuning.strings[5]);
                    
                    // Show the string states for chord diagram display
                    println!("  String states:");
                    for (i, state) in fingering.frets.iter().enumerate() {
                        let string_name = match i {
                            0 => "Low E (6th)",
                            1 => "A (5th)",
                            2 => "D (4th)", 
                            3 => "G (3rd)",
                            4 => "B (2nd)",
                            5 => "High E (1st)",
                            _ => "Unknown",
                        };
                        println!("    {}: {}", string_name, state);
                    }
                    
                    // Show the resulting pitches
                    println!("  Resulting pitches: {:?}", voiced.pitches);
                    
                    // Show additional chord information
                    println!("  Bass note: {}", voiced.bass_pitch().unwrap());
                    println!("  Soprano note: {}", voiced.soprano_pitch().unwrap());
                    println!("  Span: {} semitones", voiced.span_semitones());
                    
                    // Check if it's a barre chord
                    if fingering.is_barre() {
                        if let Some(barre_fret) = fingering.barre_fret() {
                            println!("  Barre chord at fret {}", barre_fret);
                        }
                    } else {
                        println!("  Open chord");
                    }
                    
                    // Display ASCII chord diagram
                    println!("\n{}", generate_chord_diagram(&fingering, name));
                    
                    // Example: Generate chord diagram data
                    println!("\n  Chord diagram data:");
                    println!("    Frets: {:?}", fingering.frets);
                    println!("    Root: String {}", fingering.root_string);
                    
                } else {
                    println!("  No guitar-specific details available");
                }
            }
            Err(e) => {
                println!("  Failed to voice chord: {:?}", e);
            }
        }
        
        println!();
    }
    
    // Example: Compare guitar vs. piano voicing
    println!("=== Comparing Guitar vs. Piano Voicing ===");
    let c_major = Chord::major(note!("C"));
    
    // Guitar voicing (with fingering details)
    let guitar_voiced = c_major.voice_for_guitar().unwrap();
    println!("Guitar voicing:");
    println!("  Has details: {}", guitar_voiced.has_voicing_details());
    println!("  Is guitar voicing: {}", guitar_voiced.is_guitar_voicing());
    println!("  Fingering: {:?}", guitar_voiced.guitar_fingering());
    
    if let Some(fingering) = guitar_voiced.guitar_fingering() {
        println!("\n{}", generate_chord_diagram(&fingering, "C Major"));
    }
    
    // Piano voicing (no fingering details)
    let piano_voiced = c_major.voice_closed("C4".parse().unwrap()).unwrap();
    println!("\nPiano voicing:");
    println!("  Has details: {}", piano_voiced.has_voicing_details());
    println!("  Is guitar voicing: {}", piano_voiced.is_guitar_voicing());
    println!("  Fingering: {:?}", piano_voiced.guitar_fingering());
    println!("  Pitches: {:?}", piano_voiced.pitches);
    
    println!("\n=== Usage in Composition Apps ===");
    println!("In a composition application, you can now:");
    println!("1. Voice chords with: chord.voice_for_guitar()");
    println!("2. Check if details exist: voiced.is_guitar_voicing()");
    println!("3. Extract fingering: voiced.guitar_fingering()");
    println!("4. Display chord diagrams using the fingering data");
    println!("5. Show fret numbers, string states, and root positions");
}
