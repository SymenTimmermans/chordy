# Chord Progressions in Chordy

Chordy provides a comprehensive chord progression system based on Stephen Mugglin's progression map, supporting both traditional and jazz harmony with intelligent progression suggestions.

## Quick Start

```rust
use chordy::{Key, note};

// Create a key for progression analysis
let key = Key::Major(note!("C"));

// Look up specific chord variants
let tonic = key.progression_node("I").unwrap();
let dominant_seventh = key.progression_node("V7").unwrap();

// Get intelligent progression suggestions
let options = key.progression_options(&tonic).unwrap();
println!("From I: {} strong, {} moderate, {} weak options", 
         options.strong.len(), options.moderate.len(), options.weak.len());
```

## Core Concepts

### Progression Strength Categories

Chord connections are categorized by their harmonic strength:

- **Strong**: Explicit arrows showing natural voice leading (I→IV, I→V, V→I, ii→V, etc.)
- **Moderate**: Jumps to primary nodes - stable but less conventional
- **Weak**: Jumps to secondary nodes - creates tension, needs resolution to primary areas

### Chord Variants

The system supports comprehensive jazz harmony:

- **Basic triads**: I, ii, iii, IV, V, vi, vii
- **Seventh chords**: I7, ii7, V7, Imaj7, etc.
- **Extensions**: 9, 11, 13 (automatically include lower extensions)
- **Altered dominants**: 7+b9, 7+#9 for tension and resolution
- **Added tones**: 6, add9, sus2, sus4

### Jazz Harmony Rules

- **"7" always means dominant 7th** (minor seventh interval) regardless of chord quality
- **"maj7" explicitly indicates major 7th** for major seventh chords  
- **Extensions imply 7th**: "9" = "7+9", "11" = "7+9+11", "13" = "7+9+11+13"

## Building Progressions

### Basic Diatonic Progressions

```rust
use chordy::{Key, note};

let key = Key::Major(note!("C"));

// Classic I-vi-IV-V progression
let progression = vec![
    key.progression_node("I").unwrap(),    // C major
    key.progression_node("vi").unwrap(),   // A minor  
    key.progression_node("IV").unwrap(),   // F major
    key.progression_node("V7").unwrap(),   // G dominant 7th
];

// Access full harmonic information
for chord in progression {
    println!("{}: {:?}", chord.display_name, chord.intervals);
    println!("Roman: {}", chord.roman_numeral);
    println!("Function: {}", chord.base_function);
}
```

### Intelligent Progression Building

Use the categorized progression options to build musically sound sequences:

```rust
use chordy::{Key, note};

let key = Key::Major(note!("F"));
let mut current_chord = key.progression_node("I").unwrap();
let mut progression = vec![current_chord];

// Build a progression using harmonic strength
for _ in 0..3 {
    let options = key.progression_options(current_chord).unwrap();
    
    // Prefer strong progressions for conventional sound
    current_chord = if !options.strong.is_empty() {
        options.strong[0]
    } else if !options.moderate.is_empty() {
        options.moderate[0]
    } else {
        options.weak[0]
    };
    
    progression.push(current_chord);
    println!("Next: {} ({})", 
             current_chord.display_name,
             if options.strong.contains(&current_chord) { "strong" }
             else if options.moderate.contains(&current_chord) { "moderate" }
             else { "weak" });
}
```

### Jazz Progressions with Extensions

```rust
use chordy::{Key, note, Interval};

let key = Key::Major(note!("Bb"));

// ii-V-I with jazz extensions and smooth voice leading
let jazz_progression = vec![
    key.progression_node("ii9").unwrap(),      // Cm9
    key.progression_node("V7+b9").unwrap(),    // F7♭9 (altered dominant)
    key.progression_node("Imaj9").unwrap(),    // B♭maj9
];

// Convert to concrete chords and add inversions for voice leading
let concrete_jazz = vec![
    Chord::new(note!("C"), vec![Interval::PERFECT_UNISON, Interval::MINOR_THIRD, 
                                Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, 
                                Interval::MAJOR_NINTH]),
    Chord::new(note!("F"), vec![Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, 
                                Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, 
                                Interval::MINOR_NINTH]).with_inversion(1), // F7♭9/A
    Chord::new(note!("Bb"), vec![Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, 
                                 Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH, 
                                 Interval::MAJOR_NINTH]),
];

// Analyze the altered dominant
let altered_dominant = jazz_progression[1];
assert!(altered_dominant.intervals.contains(&Interval::MINOR_SEVENTH));
assert!(altered_dominant.intervals.contains(&Interval::MINOR_NINTH));

println!("Altered dominant {} contains intervals: {:?}", 
         altered_dominant.display_name, 
         altered_dominant.intervals);
```

### Modal and Minor Key Progressions

```rust
use chordy::{Key, note};

let key = Key::Minor(note!("D"));

// Natural minor progression with borrowed chords
let modal_progression = vec![
    key.progression_node("i").unwrap(),     // D minor (tonic)
    key.progression_node("bVII").unwrap(),  // C major (borrowed from major)
    key.progression_node("bVI").unwrap(),   // B♭ major (borrowed from major)
    key.progression_node("V7").unwrap(),    // A dominant 7th (harmonic minor)
];

// Minor keys have different harmonic colors
for chord in modal_progression {
    println!("{} - Type: {:?}", chord.display_name, chord.node_type);
}
```

## Chord Inversions in Progressions

Chord inversions play a crucial role in smooth voice leading and can dramatically improve the flow of chord progressions. Chordy supports both classical inversions and slash chords.

### Basic Inversions

```rust
use chordy::{Chord, note};

let c_major = Chord::major(note!("C"));

// Root position: C-E-G
let root_position = c_major; // C

// First inversion: E-G-C (bass note is the third)
let first_inversion = c_major.with_inversion(1); // C/E

// Second inversion: G-C-E (bass note is the fifth)
let second_inversion = c_major.with_inversion(2); // C/G

// Check the bass note and inversion type
assert_eq!(first_inversion.bass_note(), note!("E"));
assert!(first_inversion.is_inverted());
assert_eq!(first_inversion.inversion_number(), Some(1));
```

### Slash Chords vs Inversions

While both use the same display format (C/E), they represent different concepts:

```rust
use chordy::{Chord, note, BassType};

let c_major = Chord::major(note!("C"));

// Classical inversion - bass note is a chord tone
let inversion = c_major.with_inversion(1); // C/E (E is the third)

// Slash chord - bass note can be any note
let slash_chord = c_major.with_slash_bass(note!("F")); // C/F (F is not a chord tone)

// Check the difference
assert!(inversion.is_inverted());
assert!(!inversion.is_slash_chord());

assert!(!slash_chord.is_inverted());
assert!(slash_chord.is_slash_chord());
```

### Voice Leading with Inversions

Use inversions to create smooth bass lines in progressions:

```rust
use chordy::{Key, Chord, note};

let key = Key::Major(note!("C"));

// Progression with smooth bass line: C - C - F - G
let smooth_progression = vec![
    Chord::major(note!("C")),                    // C (bass: C)
    Chord::minor(note!("A")).with_inversion(1),  // Am/C (bass: C)
    Chord::major(note!("F")),                    // F (bass: F)
    Chord::major(note!("G")).with_inversion(2),  // G/D (bass: D)
];

// Bass line: C - C - F - D (stepwise motion)
for chord in &smooth_progression {
    println!("{}: bass = {}", chord, chord.bass_note());
}
```

### Roman Numeral Analysis with Inversions

```rust
use chordy::{Key, RomanChord, roman};

let key = Key::Major(note!("C"));

// Create roman chord inversions
let i_chord = RomanChord::major(roman!("I"));
let i_first_inversion = i_chord.with_inversion(1); // I/III

// Convert to concrete chords
let concrete_inversion = i_first_inversion.in_key(&key);
assert_eq!(concrete_inversion.bass_note(), note!("E"));

// Analysis preserves inversion information
let analyzed = concrete_inversion.to_roman(&key).unwrap();
assert!(analyzed.is_inverted());
```

### Jazz Progressions with Inversions

```rust
use chordy::{Key, Chord, note};

let key = Key::Major(note!("F"));

// ii-V-I with inversions for smooth voice leading
let jazz_progression = vec![
    // Gm7 (ii7)
    Chord::minor_7th(note!("G")),
    // C7/E (V7/3 - first inversion for smooth bass)
    Chord::dominant_7th(note!("C")).with_inversion(1),
    // Fmaj7 (Imaj7)
    Chord::major_7th(note!("F")),
];

// Bass line: G - E - F (smoother than G - C - F)
for chord in &jazz_progression {
    println!("{}: bass = {}", chord, chord.bass_note());
}
```

### Slash Chords in Modern Progressions

```rust
use chordy::{Chord, note};

// Modern pop progression with slash chords
let modern_progression = vec![
    Chord::major(note!("C")),                    // C
    Chord::major(note!("G")).with_slash_bass(note!("B")), // G/B
    Chord::minor(note!("A")),                    // Am
    Chord::major(note!("F")),                    // F
];

// Creates a descending bass line: C - B - A - F
for chord in &modern_progression {
    println!("{}: bass = {}", chord, chord.bass_note());
}
```

### Inversion Convenience Methods

```rust
use chordy::{Chord, note};

let c_major = Chord::major(note!("C"));

// Convenience methods for common inversions
let first_inv = c_major.in_first_inversion();   // C/E
let second_inv = c_major.in_second_inversion(); // C/G

// Works with any chord type
let dm7 = Chord::minor_7th(note!("D"));
let dm7_third_inv = dm7.with_inversion(3); // Dm7/C (minor 7th in bass)
```

## Advanced Analysis

### Chord Function Analysis

```rust
use chordy::{Key, note, Chord};

let key = Key::Major(note!("G"));
let chord_node = key.progression_node("V7").unwrap();

// Convert progression node to actual chord
let roman_chord = chord_node.to_roman_chord();
let actual_chord = roman_chord.in_key(key);

// Analyze harmonic function
println!("Chord: {}", actual_chord.name());
println!("Roman numeral: {}", chord_node.roman_numeral);
println!("Base function: {}", chord_node.base_function);
println!("Node type: {:?}", chord_node.node_type);

// Check resolution potential
let options = key.progression_options(chord_node).unwrap();
println!("Resolves strongly to {} chords", options.strong.len());
for strong_target in &options.strong {
    println!("  → {}", strong_target.display_name);
}
```

### Custom Progression Analysis

```rust
use chordy::{Key, note, Chord, NoteName, Letter, Accidental};

let key = Key::Major(note!("C"));

// Analyze a custom chord sequence
let chord_names = ["Cmaj7", "Am7", "Dm7", "G7"];
for name in chord_names {
    let chord = Chord::from_str(name).unwrap();
    let roman = chord.to_roman(&key);
    
    // Find matching progression node
    if let Some(node) = key.progression_node(&roman.to_string()) {
        println!("{} = {} in key of C", name, node.display_name);
        
        let options = key.progression_options(node).unwrap();
        println!("  Strong options: {}", 
                 options.strong.iter()
                        .map(|n| n.display_name)
                        .collect::<Vec<_>>()
                        .join(", "));
    }
}
```

## Available Chord Variants

### Major Key Variants

| Roman | Variants | Example in C Major |
|-------|----------|-------------------|
| I | base, 6, 7, 9, maj7, maj9 | C, C6, C7, C9, Cmaj7, Cmaj9 |
| ii | base, 7, 9, 11, 7+b9 | Dm, Dm7, Dm9, Dm11, Dm7♭9 |
| iii | base, 7, m7 | Em, Em7, Em7 |
| IV | base, 6, 7, 9, maj7, #11 | F, F6, F7, F9, Fmaj7, F#11 |
| V | base, 7, 9, 11, 13, 7+b9, 7+#9 | G, G7, G9, G11, G13, G7♭9, G7#9 |
| vi | base, 7, 9, m7 | Am, Am7, Am9, Am7 |
| vii | base, b5, m7b5 | Bm♭5, Bm7♭5 |

### Minor Key Variants

| Roman | Variants | Example in A Minor |
|-------|----------|-------------------|
| i | base, 7, 9, m7, m9 | Am, Am7, Am9, Am7, Am9 |
| ii | base, b5, m7b5, b5+7 | Bm♭5, Bm7♭5, Bm♭5+7 |
| III | base, 7, 9, maj7 | C, C7, C9, Cmaj7 |
| iv | base, 7, 9, m7 | Dm, Dm7, Dm9, Dm7 |
| V | base, 7, 9, 7+b9 | E, E7, E9, E7♭9 |
| VI | base, 7, 9, maj7 | F, F7, F9, Fmaj7 |
| VII | base, 7, 9 | G, G7, G9 |

## Implementation Details

### Zero-Cost Architecture

The progression system uses compile-time code generation for maximum performance:

- **Static lookup tables**: All progression data stored as static references
- **Zero allocations**: Node and edge lookups require no memory allocation
- **Instant access**: O(1) chord variant lookup via generated match statements
- **Type safety**: All chord variants validated at compile time

### Data Sources

Progression data is generated from human-readable `.progression` files:

```
# data/progressions/major_simple.progression
I     | primary   | I   | ,6,7,9,maj7,maj9
ii    | primary   | ii  | ,7,9,11,7+b9
V     | primary   | V   | ,7,9,11,13,7+b9,7+#9

# Explicit voice leading connections
I     -> IV
I     -> V
V     -> I
ii    -> V
```

The build script parses these files and generates static Rust code with all chord variants and their interval content.

## Integration with Other Systems

### Roman Numeral Analysis

```rust
use chordy::{Key, note, RomanDegree};

let key = Key::Major(note!("F"));
let node = key.progression_node("V7").unwrap();

// Access roman numeral components
assert_eq!(node.roman_numeral.degree(), RomanDegree::V);
assert_eq!(node.display_name, "V7");

// Convert to roman chord for analysis
let roman_chord = node.to_roman_chord();
let actual_chord = roman_chord.in_key(key);
```

### Chord Identification

```rust
use chordy::{Key, note, Chord};

let key = Key::Major(note!("D"));

// Identify chord from notes
let chord = Chord::from_notes(&[
    note!("F#"), note!("A"), note!("D"), note!("G")
]).unwrap();

// Find in progression system
let roman = chord.to_roman(&key);
if let Some(node) = key.progression_node(&roman.to_string()) {
    println!("Chord {} is {} in D major", chord.name(), node.display_name);
}
```

### Scale Integration

```rust
use chordy::{Key, Scale, scales, note};

let key = Key::Major(note!("E"));
let scale = key.scale();

// Build chords from scale degrees
let chord_at_degree_5 = scale.chord_at_degree(5);
let progression_node = key.progression_node("V").unwrap();

// Both represent the same harmonic function
assert_eq!(chord_at_degree_5.root(), progression_node.roman_numeral.to_interval());
```

This progression system provides a complete foundation for harmonic analysis, composition tools, and music theory education applications.