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

// ii-V-I with jazz extensions
let jazz_progression = vec![
    key.progression_node("ii9").unwrap(),      // Cm9
    key.progression_node("V7+b9").unwrap(),    // F7♭9 (altered dominant)
    key.progression_node("Imaj9").unwrap(),    // B♭maj9
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