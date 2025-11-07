//! Frequency Ratios Example
//!
//! This example demonstrates the frequency ratio utilities for the Interval type,
//! showing how different tuning systems represent musical intervals.

use chordy::types::*;

fn main() {
    println!("=== Frequency Ratios Example ===\n");

    example1_equal_temperament();
    example2_just_intonation();
    example3_pythagorean_tuning();
    example4_ratio_matching();
    example5_tuning_comparison();
}

/// Example 1: Equal Temperament Ratios
fn example1_equal_temperament() {
    println!("Example 1: Equal Temperament Ratios");
    println!("-----------------------------------");

    // Equal temperament divides the octave into 12 equal semitones
    // Each semitone has a frequency ratio of 2^(1/12)
    let intervals = [
        ("Perfect unison", Interval::PERFECT_UNISON),
        ("Minor second", Interval::MINOR_SECOND),
        ("Major second", Interval::MAJOR_SECOND),
        ("Minor third", Interval::MINOR_THIRD),
        ("Major third", Interval::MAJOR_THIRD),
        ("Perfect fourth", Interval::PERFECT_FOURTH),
        ("Tritone", Interval::AUGMENTED_FOURTH),
        ("Perfect fifth", Interval::PERFECT_FIFTH),
        ("Minor sixth", Interval::MINOR_SIXTH),
        ("Major sixth", Interval::MAJOR_SIXTH),
        ("Minor seventh", Interval::MINOR_SEVENTH),
        ("Major seventh", Interval::MAJOR_SEVENTH),
        ("Octave", Interval::OCTAVE),
    ];

    for (name, interval) in intervals {
        let ratio = interval.frequency_ratio();
        let cents = interval.cents();
        println!("  {:15} | Ratio: {:.4} | Cents: {:4.0}", name, ratio, cents);
    }

    println!();
}

/// Example 2: Just Intonation Ratios
fn example2_just_intonation() {
    println!("Example 2: Just Intonation Ratios");
    println!("---------------------------------");

    // Just intonation uses simple integer ratios for pure consonance
    let intervals = [
        ("Perfect unison", Interval::PERFECT_UNISON),
        ("Perfect fifth", Interval::PERFECT_FIFTH),
        ("Perfect fourth", Interval::PERFECT_FOURTH),
        ("Major third", Interval::MAJOR_THIRD),
        ("Minor third", Interval::MINOR_THIRD),
        ("Major sixth", Interval::MAJOR_SIXTH),
        ("Minor sixth", Interval::MINOR_SIXTH),
    ];

    for (name, interval) in intervals {
        let (num, den) = interval.just_intonation_ratio();
        let ratio = num as f32 / den as f32;
        println!("  {:15} | Ratio: {}/{} = {:.4}", name, num, den, ratio);
    }

    println!();
}

/// Example 3: Pythagorean Tuning Ratios
fn example3_pythagorean_tuning() {
    println!("Example 3: Pythagorean Tuning Ratios");
    println!("------------------------------------");

    // Pythagorean tuning uses pure fifths (3:2) to derive all intervals
    let intervals = [
        ("Perfect unison", Interval::PERFECT_UNISON),
        ("Perfect fifth", Interval::PERFECT_FIFTH),
        ("Perfect fourth", Interval::PERFECT_FOURTH),
        ("Major third", Interval::MAJOR_THIRD),
        ("Minor third", Interval::MINOR_THIRD),
        ("Major sixth", Interval::MAJOR_SIXTH),
        ("Minor sixth", Interval::MINOR_SIXTH),
    ];

    for (name, interval) in intervals {
        let (num, den) = interval.pythagorean_ratio();
        let ratio = num as f32 / den as f32;
        println!("  {:15} | Ratio: {}/{} = {:.4}", name, num, den, ratio);
    }

    println!();
}

/// Example 4: Ratio Matching
fn example4_ratio_matching() {
    println!("Example 4: Ratio Matching");
    println!("--------------------------");

    // Identify intervals from frequency ratios
    let ratios = [
        ("Unison", 1.0),
        ("Octave", 2.0),
        ("Perfect fifth", 1.5),
        ("Perfect fourth", 1.333),
        ("Major third", 1.25),
        ("Minor third", 1.2),
        ("Major second", 1.125),
    ];

    for (name, ratio) in ratios {
        if let Some(interval) = Interval::from_ratio(ratio) {
            println!("  Ratio {:.3} ({:15}) → {}", ratio, name, interval);
        } else {
            println!("  Ratio {:.3} ({:15}) → No match", ratio, name);
        }
    }

    println!();
}

/// Example 5: Tuning System Comparison
fn example5_tuning_comparison() {
    println!("Example 5: Tuning System Comparison");
    println!("-----------------------------------");

    // Compare how different tuning systems represent the same interval
    let intervals = [
        ("Major third", Interval::MAJOR_THIRD),
        ("Perfect fifth", Interval::PERFECT_FIFTH),
        ("Perfect fourth", Interval::PERFECT_FOURTH),
    ];

    for (name, interval) in intervals {
        let equal_temp = interval.frequency_ratio();
        let (just_num, just_den) = interval.just_intonation_ratio();
        let just_ratio = just_num as f32 / just_den as f32;
        let (pyth_num, pyth_den) = interval.pythagorean_ratio();
        let pyth_ratio = pyth_num as f32 / pyth_den as f32;

        println!("  {:15}:", name);
        println!("    Equal temperament: {:.4}", equal_temp);
        println!("    Just intonation:   {}/{} = {:.4}", just_num, just_den, just_ratio);
        println!("    Pythagorean:       {}/{} = {:.4}", pyth_num, pyth_den, pyth_ratio);

        // Show cents deviation from equal temperament
        let just_cents = 1200.0 * (just_ratio / equal_temp).log2();
        let pyth_cents = 1200.0 * (pyth_ratio / equal_temp).log2();
        println!("    Cents deviation:   Just: {:+.1}, Pythagorean: {:+.1}", just_cents, pyth_cents);
        println!();
    }
}