//! This file is generated via build.rs from the scales.csv file. Do not edit manually.

use crate::{ScaleDefinition, Interval, ScaleBitmask};

pub const IONIAN: ScaleDefinition = ScaleDefinition {
    name: "Ionian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MAJOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH, Interval::MAJOR_SEVENTH],
    bitmask: ScaleBitmask(0b101010110101),
    mode_of: None,
    degree_offset: None,
};

pub const DORIAN: ScaleDefinition = ScaleDefinition {
    name: "Dorian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b011010101101),
    mode_of: Some("Ionian"),
    degree_offset: Some(1),
};

pub const PHRYGIAN: ScaleDefinition = ScaleDefinition {
    name: "Phrygian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MINOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MINOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b010110101011),
    mode_of: Some("Ionian"),
    degree_offset: Some(2),
};

pub const LYDIAN: ScaleDefinition = ScaleDefinition {
    name: "Lydian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MAJOR_THIRD, Interval::AUGMENTED_FOURTH, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH, Interval::MAJOR_SEVENTH],
    bitmask: ScaleBitmask(0b101011010101),
    mode_of: Some("Ionian"),
    degree_offset: Some(3),
};

pub const MIXOLYDIAN: ScaleDefinition = ScaleDefinition {
    name: "Mixolydian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MAJOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b011010110101),
    mode_of: Some("Ionian"),
    degree_offset: Some(4),
};

pub const AEOLIAN: ScaleDefinition = ScaleDefinition {
    name: "Aeolian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MINOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b010110101101),
    mode_of: Some("Ionian"),
    degree_offset: Some(5),
};

pub const HARMONIC_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Harmonic Minor",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MINOR_SIXTH, Interval::MAJOR_SEVENTH],
    bitmask: ScaleBitmask(0b100110101101),
    mode_of: None,
    degree_offset: None,
};

pub const MELODIC_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Melodic Minor",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH, Interval::MAJOR_SEVENTH],
    bitmask: ScaleBitmask(0b101010101101),
    mode_of: None,
    degree_offset: None,
};

pub const LOCRIAN: ScaleDefinition = ScaleDefinition {
    name: "Locrian",
    intervals: &[Interval::PERFECT_UNISON, Interval::MINOR_SECOND, Interval::MINOR_THIRD, Interval::PERFECT_FOURTH, Interval::DIMINISHED_FIFTH, Interval::MINOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b010101101011),
    mode_of: Some("Ionian"),
    degree_offset: Some(6),
};

pub const WHOLE_TONE: ScaleDefinition = ScaleDefinition {
    name: "Whole Tone",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MAJOR_THIRD, Interval::AUGMENTED_FOURTH, Interval::AUGMENTED_FIFTH, Interval::AUGMENTED_SIXTH],
    bitmask: ScaleBitmask(0b010101010101),
    mode_of: None,
    degree_offset: None,
};

pub const HUNGARIAN_MINOR: ScaleDefinition = ScaleDefinition {
    name: "Hungarian Minor",
    intervals: &[Interval::PERFECT_UNISON, Interval::MAJOR_SECOND, Interval::MINOR_THIRD, Interval::AUGMENTED_FOURTH, Interval::PERFECT_FIFTH, Interval::MINOR_SIXTH, Interval::MAJOR_SEVENTH],
    bitmask: ScaleBitmask(0b100111001101),
    mode_of: None,
    degree_offset: None,
};

pub const ALTERED: ScaleDefinition = ScaleDefinition {
    name: "Altered",
    intervals: &[Interval::PERFECT_UNISON, Interval::MINOR_SECOND, Interval::MINOR_THIRD, Interval::DIMINISHED_FOURTH, Interval::DIMINISHED_FIFTH, Interval::MINOR_SIXTH, Interval::MINOR_SEVENTH],
    bitmask: ScaleBitmask(0b010101011011),
    mode_of: None,
    degree_offset: None,
};

pub const REGISTRY: &[ScaleDefinition] = &[
    IONIAN,
    DORIAN,
    PHRYGIAN,
    LYDIAN,
    MIXOLYDIAN,
    AEOLIAN,
    HARMONIC_MINOR,
    MELODIC_MINOR,
    LOCRIAN,
    WHOLE_TONE,
    HUNGARIAN_MINOR,
    ALTERED,
];
