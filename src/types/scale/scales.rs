//! This file is generated via build.rs from the scales.csv file. Do not edit manually.

use crate::{Interval, ScaleBitmask, ScaleDefinition};

/// Ionian: P1,M2,M3,P4,P5,M6,M7
pub const IONIAN: ScaleDefinition = ScaleDefinition {
    name: "Ionian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101010110101),
    mode_of: None,
    degree_offset: None,
};

/// Ionian Sharp 5 (mode 3 of Harmonic Minor): P1,M2,M3,P4,A5,M6,M7
pub const IONIAN_SHARP_5: ScaleDefinition = ScaleDefinition {
    name: "Ionian Sharp 5",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::AUGMENTED_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101100110101),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(2),
};

/// Dorian (mode 2 of Ionian): P1,M2,m3,P4,P5,M6,m7
pub const DORIAN: ScaleDefinition = ScaleDefinition {
    name: "Dorian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011010101101),
    mode_of: Some("Ionian"),
    degree_offset: Some(1),
};

/// Dorian Altered (mode 4 of Harmonic Minor): P1,M2,m3,A4,P5,M6,m7
pub const DORIAN_ALTERED: ScaleDefinition = ScaleDefinition {
    name: "Dorian Altered",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011011001101),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(3),
};

/// Dorian Flat 2 (mode 2 of Melodic Minor): P1,m2,m3,P4,P5,M6,m7
pub const DORIAN_FLAT_2: ScaleDefinition = ScaleDefinition {
    name: "Dorian Flat 2",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011010101011),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(1),
};

/// Phrygian (mode 3 of Ionian): P1,m2,m3,P4,P5,m6,m7
pub const PHRYGIAN: ScaleDefinition = ScaleDefinition {
    name: "Phrygian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010110101011),
    mode_of: Some("Ionian"),
    degree_offset: Some(2),
};

/// Phrygian Dominant (mode 5 of Harmonic Minor): P1,m2,M3,P4,P5,m6,m7
pub const PHRYGIAN_DOMINANT: ScaleDefinition = ScaleDefinition {
    name: "Phrygian Dominant",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010110110011),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(4),
};

/// Lydian (mode 4 of Ionian): P1,M2,M3,A4,P5,M6,M7
pub const LYDIAN: ScaleDefinition = ScaleDefinition {
    name: "Lydian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101011010101),
    mode_of: Some("Ionian"),
    degree_offset: Some(3),
};

/// Lydian Sharp 2 (mode 6 of Harmonic Minor): P1,A2,M3,A4,P5,M6,M7
pub const LYDIAN_SHARP_2: ScaleDefinition = ScaleDefinition {
    name: "Lydian Sharp 2",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::AUGMENTED_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101011011001),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(5),
};

/// Lydian Augmented (mode 3 of Melodic Minor): P1,M2,M3,A4,A5,M6,M7
pub const LYDIAN_AUGMENTED: ScaleDefinition = ScaleDefinition {
    name: "Lydian Augmented",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::AUGMENTED_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101101010101),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(2),
};

/// Lydian Dominant (mode 4 of Melodic Minor): P1,M2,M3,A4,P5,M6,m7
pub const LYDIAN_DOMINANT: ScaleDefinition = ScaleDefinition {
    name: "Lydian Dominant",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011011010101),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(3),
};

/// Mixolydian (mode 5 of Ionian): P1,M2,M3,P4,P5,M6,m7
pub const MIXOLYDIAN: ScaleDefinition = ScaleDefinition {
    name: "Mixolydian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011010110101),
    mode_of: Some("Ionian"),
    degree_offset: Some(4),
};

/// Mixolydian Flat 6 (mode 5 of Melodic Minor): P1,M2,M3,P4,P5,m6,m7
pub const MIXOLYDIAN_FLAT_6: ScaleDefinition = ScaleDefinition {
    name: "Mixolydian Flat 6",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010110110101),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(4),
};

/// Aeolian (mode 6 of Ionian): P1,M2,m3,P4,P5,m6,m7
pub const AEOLIAN: ScaleDefinition = ScaleDefinition {
    name: "Aeolian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010110101101),
    mode_of: Some("Ionian"),
    degree_offset: Some(5),
};

/// Harmonic Minor: P1,M2,m3,P4,P5,m6,M7
pub const HARMONIC_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Harmonic Minor",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110101101),
    mode_of: None,
    degree_offset: None,
};

/// Melodic Minor: P1,M2,m3,P4,P5,M6,M7
pub const MELODIC_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Melodic Minor",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101010101101),
    mode_of: None,
    degree_offset: None,
};

/// Locrian (mode 7 of Ionian): P1,m2,m3,P4,d5,m6,m7
pub const LOCRIAN: ScaleDefinition = ScaleDefinition {
    name: "Locrian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101101011),
    mode_of: Some("Ionian"),
    degree_offset: Some(6),
};

/// Locrian Natural 2 (mode 6 of Melodic Minor): P1,M2,m3,P4,d5,m6,m7
pub const LOCRIAN_NATURAL_2: ScaleDefinition = ScaleDefinition {
    name: "Locrian Natural 2",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101101101),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(5),
};

/// Locrian Natural 6 (mode 3 of Harmonic Minor): P1,m2,m3,P4,d5,M6,m7
pub const LOCRIAN_NATURAL_6: ScaleDefinition = ScaleDefinition {
    name: "Locrian Natural 6",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011001101011),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(2),
};

/// Super Locrian (mode 7 of Melodic Minor): P1,m2,m3,M3,d5,m6,m7
pub const SUPER_LOCRIAN: ScaleDefinition = ScaleDefinition {
    name: "Super Locrian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::MAJOR_THIRD,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101011011),
    mode_of: Some("Melodic Minor"),
    degree_offset: Some(6),
};

/// Ultralocrian (mode 7 of Harmonic Minor): P1,m2,m3,d4,d5,m6,d7
pub const ULTRALOCRIAN: ScaleDefinition = ScaleDefinition {
    name: "Ultralocrian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::DIMINISHED_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::DIMINISHED_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b001101011011),
    mode_of: Some("Harmonic Minor"),
    degree_offset: Some(6),
};

/// Whole Tone: P1,M2,M3,A4,A5,A6
pub const WHOLE_TONE: ScaleDefinition = ScaleDefinition {
    name: "Whole Tone",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::AUGMENTED_FIFTH,
        Interval::AUGMENTED_SIXTH,
    ],
    bitmask: ScaleBitmask(0b010101010101),
    mode_of: None,
    degree_offset: None,
};

/// Hungarian Minor: P1,M2,m3,A4,P5,m6,M7
pub const HUNGARIAN_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Hungarian Minor",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100111001101),
    mode_of: None,
    degree_offset: None,
};

/// Altered: P1,m2,m3,d4,d5,m6,m7
pub const ALTERED: ScaleDefinition = ScaleDefinition {
    name: "Altered",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::DIMINISHED_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101011011),
    mode_of: None,
    degree_offset: None,
};

/// Algerian: P1,M2,m3,A4,P5,m6,M7
pub const ALGERIAN: ScaleDefinition = ScaleDefinition {
    name: "Algerian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100111001101),
    mode_of: None,
    degree_offset: None,
};

/// Augmented: P1,m3,M3,P5,A5,M7
pub const AUGMENTED: ScaleDefinition = ScaleDefinition {
    name: "Augmented",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::AUGMENTED_FIFTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110011001),
    mode_of: None,
    degree_offset: None,
};

/// Bebop Dominant: P1,M2,M3,P4,P5,M6,m7,M7
pub const BEBOP_DOMINANT: ScaleDefinition = ScaleDefinition {
    name: "Bebop Dominant",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b111010110101),
    mode_of: None,
    degree_offset: None,
};

/// Blues: P1,m3,P4,d5,P5,m7
pub const BLUES: ScaleDefinition = ScaleDefinition {
    name: "Blues",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010011101001),
    mode_of: None,
    degree_offset: None,
};

/// Double Harmonic: P1,m2,M3,P4,P5,m6,M7
pub const DOUBLE_HARMONIC: ScaleDefinition = ScaleDefinition {
    name: "Double Harmonic",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110110011),
    mode_of: None,
    degree_offset: None,
};

/// Enigmatic: P1,m2,M3,A4,A5,A6,M7
pub const ENIGMATIC: ScaleDefinition = ScaleDefinition {
    name: "Enigmatic",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::AUGMENTED_FIFTH,
        Interval::AUGMENTED_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b110101010011),
    mode_of: None,
    degree_offset: None,
};

/// Flamenco: P1,m2,M3,P4,P5,m6,M7
pub const FLAMENCO: ScaleDefinition = ScaleDefinition {
    name: "Flamenco",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110110011),
    mode_of: None,
    degree_offset: None,
};

/// Gypsy: P1,M2,m3,A4,P5,m6,m7
pub const GYPSY: ScaleDefinition = ScaleDefinition {
    name: "Gypsy",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010111001101),
    mode_of: None,
    degree_offset: None,
};

/// Half Diminished: P1,M2,m3,P4,d5,m6,m7
pub const HALF_DIMINISHED: ScaleDefinition = ScaleDefinition {
    name: "Half Diminished",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101101101),
    mode_of: None,
    degree_offset: None,
};

/// Harmonic Major: P1,M2,M3,P4,P5,m6,M7
pub const HARMONIC_MAJOR: ScaleDefinition = ScaleDefinition {
    name: "Harmonic Major",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110110101),
    mode_of: None,
    degree_offset: None,
};

/// Hungarian Major: P1,A2,M3,A4,P5,M6,m7
pub const HUNGARIAN_MAJOR: ScaleDefinition = ScaleDefinition {
    name: "Hungarian Major",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::AUGMENTED_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011011011001),
    mode_of: None,
    degree_offset: None,
};

/// Major Bebop: P1,M2,M3,P4,P5,A5,M6,M7
pub const MAJOR_BEBOP: ScaleDefinition = ScaleDefinition {
    name: "Major Bebop",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::AUGMENTED_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101110110101),
    mode_of: None,
    degree_offset: None,
};

/// Major Locrian: P1,M2,M3,P4,d5,m6,m7
pub const MAJOR_LOCRIAN: ScaleDefinition = ScaleDefinition {
    name: "Major Locrian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010101110101),
    mode_of: None,
    degree_offset: None,
};

/// Major Pentatonic: P1,M2,M3,P5,M6
pub const MAJOR_PENTATONIC: ScaleDefinition = ScaleDefinition {
    name: "Major Pentatonic",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
    ],
    bitmask: ScaleBitmask(0b001010010101),
    mode_of: None,
    degree_offset: None,
};

/// Neapolitan Major: P1,m2,m3,P4,P5,M6,M7
pub const NEAPOLITAN_MAJOR: ScaleDefinition = ScaleDefinition {
    name: "Neapolitan Major",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b101010101011),
    mode_of: None,
    degree_offset: None,
};

/// Neapolitan Minor: P1,m2,m3,P4,P5,m6,M7
pub const NEAPOLITAN_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Neapolitan Minor",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100110101011),
    mode_of: None,
    degree_offset: None,
};

/// Octatonic 1: P1,M2,m3,P4,d5,m6,M6,m7
pub const OCTATONIC_1: ScaleDefinition = ScaleDefinition {
    name: "Octatonic 1",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011101101101),
    mode_of: None,
    degree_offset: None,
};

/// Octatonic 2: P1,m2,m3,M3,A4,P5,M6,m7
pub const OCTATONIC_2: ScaleDefinition = ScaleDefinition {
    name: "Octatonic 2",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011011011011),
    mode_of: None,
    degree_offset: None,
};

/// Persian: P1,m2,M3,P4,d5,m6,M7
pub const PERSIAN: ScaleDefinition = ScaleDefinition {
    name: "Persian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::PERFECT_FOURTH,
        Interval::DIMINISHED_FIFTH,
        Interval::MINOR_SIXTH,
        Interval::MAJOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b100101110011),
    mode_of: None,
    degree_offset: None,
};

/// Prometheus: P1,M2,M3,A4,M6,m7
pub const PROMETHEUS: ScaleDefinition = ScaleDefinition {
    name: "Prometheus",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011001010101),
    mode_of: None,
    degree_offset: None,
};

/// Tritone: P1,m2,M3,d5,P5,m7
pub const TRITONE: ScaleDefinition = ScaleDefinition {
    name: "Tritone",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MINOR_SECOND,
        Interval::MAJOR_THIRD,
        Interval::DIMINISHED_FIFTH,
        Interval::PERFECT_FIFTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b010011010011),
    mode_of: None,
    degree_offset: None,
};

/// Ukranian Dorian: P1,M2,m3,A4,P5,M6,m7
pub const UKRANIAN_DORIAN: ScaleDefinition = ScaleDefinition {
    name: "Ukranian Dorian",
    intervals: &[
        Interval::PERFECT_UNISON,
        Interval::MAJOR_SECOND,
        Interval::MINOR_THIRD,
        Interval::AUGMENTED_FOURTH,
        Interval::PERFECT_FIFTH,
        Interval::MAJOR_SIXTH,
        Interval::MINOR_SEVENTH,
    ],
    bitmask: ScaleBitmask(0b011011001101),
    mode_of: None,
    degree_offset: None,
};

/// Registry of all scales.
pub const REGISTRY: &[ScaleDefinition] = &[
    IONIAN,
    IONIAN_SHARP_5,
    DORIAN,
    DORIAN_ALTERED,
    DORIAN_FLAT_2,
    PHRYGIAN,
    PHRYGIAN_DOMINANT,
    LYDIAN,
    LYDIAN_SHARP_2,
    LYDIAN_AUGMENTED,
    LYDIAN_DOMINANT,
    MIXOLYDIAN,
    MIXOLYDIAN_FLAT_6,
    AEOLIAN,
    HARMONIC_MINOR,
    MELODIC_MINOR,
    LOCRIAN,
    LOCRIAN_NATURAL_2,
    LOCRIAN_NATURAL_6,
    SUPER_LOCRIAN,
    ULTRALOCRIAN,
    WHOLE_TONE,
    HUNGARIAN_MINOR,
    ALTERED,
    ALGERIAN,
    AUGMENTED,
    BEBOP_DOMINANT,
    BLUES,
    DOUBLE_HARMONIC,
    ENIGMATIC,
    FLAMENCO,
    GYPSY,
    HALF_DIMINISHED,
    HARMONIC_MAJOR,
    HUNGARIAN_MAJOR,
    MAJOR_BEBOP,
    MAJOR_LOCRIAN,
    MAJOR_PENTATONIC,
    NEAPOLITAN_MAJOR,
    NEAPOLITAN_MINOR,
    OCTATONIC_1,
    OCTATONIC_2,
    PERSIAN,
    PROMETHEUS,
    TRITONE,
    UKRANIAN_DORIAN,
];
