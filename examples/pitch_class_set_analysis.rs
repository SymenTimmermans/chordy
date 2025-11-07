/// Pitch Class Set Analysis Examples
///
/// This example demonstrates how to use PitchClassSet for analyzing harmonic structures
/// in both tonal and atonal music. It covers common use cases including chord identification,
/// set theory analysis, and comparing pitch collections.

use chordy::{note, PitchClassSet};

fn main() {
    println!("=== Pitch Class Set Analysis Examples ===\n");

    // Example 1: Basic Chord Recognition
    basic_chord_recognition();

    // Example 2: Set Theory Analysis
    set_theory_analysis();

    // Example 3: Comparing Chord Voicings
    comparing_voicings();

    // Example 4: Analyzing Complement Relationships
    complement_analysis();

    // Example 5: Interval Vector Analysis
    interval_vector_analysis();

    // Example 6: Finding Common Tones
    common_tone_analysis();
}

/// Example 1: Identify chord types from pitch collections
fn basic_chord_recognition() {
    println!("Example 1: Basic Chord Recognition");
    println!("-----------------------------------");

    let chords = vec![
        (vec![note!("C"), note!("E"), note!("G")], "C major triad"),
        (vec![note!("D"), note!("F"), note!("A")], "D minor triad"),
        (vec![ note!("C"), note!("Eb"), note!("Gb")], "C diminished triad"),
        (vec![note!("C"), note!("E"), note!("G#")], "C augmented triad"),
        (vec![note!("C"), note!("E"), note!("G"), note!("Bb")], "C dominant seventh"),
        (vec![note!("C"), note!("E"), note!("G"), note!("B")], "C major seventh"),
        (vec![note!("C"), note!("Eb"), note!("G"), note!("Bb")], "C minor seventh"),
    ];

    for (notes, name) in chords {
        let pc_set = PitchClassSet::new(&notes);

        print!("{}: ", name);

        if pc_set.is_major_triad() {
            println!("✓ Major triad");
        } else if pc_set.is_minor_triad() {
            println!("✓ Minor triad");
        } else if pc_set.is_diminished_triad() {
            println!("✓ Diminished triad");
        } else if pc_set.is_augmented_triad() {
            println!("✓ Augmented triad");
        } else if pc_set.is_dominant_seventh() {
            println!("✓ Dominant seventh");
        } else if pc_set.is_major_seventh() {
            println!("✓ Major seventh");
        } else if pc_set.is_minor_seventh() {
            println!("✓ Minor seventh");
        } else if pc_set.is_half_diminished_seventh() {
            println!("✓ Half-diminished seventh");
        } else if pc_set.is_fully_diminished_seventh() {
            println!("✓ Fully diminished seventh");
        } else {
            println!("✗ Unrecognized chord type");
        }
    }
    println!();
}

/// Example 2: Analyze pitch collections using set theory
fn set_theory_analysis() {
    println!("Example 2: Set Theory Analysis");
    println!("-------------------------------");

    // Analyze an atonal pitch collection
    let set = PitchClassSet::new(&[
        note!("C"), note!("C#"), note!("E"), note!("F#")
    ]);

    println!("Analyzing set: {}", set);
    println!("  Cardinality: {}", set.len());
    println!("  Normal form: {:?}", set.normal_form());
    println!("  Prime form: {:?}", set.prime_form());
    println!("  Interval vector: {:?}", set.interval_vector());

    // The interval vector tells us about the intervallic content
    let iv = set.interval_vector();
    println!("\nInterval vector breakdown:");
    println!("  Minor 2nds/Major 7ths (class 1): {}", iv[0]);
    println!("  Major 2nds/Minor 7ths (class 2): {}", iv[1]);
    println!("  Minor 3rds/Major 6ths (class 3): {}", iv[2]);
    println!("  Major 3rds/Minor 6ths (class 4): {}", iv[3]);
    println!("  Perfect 4ths/Perfect 5ths (class 5): {}", iv[4]);
    println!("  Tritones (class 6): {}", iv[5]);
    println!();
}

/// Example 3: Compare different spellings of the same chord
fn comparing_voicings() {
    println!("Example 3: Comparing Different Chord Spellings");
    println!("-----------------------------------------------");

    // Different orderings and enharmonic spellings all represent C major
    let spelling1 = PitchClassSet::new(&[
        note!("C"), note!("E"), note!("G")
    ]);

    let spelling2 = PitchClassSet::new(&[
        note!("E"), note!("G"), note!("C")  // First inversion ordering
    ]);

    let spelling3 = PitchClassSet::new(&[
        note!("G"), note!("C"), note!("E")  // Second inversion ordering
    ]);

    let spelling4 = PitchClassSet::new(&[
        note!("Dbb"), note!("Fb"), note!("G")  // Enharmonic spelling
    ]);

    println!("Spelling 1 (root position): {}", spelling1);
    println!("Spelling 2 (first inversion order): {}", spelling2);
    println!("Spelling 3 (second inversion order): {}", spelling3);
    println!("Spelling 4 (enharmonic): {}", spelling4);

    println!("\nAll four spellings have the same prime form:");
    println!("  Spelling 1: {:?}", spelling1.prime_form());
    println!("  Spelling 2: {:?}", spelling2.prime_form());
    println!("  Spelling 3: {:?}", spelling3.prime_form());
    println!("  Spelling 4: {:?}", spelling4.prime_form());

    println!("\nAll are recognized as major triads:");
    println!("  Spelling 1: {}", spelling1.is_major_triad());
    println!("  Spelling 2: {}", spelling2.is_major_triad());
    println!("  Spelling 3: {}", spelling3.is_major_triad());
    println!("  Spelling 4: {}", spelling4.is_major_triad());
    println!();
}

/// Example 4: Analyze complement relationships
fn complement_analysis() {
    println!("Example 4: Complement Relationships");
    println!("------------------------------------");

    let set = PitchClassSet::new(&[
        note!("C"), note!("D"), note!("E")
    ]);

    let complement = set.complement();

    println!("Original set: {}", set);
    println!("  Cardinality: {}", set.len());
    println!("  Prime form: {:?}", set.prime_form());

    println!("\nComplement set: {}", complement);
    println!("  Cardinality: {}", complement.len());
    println!("  Prime form: {:?}", complement.prime_form());

    println!("\nNote: Original + Complement = all 12 pitch classes");
    println!("  {} + {} = 12", set.len(), complement.len());
    println!();
}

/// Example 5: Analyze interval vectors for different chord types
fn interval_vector_analysis() {
    println!("Example 5: Interval Vector Analysis");
    println!("------------------------------------");

    let chords = vec![
        (vec![note!("C"), note!("E"), note!("G")], "Major triad"),
        (vec![note!("C"), note!("Eb"), note!("G")], "Minor triad"),
        (vec![note!("C"), note!("Eb"), note!("Gb")], "Diminished triad"),
        (vec![note!("C"), note!("E"), note!("G#")], "Augmented triad"),
    ];

    println!("Comparing interval vectors of different triads:\n");

    for (notes, name) in chords {
        let pc_set = PitchClassSet::new(&notes);
        let iv = pc_set.interval_vector();

        println!("{:20} -> {:?}", name, iv);
    }

    println!("\nNotice how each chord type has a unique interval vector,");
    println!("which serves as a \"fingerprint\" for that chord quality.");
    println!();
}

/// Example 6: Find common tones between chords
fn common_tone_analysis() {
    println!("Example 6: Common Tone Analysis");
    println!("--------------------------------");

    let c_major = PitchClassSet::new(&[note!("C"), note!("E"), note!("G")]);
    let a_minor = PitchClassSet::new(&[note!("A"), note!("C"), note!("E")]);
    let f_major = PitchClassSet::new(&[note!("F"), note!("A"), note!("C")]);

    println!("Analyzing common tones in chord progressions:\n");

    println!("C major: {}", c_major);
    println!("A minor: {}", a_minor);
    println!("F major: {}", f_major);

    let c_to_a = c_major.intersection(&a_minor);
    let a_to_f = a_minor.intersection(&f_major);
    let c_to_f = c_major.intersection(&f_major);

    println!("\nCommon tones:");
    println!("  C major → A minor: {} (cardinality: {})", c_to_a, c_to_a.len());
    println!("  A minor → F major: {} (cardinality: {})", a_to_f, a_to_f.len());
    println!("  C major → F major: {} (cardinality: {})", c_to_f, c_to_f.len());

    println!("\nC major is a subset of neither:");
    println!("  Is C major a subset of A minor? {}", c_major.is_subset_of(&a_minor));
    println!("  Is C major a subset of F major? {}", c_major.is_subset_of(&f_major));

    println!("\nBut we can check if smaller collections are subsets:");
    let two_notes = PitchClassSet::new(&[note!("C"), note!("E")]);
    println!("  Is {{C, E}} a subset of C major? {}", two_notes.is_subset_of(&c_major));
    println!("  Is {{C, E}} a subset of A minor? {}", two_notes.is_subset_of(&a_minor));
    println!();
}
