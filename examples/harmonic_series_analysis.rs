//! Harmonic Series Analysis Example
//!
//! This example demonstrates the harmonic series utilities for the Pitch type,
//! which are fundamental to acoustics and timbre analysis.

use chordy::types::*;

fn main() {
    println!("=== Harmonic Series Analysis Examples ===\n");

    example1_basic_harmonic_series();
    example2_timbre_analysis();
    example3_fundamental_detection();
    example4_subharmonic_series();
    example5_harmonic_relationships();
}

/// Example 1: Basic Harmonic Series Generation
fn example1_basic_harmonic_series() {
    println!("Example 1: Basic Harmonic Series Generation");
    println!("--------------------------------------------");

    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);
    println!("Fundamental: {}", c2);

    // Generate first 8 harmonics
    let harmonics = c2.harmonics(8);
    println!("\nFirst 8 harmonics:");
    for (i, harmonic) in harmonics.iter().enumerate() {
        println!("  Harmonic {}: {}", i + 1, harmonic);
    }

    // Show the musical intervals created by the harmonic series
    println!("\nMusical intervals in harmonic series:");
    println!("  H1: Fundamental (unison)");
    println!("  H2: Octave above");
    println!("  H3: Perfect fifth above");
    println!("  H4: Two octaves above");
    println!("  H5: Major third above");
    println!("  H6: Perfect fifth above");
    println!("  H7: Minor seventh above");
    println!("  H8: Three octaves above");

    println!();
}

/// Example 2: Timbre Analysis
fn example2_timbre_analysis() {
    println!("Example 2: Timbre Analysis");
    println!("---------------------------");

    // Different instruments emphasize different harmonics
    let a4 = Pitch::new(Letter::A, Accidental::Natural, 4);
    println!("Fundamental: {}", a4);

    // Flute-like timbre (strong fundamental, weak upper harmonics)
    let flute_harmonics = a4.harmonics(4);
    println!("\nFlute-like timbre (strong fundamental, weak upper harmonics):");
    for harmonic in flute_harmonics {
        println!("  {}", harmonic);
    }

    // Trumpet-like timbre (strong upper harmonics)
    let trumpet_harmonics = a4.harmonics(8);
    println!("\nTrumpet-like timbre (strong upper harmonics):");
    for harmonic in trumpet_harmonics {
        println!("  {}", harmonic);
    }

    // Piano-like timbre (complex harmonic spectrum)
    let piano_harmonics = a4.harmonics(12);
    println!("\nPiano-like timbre (complex harmonic spectrum, first 12):");
    for harmonic in piano_harmonics {
        println!("  {}", harmonic);
    }

    println!();
}

/// Example 3: Fundamental Frequency Detection
fn example3_fundamental_detection() {
    println!("Example 3: Fundamental Frequency Detection");
    println!("-------------------------------------------");

    // Detect fundamental from overtones
    println!("Detecting fundamental from overtones:");

    // Case 1: G3 is the 3rd harmonic of C2
    let g3 = Pitch::new(Letter::G, Accidental::Natural, 3);
    let fundamental_c2 = g3.fundamental_of_harmonic(3);
    println!("  {} is the 3rd harmonic of {}", g3, fundamental_c2);

    // Case 2: E4 is the 5th harmonic of C2
    let e4 = Pitch::new(Letter::E, Accidental::Natural, 4);
    let fundamental_c2_again = e4.fundamental_of_harmonic(5);
    println!("  {} is the 5th harmonic of {}", e4, fundamental_c2_again);

    // Case 3: A5 is the 2nd harmonic of A4
    let a5 = Pitch::new(Letter::A, Accidental::Natural, 5);
    let fundamental_a4 = a5.fundamental_of_harmonic(2);
    println!("  {} is the 2nd harmonic of {}", a5, fundamental_a4);

    // Real-world application: identifying missing fundamentals
    println!("\nMissing fundamental phenomenon:");
    println!("  When you hear harmonics 2, 3, 4, 5 of C2 (C3, G3, C4, E4),");
    println!("  your brain perceives the missing fundamental C2 even if it's not physically present.");

    println!();
}

/// Example 4: Subharmonic Series
fn example4_subharmonic_series() {
    println!("Example 4: Subharmonic Series");
    println!("------------------------------");

    let c4 = Pitch::new(Letter::C, Accidental::Natural, 4);
    println!("Fundamental: {}", c4);

    // Generate first 5 subharmonics (avoiding frequencies that are too low)
    println!("\nFirst 5 subharmonics:");
    for n in 1..=5 {
        let subharmonic = c4.subharmonic(n);
        println!("  Subharmonic {}: {}", n, subharmonic);
    }

    // Show the musical intervals created by the subharmonic series
    println!("\nMusical intervals in subharmonic series:");
    println!("  S1: Fundamental (unison)");
    println!("  S2: Octave below");
    println!("  S3: Perfect fifth below");
    println!("  S4: Two octaves below");
    println!("  S5: Major third below");

    println!();
}

/// Example 5: Harmonic Relationships in Music Theory
fn example5_harmonic_relationships() {
    println!("Example 5: Harmonic Relationships in Music Theory");
    println!("---------------------------------------------------");

    // Why perfect intervals sound consonant
    let c2 = Pitch::new(Letter::C, Accidental::Natural, 2);

    println!("Harmonic relationships explain why certain intervals sound consonant:");

    // Perfect fifth (2:3 ratio)
    let g2 = c2.harmonic(3); // G2 is the 3rd harmonic of C2
    println!("  Perfect fifth (C2-G2): {} is the 3rd harmonic of {}", g2, c2);

    // Perfect fourth (3:4 ratio)
    let f2 = c2.subharmonic(3); // F2 is the 3rd subharmonic of C2
    println!("  Perfect fourth (F2-C2): {} is the 3rd subharmonic of {}", f2, c2);

    // Major third (4:5 ratio)
    let e2 = c2.harmonic(5); // E2 is the 5th harmonic of C2
    println!("  Major third (C2-E2): {} is the 5th harmonic of {}", e2, c2);

    // Minor third (5:6 ratio)
    let eb2 = Pitch::new(Letter::E, Accidental::Flat, 2);
    println!("  Minor third (C2-Eb2): Simple 5:6 frequency ratio");

    println!("\nThese simple integer ratios are why these intervals sound harmonious!");
    println!();
}