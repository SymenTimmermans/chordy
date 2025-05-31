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
use chordy::{NoteName, Letter, Accidental, Scale, scales, traits::ChordLike};

// Create a C major scale
let c = NoteName::new(Letter::C, Accidental::Natural);
let c_major = Scale::new(c, scales::IONIAN);

// Get the notes in the scale
let notes = c_major.notes();
// [C, D, E, F, G, A, B]

// Generate a diatonic chord
// TODO: FIX let c_major_triad = c_major.triad_at_degree(1);
// [C, E, G]
```

## Project Status

Chordy is currently in early development. The core pitch and interval systems are stable, with
ongoing work on scales, chords, and more advanced music theory concepts.

### Roadmap

- [x] Notes, pitches, and enharmonic relationships
- [x] Basic interval representation 
- [x] Scales and modes
- [x] Chords and harmony
    - [x] Deriving basic triads from scales
    - [ ] Deriving extended chords from scales
    - [ ] Deriving chords from multiple scales
    - [ ] Piano chord voicings based on hand ergonomics
- [ ] Voice leading
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
