# Chordy

[![Crates.io](https://img.shields.io/crates/v/chordy.svg)](https://crates.io/crates/chordy)
[![Documentation](https://docs.rs/chordy/badge.svg)](https://docs.rs/chordy)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)

A theoretically correct music theory library for Rust with a focus on proper pitch spelling,
enharmonic awareness, and practical music applications.

## Overview

Chordy provides foundational music theory tools with unwavering commitment to theoretical 
correctness. It handles the complexities of Western music theory including proper enharmonic 
spelling, interval relationships, scales, and chords.

```rust
use chordy::{NoteName, Letter, Accidental, Pitch};

// Create a C# note
let c_sharp = NoteName::new(Letter::C, Accidental::Sharp);

// Check enharmonic equivalence with D♭
let d_flat = NoteName::new(Letter::D, Accidental::Flat);
assert!(c_sharp.is_enharmonic_with(&d_flat));
assert_ne!(c_sharp, d_flat); // Different note names
```

## Key Features

- **Correct Enharmonic Handling**: Distinguish between enharmonically equivalent notes (C♯/D♭)
- **Intelligent Transposition**: Transpose with proper music theory rules, not just semitone math
- **Chord Inversions and Slash Chords**: Full support for chord inversions (C/E) and slash chords (C/G)
- **Chord Progression Analysis**: Built-in progression maps with jazz harmony support
- **Roman Numeral Analysis**: Complete system for harmonic analysis and chord functions
- **Chord Identification**: Advanced chord naming with proper jazz conventions
- **Zero Dependencies**: Pure Rust implementation with no external dependencies
- **WebAssembly Compatible**: Use in browsers without modification
- **Theoretically Sound**: Built with accuracy and musical principles as the highest priority

## Installation

Add Chordy to your `Cargo.toml`:

```toml
[dependencies]
chordy = "0.1.0"
```

## Usage Examples

### Working with Notes and Pitches

```rust
use chordy::{NoteName, Letter, Accidental, Pitch};

// Create a note name (letter + accidental)
let e_flat = NoteName::new(Letter::E, Accidental::Flat);

// Create a pitch (note name + octave)
let middle_e_flat = Pitch::new(Letter::E, Accidental::Flat, 4);

// Get the MIDI note number
assert_eq!(middle_e_flat.midi_number(), 75);
```

### Transposition

```rust
use chordy::{NoteName, Letter, Accidental, Pitch};

// Create a pitch
let f = Pitch::new(Letter::F, Accidental::Natural, 4);

// Transpose up a perfect fifth (7 semitones)
let c = f.transpose(7);
assert_eq!(c.name.letter(), Letter::C);
assert_eq!(c.name.accidental(), Accidental::Natural);

// Chordy handles enharmonic spelling correctly
let c_sharp = Pitch::new(Letter::C, Accidental::Sharp, 4);
let e_sharp = c_sharp.transpose(4);
assert_eq!(e_sharp.name.letter(), Letter::E);
assert_eq!(e_sharp.name.accidental(), Accidental::Sharp);
```

### Scales and Chords

```rust
use chordy::{NoteName, Letter, Accidental, Scale, Key, scales, traits::ChordLike, Chord, note};

// Create a C major scale
let c = NoteName::new(Letter::C, Accidental::Natural);
let c_major = Scale::from_definition(c, scales::IONIAN);

// Get the notes in the scale
let notes = c_major.notes();
// [C, D, E, F, G, A, B]

// Generate a diatonic chord
let c_major_triad = c_major.chord_at_degree(1);
// [C, E, G]

// Create chord inversions
let c_major = Chord::major(note!("C"));
let first_inversion = c_major.with_inversion(1);  // C/E
let second_inversion = c_major.with_inversion(2); // C/G

// Create slash chords
let c_slash_f = c_major.with_slash_bass(note!("F")); // C/F
```

### Chord Progressions

```rust
use chordy::{Key, NoteName, Letter, Accidental, Chord, note};

// Create a key for progression analysis
let c_major = Key::Major(NoteName::new(Letter::C, Accidental::Natural));

// Create a chord for analysis  
let tonic_chord = Chord::major(note!("C"));

// Get progression options from a chord
let options = c_major.progression_options(&tonic_chord).unwrap();

// Categorized by harmonic strength
println!("Strong progressions: {:?}", options.strong.len());
println!("Moderate progressions: {:?}", options.moderate.len());
println!("Weak progressions: {:?}", options.weak.len());

// Use inversions in progressions for smoother voice leading
let progression = vec![
    Chord::major(note!("C")),                    // C major
    Chord::major(note!("A")).with_inversion(1),  // Am/C (first inversion)
    Chord::major(note!("F")),                    // F major
    Chord::major(note!("G")).with_inversion(2),  // G/D (second inversion)
];
```

**📖 [Complete Chord Progression Guide](docs/chord-progressions.md)** - Comprehensive documentation with examples for building progressions, jazz harmony, modal progressions, and harmonic analysis.

## Project Status

Chordy has mature implementations for core music theory concepts including pitch spelling, 
interval relationships, scales, chords, roman numeral analysis, and chord progressions. 
The API is stabilizing with ongoing work on advanced analysis tools and output formats.

### Roadmap

- [x] Notes, pitches, and enharmonic relationships
- [x] Basic interval representation 
- [x] Scales and modes
- [x] Chords and harmony
    - [x] Deriving basic triads from scales
    - [x] Chord identification and naming system
    - [x] Roman numeral analysis
    - [x] Chord inversions and slash chords
    - [ ] Deriving extended chords from scales
    - [ ] Deriving chords from multiple scales
    - [ ] Piano chord voicings based on hand ergonomics
- [x] Chord progressions
    - [x] Progression graph analysis (Stephen Mugglin's map)
    - [x] Chord variants and jazz extensions
    - [x] Categorized progression suggestions (strong/moderate/weak)
    - [ ] Voice leading analysis between progressions
- [ ] Advanced analysis tools
- [ ] MIDI integration
- [ ] Staff writing via MusicXML or similar
- [ ] HTML representation
    - [ ] Staff writing as SVG
    - [ ] Keyboard diagrams
- [ ] Guitar chord diagrams
    - [ ] Alternate tunings

## Design Philosophy

Chordy is built on these core principles:

1. **Theoretical Correctness**: Music theory concepts are represented with proper attention to their
   theoretical foundations
2. **Rustic API Design**: The library feels natural to Rust programmers while accurately
   representing musical concepts
3. **Zero Dependencies**: Pure Rust implementation for maximum portability and WebAssembly
   compatibility

## Features

- **utf8_symbols** (enabled by default): Uses proper UTF-8 musical symbols (♭, ♯, etc.) 
  in string representations. Disable this feature with `default-features = false` if you
  need ASCII-only output (for compatibility with terminals or systems that don't support
  these characters).

## Documentation

- **[Chord Progressions Guide](docs/chord-progressions.md)** - Complete guide to building and analyzing chord progressions
- **[API Documentation](https://docs.rs/chordy)** - Full API reference with examples

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Prerequisites

- Rust 1.56+ (2021 edition)
- Basic understanding of music theory concepts

### Testing

Run the test suite with:

```bash
cargo test
```

For doctests and examples:

```bash
cargo test --doc
```

## License

Chordy is licensed under the [GNU Lesser General Public License v3.0](LICENSE).

## Acknowledgments

Chordy's pitch spelling algorithm is influenced by the research of Emilios Cambouropoulos in
"Automatic Pitch Spelling: From Numbers to Sharps and Flats."
