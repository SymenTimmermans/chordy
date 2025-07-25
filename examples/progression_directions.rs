//! Example demonstrating bidirectional chord progression analysis
//!
//! Shows both progressions_from (where can I go?) and progressions_to (what leads here?)

use chordy::prelude::*;

fn main() {
    let c_major = Key::Major(note!("C"));
    let tonic = Chord::major(note!("C")); // I chord
    let dominant = Chord::major(note!("G")); // V chord

    println!("=== C Major Key Progression Analysis ===\n");

    // What can lead TO the tonic (I chord)?
    println!("Progressions TO I (Tonic Resolution):");
    if let Some(to_tonic) = c_major.progressions_to(&tonic) {
        print_progression_options(&to_tonic);

        // Check if V is a strong resolution to I
        let v_resolves_strong = to_tonic.strong.iter().any(|chord| {
            chord.root == note!("G") && chord.intervals.contains(Interval::MAJOR_THIRD)
        });

        if v_resolves_strong {
            println!("✓ V strongly resolves to I (dominant resolution)");
        }
    }

    println!();

    // What can we go TO FROM the tonic?
    println!("Progressions FROM I (Tonic Departure):");
    if let Some(from_tonic) = c_major.progressions_from(&tonic) {
        print_progression_options(&from_tonic);
    }

    println!();

    // What can lead TO the dominant (V chord)?
    println!("Progressions TO V (Pre-dominant function):");
    if let Some(to_dominant) = c_major.progressions_to(&dominant) {
        print_progression_options(&to_dominant);

        // Check for ii-V motion
        let ii_to_v_strong = to_dominant.strong.iter().any(|chord| {
            chord.root == note!("D") && chord.intervals.contains(Interval::MINOR_THIRD)
        });

        if ii_to_v_strong {
            println!("✓ ii strongly leads to V (pre-dominant function)");
        }
    }

    println!();

    // What can we go TO FROM the dominant?
    println!("Progressions FROM V (Dominant function):");
    if let Some(from_dominant) = c_major.progressions_from(&dominant) {
        print_progression_options(&from_dominant);

        // Check for V-I resolution
        let v_to_i_strong = from_dominant.strong.iter().any(|chord| {
            chord.root == note!("C") && chord.intervals.contains(Interval::MAJOR_THIRD)
        });

        if v_to_i_strong {
            println!("✓ V strongly resolves to I (tonic resolution)");
        }
    }
}

fn print_progression_options(options: &ChordProgressionOptions) {
    if !options.strong.is_empty() {
        println!("  Strong: {}", format_chord_list(&options.strong));
    }
    if !options.moderate.is_empty() {
        println!("  Moderate: {}", format_chord_list(&options.moderate));
    }
    if !options.weak.is_empty() {
        println!("  Weak: {}", format_chord_list(&options.weak));
    }
}

fn format_chord_list(chords: &[Chord]) -> String {
    chords
        .iter()
        .take(8) // Show first 8 to avoid overwhelming output
        .map(|c| c.to_html())
        .collect::<Vec<_>>()
        .join(", ")
}
