//! Generated progression data for minor keys from minor.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionEdge, NodeType};
use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};
use std::collections::HashMap;

// Common interval patterns (reused across multiple chords)
/// Standard major triad intervals: root, major third, perfect fifth
const MAJOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE], 3);

/// Standard minor triad intervals: root, minor third, perfect fifth
const MINOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE, Interval::NONE, Interval::NONE,
     Interval::NONE], 3);

/// i chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_I: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_I_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// i7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_I_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_7_INTERVALS_SET,
    bass: None,
};

const MINOR_I_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// i9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_I_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_9_INTERVALS_SET,
    bass: None,
};

const MINOR_I_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// im7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_I_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_M7_INTERVALS_SET,
    bass: None,
};

const MINOR_I_M9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// im9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_I_M9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: MINOR_I_M9_INTERVALS_SET,
    bass: None,
};

/// ii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_II: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_II_FLAT_5_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::DIMINISHED_FIFTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 3);

/// iib5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth
pub static MINOR_II_FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_FLAT_5_INTERVALS_SET,
    bass: None,
};

const MINOR_II_M7_FLAT_5_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::DIMINISHED_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// iim7b5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_II_M7_FLAT_5: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_M7_FLAT_5_INTERVALS_SET,
    bass: None,
};

const MINOR_II_FLAT_5_PLUS_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::DIMINISHED_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// iib5+7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_II_FLAT_5_PLUS_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_II_FLAT_5_PLUS_7_INTERVALS_SET,
    bass: None,
};

/// III chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static III: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const III_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// III7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static III_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_7_INTERVALS_SET,
    bass: None,
};

const III_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// III9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static III_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_9_INTERVALS_SET,
    bass: None,
};

const III_MAJ7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// IIImaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static III_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: III_MAJ7_INTERVALS_SET,
    bass: None,
};

/// iv chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_IV: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_IV_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// iv7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IV_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_7_INTERVALS_SET,
    bass: None,
};

const MINOR_IV_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// iv9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_IV_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_9_INTERVALS_SET,
    bass: None,
};

const MINOR_IV_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// ivm7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_IV_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: MINOR_IV_M7_INTERVALS_SET,
    bass: None,
};

/// v chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_V: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_TRIAD_SET,
    bass: None,
};

const MINOR_V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// v7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_V_7_INTERVALS_SET,
    bass: None,
};

const MINOR_V_M7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MINOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// vm7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_V_M7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MINOR_V_M7_INTERVALS_SET,
    bass: None,
};

/// V chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const V_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// V7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_INTERVALS_SET,
    bass: None,
};

const V_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// V9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static V_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_9_INTERVALS_SET,
    bass: None,
};

const V_7_PLUS_FLAT_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MINOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// V7+b9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, minor ninth
pub static V_7_PLUS_FLAT_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: V_7_PLUS_FLAT_9_INTERVALS_SET,
    bass: None,
};

/// VI chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static VI: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VI7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_7_INTERVALS_SET,
    bass: None,
};

const VI_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// VI9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static VI_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_9_INTERVALS_SET,
    bass: None,
};

const VI_MAJ7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MAJOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VImaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static VI_MAJ7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: VI_MAJ7_INTERVALS_SET,
    bass: None,
};

/// VII chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static VII: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: MAJOR_TRIAD_SET,
    bass: None,
};

const VII_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// VII7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static VII_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: VII_7_INTERVALS_SET,
    bass: None,
};

const VII_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// VII9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static VII_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: VII_9_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_II_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// bII7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_II_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: MINOR_FLAT_II_7_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_VI_7_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 4);

/// bVI7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_VI_7: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_7_INTERVALS_SET,
    bass: None,
};

const MINOR_FLAT_VI_9_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(
    [Interval::PERFECT_UNISON,
     Interval::MAJOR_THIRD,
     Interval::PERFECT_FIFTH,
     Interval::MINOR_SEVENTH,
     Interval::MAJOR_NINTH,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE,
     Interval::NONE], 5);

/// bVI9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VI_9: RomanChord = RomanChord {
    root: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: MINOR_FLAT_VI_9_INTERVALS_SET,
    bass: None,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: MINOR_IV,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: MINOR_IV_7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: MINOR_IV_9,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: MINOR_IV_M7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: MINOR_IV,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: MINOR_IV_7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: MINOR_IV_9,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: MINOR_IV_M7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: MINOR_IV,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: MINOR_IV_7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: MINOR_IV_9,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: MINOR_IV_M7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: MINOR_IV,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: MINOR_IV_7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: MINOR_IV_9,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: MINOR_IV_M7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: MINOR_IV,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: MINOR_IV_7,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: MINOR_IV_9,
};

/// Progression edge: i → iv
pub static EDGE_MINOR_I_M9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: MINOR_IV_M7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: V,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: V_7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: V_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: V,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: V_7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: V_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: V,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: V_7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: V_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: V,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: V_7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: V_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: V,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: V_7,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: V_9,
};

/// Progression edge: i → V
pub static EDGE_MINOR_I_M9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_TO_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: VI,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: VI_7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: VI_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: VI_MAJ7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: VI,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: VI_7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: VI_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: VI_MAJ7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: VI,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: VI_7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: VI_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: VI_MAJ7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: VI,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: VI_7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: VI_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: VI_MAJ7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: VI,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: VI_7,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: VI_9,
};

/// Progression edge: i → VI
pub static EDGE_MINOR_I_M9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: VI_MAJ7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_TO_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: III,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: III_7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: III_9,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I,
    to: III_MAJ7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_7_TO_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: III,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: III_7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: III_9,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_7,
    to: III_MAJ7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_9_TO_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: III,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: III_7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: III_9,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_9,
    to: III_MAJ7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M7_TO_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: III,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: III_7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: III_9,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M7,
    to: III_MAJ7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M9_TO_III: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: III,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: III_7,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: III_9,
};

/// Progression edge: i → III
pub static EDGE_MINOR_I_M9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: MINOR_I_M9,
    to: III_MAJ7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_FLAT_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: VII,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: VII_7,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II,
    to: VII_9,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: VII,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: VII_7,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5,
    to: VII_9,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: VII,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: VII_7,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_M7_FLAT_5,
    to: VII_9,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: VII,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: VII_7,
};

/// Progression edge: ii → VII
pub static EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_II_FLAT_5_PLUS_7,
    to: VII_9,
};

/// Progression edge: III → VI
pub static EDGE_III_TO_VI: ProgressionEdge = ProgressionEdge {
    from: III,
    to: VI,
};

/// Progression edge: III → VI
pub static EDGE_III_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: III,
    to: VI_7,
};

/// Progression edge: III → VI
pub static EDGE_III_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: III,
    to: VI_9,
};

/// Progression edge: III → VI
pub static EDGE_III_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: III,
    to: VI_MAJ7,
};

/// Progression edge: III → VI
pub static EDGE_III_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: VI,
};

/// Progression edge: III → VI
pub static EDGE_III_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: VI_7,
};

/// Progression edge: III → VI
pub static EDGE_III_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: VI_9,
};

/// Progression edge: III → VI
pub static EDGE_III_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: VI_MAJ7,
};

/// Progression edge: III → VI
pub static EDGE_III_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: VI,
};

/// Progression edge: III → VI
pub static EDGE_III_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: VI_7,
};

/// Progression edge: III → VI
pub static EDGE_III_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: VI_9,
};

/// Progression edge: III → VI
pub static EDGE_III_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: VI_MAJ7,
};

/// Progression edge: III → VI
pub static EDGE_III_MAJ7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: VI,
};

/// Progression edge: III → VI
pub static EDGE_III_MAJ7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: VI_7,
};

/// Progression edge: III → VI
pub static EDGE_III_MAJ7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: VI_9,
};

/// Progression edge: III → VI
pub static EDGE_III_MAJ7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: VI_MAJ7,
};

/// Progression edge: III → iv
pub static EDGE_III_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: III,
    to: MINOR_IV,
};

/// Progression edge: III → iv
pub static EDGE_III_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: III,
    to: MINOR_IV_7,
};

/// Progression edge: III → iv
pub static EDGE_III_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: III,
    to: MINOR_IV_9,
};

/// Progression edge: III → iv
pub static EDGE_III_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: III,
    to: MINOR_IV_M7,
};

/// Progression edge: III → iv
pub static EDGE_III_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_IV,
};

/// Progression edge: III → iv
pub static EDGE_III_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_IV_7,
};

/// Progression edge: III → iv
pub static EDGE_III_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_IV_9,
};

/// Progression edge: III → iv
pub static EDGE_III_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: III_7,
    to: MINOR_IV_M7,
};

/// Progression edge: III → iv
pub static EDGE_III_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_IV,
};

/// Progression edge: III → iv
pub static EDGE_III_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_IV_7,
};

/// Progression edge: III → iv
pub static EDGE_III_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_IV_9,
};

/// Progression edge: III → iv
pub static EDGE_III_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: III_9,
    to: MINOR_IV_M7,
};

/// Progression edge: III → iv
pub static EDGE_III_MAJ7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: MINOR_IV,
};

/// Progression edge: III → iv
pub static EDGE_III_MAJ7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: MINOR_IV_7,
};

/// Progression edge: III → iv
pub static EDGE_III_MAJ7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: MINOR_IV_9,
};

/// Progression edge: III → iv
pub static EDGE_III_MAJ7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: III_MAJ7,
    to: MINOR_IV_M7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_M7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_I_M9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_I_7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_I_9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_I_M7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_I_M9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_I_7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_I_9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_I_M7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_I_M9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_9,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_M7,
};

/// Progression edge: iv → i
pub static EDGE_MINOR_IV_M7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_I_M9,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_II,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_II,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_7_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_7_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_II,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_9_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_9_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_9_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_M7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_II,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_M7_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_M7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: iv → ii
pub static EDGE_MINOR_IV_M7_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: V_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: V_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_9_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_7,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_9,
};

/// Progression edge: iv → V
pub static EDGE_MINOR_IV_M7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: VII,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: VII_7,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV,
    to: VII_9,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: VII,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: VII_7,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_7,
    to: VII_9,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_9_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: VII,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_9_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: VII_7,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_9_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_9,
    to: VII_9,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_M7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: VII,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_M7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: VII_7,
};

/// Progression edge: iv → VII
pub static EDGE_MINOR_IV_M7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_IV_M7,
    to: VII_9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_V,
    to: MINOR_I,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V,
    to: MINOR_I_7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V,
    to: MINOR_I_9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V,
    to: MINOR_I_M7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V,
    to: MINOR_I_M9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: MINOR_I,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: MINOR_I_7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: MINOR_I_9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: MINOR_I_M7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_7,
    to: MINOR_I_M9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_M7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_M7,
    to: MINOR_I,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_M7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_M7,
    to: MINOR_I_7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_M7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_M7,
    to: MINOR_I_9,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_M7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_M7,
    to: MINOR_I_M7,
};

/// Progression edge: v → i
pub static EDGE_MINOR_V_M7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: MINOR_V_M7,
    to: MINOR_I_M9,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_7,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_9,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_M7,
};

/// Progression edge: V → i
pub static EDGE_V_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: MINOR_I_M9,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_7,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_9,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_M7,
};

/// Progression edge: V → i
pub static EDGE_V_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: MINOR_I_M9,
};

/// Progression edge: V → i
pub static EDGE_V_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_I_7,
};

/// Progression edge: V → i
pub static EDGE_V_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_I_9,
};

/// Progression edge: V → i
pub static EDGE_V_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_I_M7,
};

/// Progression edge: V → i
pub static EDGE_V_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: MINOR_I_M9,
};

/// Progression edge: V → i
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_I,
};

/// Progression edge: V → i
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_I_7,
};

/// Progression edge: V → i
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_I_9,
};

/// Progression edge: V → i
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_I_M7,
};

/// Progression edge: V → i
pub static EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: MINOR_I_M9,
};

/// Progression edge: V → VI
pub static EDGE_V_TO_VI: ProgressionEdge = ProgressionEdge {
    from: V,
    to: VI,
};

/// Progression edge: V → VI
pub static EDGE_V_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: VI_7,
};

/// Progression edge: V → VI
pub static EDGE_V_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: V,
    to: VI_9,
};

/// Progression edge: V → VI
pub static EDGE_V_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V,
    to: VI_MAJ7,
};

/// Progression edge: V → VI
pub static EDGE_V_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: VI,
};

/// Progression edge: V → VI
pub static EDGE_V_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: VI_7,
};

/// Progression edge: V → VI
pub static EDGE_V_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: VI_9,
};

/// Progression edge: V → VI
pub static EDGE_V_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7,
    to: VI_MAJ7,
};

/// Progression edge: V → VI
pub static EDGE_V_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: VI,
};

/// Progression edge: V → VI
pub static EDGE_V_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: VI_7,
};

/// Progression edge: V → VI
pub static EDGE_V_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: VI_9,
};

/// Progression edge: V → VI
pub static EDGE_V_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_9,
    to: VI_MAJ7,
};

/// Progression edge: V → VI
pub static EDGE_V_7_PLUS_FLAT_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: VI,
};

/// Progression edge: V → VI
pub static EDGE_V_7_PLUS_FLAT_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: VI_7,
};

/// Progression edge: V → VI
pub static EDGE_V_7_PLUS_FLAT_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: VI_9,
};

/// Progression edge: V → VI
pub static EDGE_V_7_PLUS_FLAT_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: V_7_PLUS_FLAT_9,
    to: VI_MAJ7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_7_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_9_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: VI → ii
pub static EDGE_VI_MAJ7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_II,
};

/// Progression edge: VI → ii
pub static EDGE_VI_MAJ7_TO_MINOR_II_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_II_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_MAJ7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_II_M7_FLAT_5,
};

/// Progression edge: VI → ii
pub static EDGE_VI_MAJ7_TO_MINOR_II_FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_II_FLAT_5_PLUS_7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_IV,
};

/// Progression edge: VI → iv
pub static EDGE_VI_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_IV_7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_IV_9,
};

/// Progression edge: VI → iv
pub static EDGE_VI_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: VI,
    to: MINOR_IV_M7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_IV,
};

/// Progression edge: VI → iv
pub static EDGE_VI_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_IV_7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_IV_9,
};

/// Progression edge: VI → iv
pub static EDGE_VI_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: VI_7,
    to: MINOR_IV_M7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_IV,
};

/// Progression edge: VI → iv
pub static EDGE_VI_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_IV_7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_IV_9,
};

/// Progression edge: VI → iv
pub static EDGE_VI_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: VI_9,
    to: MINOR_IV_M7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_MAJ7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_IV,
};

/// Progression edge: VI → iv
pub static EDGE_VI_MAJ7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_IV_7,
};

/// Progression edge: VI → iv
pub static EDGE_VI_MAJ7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_IV_9,
};

/// Progression edge: VI → iv
pub static EDGE_VI_MAJ7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: VI_MAJ7,
    to: MINOR_IV_M7,
};

/// Progression edge: VII → III
pub static EDGE_VII_TO_III: ProgressionEdge = ProgressionEdge {
    from: VII,
    to: III,
};

/// Progression edge: VII → III
pub static EDGE_VII_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: VII,
    to: III_7,
};

/// Progression edge: VII → III
pub static EDGE_VII_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: VII,
    to: III_9,
};

/// Progression edge: VII → III
pub static EDGE_VII_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: VII,
    to: III_MAJ7,
};

/// Progression edge: VII → III
pub static EDGE_VII_7_TO_III: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: III,
};

/// Progression edge: VII → III
pub static EDGE_VII_7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: III_7,
};

/// Progression edge: VII → III
pub static EDGE_VII_7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: III_9,
};

/// Progression edge: VII → III
pub static EDGE_VII_7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: VII_7,
    to: III_MAJ7,
};

/// Progression edge: VII → III
pub static EDGE_VII_9_TO_III: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: III,
};

/// Progression edge: VII → III
pub static EDGE_VII_9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: III_7,
};

/// Progression edge: VII → III
pub static EDGE_VII_9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: III_9,
};

/// Progression edge: VII → III
pub static EDGE_VII_9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: VII_9,
    to: III_MAJ7,
};

/// Progression edge: bII → V
pub static EDGE_MINOR_FLAT_II_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_7,
    to: V,
};

/// Progression edge: bII → V
pub static EDGE_MINOR_FLAT_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_7,
    to: V_7,
};

/// Progression edge: bII → V
pub static EDGE_MINOR_FLAT_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_7,
    to: V_9,
};

/// Progression edge: bII → V
pub static EDGE_MINOR_FLAT_II_7_TO_V_7_PLUS_FLAT_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_II_7,
    to: V_7_PLUS_FLAT_9,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_7,
    to: VII,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_7,
    to: VII_7,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_7,
    to: VII_9,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_9_TO_VII: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_9,
    to: VII,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_9_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_9,
    to: VII_7,
};

/// Progression edge: bVI → VII
pub static EDGE_MINOR_FLAT_VI_9_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: MINOR_FLAT_VI_9,
    to: VII_9,
};

/// Complete registry of all progression chords for minor keys
/// 
/// Contains 34 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&RomanChord] = &[
    &MINOR_I,
    &MINOR_I_7,
    &MINOR_I_9,
    &MINOR_I_M7,
    &MINOR_I_M9,
    &MINOR_II,
    &MINOR_II_FLAT_5,
    &MINOR_II_M7_FLAT_5,
    &MINOR_II_FLAT_5_PLUS_7,
    &III,
    &III_7,
    &III_9,
    &III_MAJ7,
    &MINOR_IV,
    &MINOR_IV_7,
    &MINOR_IV_9,
    &MINOR_IV_M7,
    &MINOR_V,
    &MINOR_V_7,
    &MINOR_V_M7,
    &V,
    &V_7,
    &V_9,
    &V_7_PLUS_FLAT_9,
    &VI,
    &VI_7,
    &VI_9,
    &VI_MAJ7,
    &VII,
    &VII_7,
    &VII_9,
    &MINOR_FLAT_II_7,
    &MINOR_FLAT_VI_7,
    &MINOR_FLAT_VI_9,
];

/// Complete registry of all progression edges for minor keys
/// 
/// Contains 309 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_MINOR_I_TO_MINOR_IV,
    &EDGE_MINOR_I_TO_MINOR_IV_7,
    &EDGE_MINOR_I_TO_MINOR_IV_9,
    &EDGE_MINOR_I_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_7_TO_MINOR_IV,
    &EDGE_MINOR_I_7_TO_MINOR_IV_7,
    &EDGE_MINOR_I_7_TO_MINOR_IV_9,
    &EDGE_MINOR_I_7_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_9_TO_MINOR_IV,
    &EDGE_MINOR_I_9_TO_MINOR_IV_7,
    &EDGE_MINOR_I_9_TO_MINOR_IV_9,
    &EDGE_MINOR_I_9_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_M7_TO_MINOR_IV,
    &EDGE_MINOR_I_M7_TO_MINOR_IV_7,
    &EDGE_MINOR_I_M7_TO_MINOR_IV_9,
    &EDGE_MINOR_I_M7_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_M9_TO_MINOR_IV,
    &EDGE_MINOR_I_M9_TO_MINOR_IV_7,
    &EDGE_MINOR_I_M9_TO_MINOR_IV_9,
    &EDGE_MINOR_I_M9_TO_MINOR_IV_M7,
    &EDGE_MINOR_I_TO_V,
    &EDGE_MINOR_I_TO_V_7,
    &EDGE_MINOR_I_TO_V_9,
    &EDGE_MINOR_I_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_I_7_TO_V,
    &EDGE_MINOR_I_7_TO_V_7,
    &EDGE_MINOR_I_7_TO_V_9,
    &EDGE_MINOR_I_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_I_9_TO_V,
    &EDGE_MINOR_I_9_TO_V_7,
    &EDGE_MINOR_I_9_TO_V_9,
    &EDGE_MINOR_I_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_I_M7_TO_V,
    &EDGE_MINOR_I_M7_TO_V_7,
    &EDGE_MINOR_I_M7_TO_V_9,
    &EDGE_MINOR_I_M7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_I_M9_TO_V,
    &EDGE_MINOR_I_M9_TO_V_7,
    &EDGE_MINOR_I_M9_TO_V_9,
    &EDGE_MINOR_I_M9_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_I_TO_VI,
    &EDGE_MINOR_I_TO_VI_7,
    &EDGE_MINOR_I_TO_VI_9,
    &EDGE_MINOR_I_TO_VI_MAJ7,
    &EDGE_MINOR_I_7_TO_VI,
    &EDGE_MINOR_I_7_TO_VI_7,
    &EDGE_MINOR_I_7_TO_VI_9,
    &EDGE_MINOR_I_7_TO_VI_MAJ7,
    &EDGE_MINOR_I_9_TO_VI,
    &EDGE_MINOR_I_9_TO_VI_7,
    &EDGE_MINOR_I_9_TO_VI_9,
    &EDGE_MINOR_I_9_TO_VI_MAJ7,
    &EDGE_MINOR_I_M7_TO_VI,
    &EDGE_MINOR_I_M7_TO_VI_7,
    &EDGE_MINOR_I_M7_TO_VI_9,
    &EDGE_MINOR_I_M7_TO_VI_MAJ7,
    &EDGE_MINOR_I_M9_TO_VI,
    &EDGE_MINOR_I_M9_TO_VI_7,
    &EDGE_MINOR_I_M9_TO_VI_9,
    &EDGE_MINOR_I_M9_TO_VI_MAJ7,
    &EDGE_MINOR_I_TO_III,
    &EDGE_MINOR_I_TO_III_7,
    &EDGE_MINOR_I_TO_III_9,
    &EDGE_MINOR_I_TO_III_MAJ7,
    &EDGE_MINOR_I_7_TO_III,
    &EDGE_MINOR_I_7_TO_III_7,
    &EDGE_MINOR_I_7_TO_III_9,
    &EDGE_MINOR_I_7_TO_III_MAJ7,
    &EDGE_MINOR_I_9_TO_III,
    &EDGE_MINOR_I_9_TO_III_7,
    &EDGE_MINOR_I_9_TO_III_9,
    &EDGE_MINOR_I_9_TO_III_MAJ7,
    &EDGE_MINOR_I_M7_TO_III,
    &EDGE_MINOR_I_M7_TO_III_7,
    &EDGE_MINOR_I_M7_TO_III_9,
    &EDGE_MINOR_I_M7_TO_III_MAJ7,
    &EDGE_MINOR_I_M9_TO_III,
    &EDGE_MINOR_I_M9_TO_III_7,
    &EDGE_MINOR_I_M9_TO_III_9,
    &EDGE_MINOR_I_M9_TO_III_MAJ7,
    &EDGE_MINOR_II_TO_V,
    &EDGE_MINOR_II_TO_V_7,
    &EDGE_MINOR_II_TO_V_9,
    &EDGE_MINOR_II_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_FLAT_5_TO_V,
    &EDGE_MINOR_II_FLAT_5_TO_V_7,
    &EDGE_MINOR_II_FLAT_5_TO_V_9,
    &EDGE_MINOR_II_FLAT_5_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_7,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_7,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_9,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_II_TO_VII,
    &EDGE_MINOR_II_TO_VII_7,
    &EDGE_MINOR_II_TO_VII_9,
    &EDGE_MINOR_II_FLAT_5_TO_VII,
    &EDGE_MINOR_II_FLAT_5_TO_VII_7,
    &EDGE_MINOR_II_FLAT_5_TO_VII_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII_7,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII_9,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII_7,
    &EDGE_MINOR_II_FLAT_5_PLUS_7_TO_VII_9,
    &EDGE_III_TO_VI,
    &EDGE_III_TO_VI_7,
    &EDGE_III_TO_VI_9,
    &EDGE_III_TO_VI_MAJ7,
    &EDGE_III_7_TO_VI,
    &EDGE_III_7_TO_VI_7,
    &EDGE_III_7_TO_VI_9,
    &EDGE_III_7_TO_VI_MAJ7,
    &EDGE_III_9_TO_VI,
    &EDGE_III_9_TO_VI_7,
    &EDGE_III_9_TO_VI_9,
    &EDGE_III_9_TO_VI_MAJ7,
    &EDGE_III_MAJ7_TO_VI,
    &EDGE_III_MAJ7_TO_VI_7,
    &EDGE_III_MAJ7_TO_VI_9,
    &EDGE_III_MAJ7_TO_VI_MAJ7,
    &EDGE_III_TO_MINOR_IV,
    &EDGE_III_TO_MINOR_IV_7,
    &EDGE_III_TO_MINOR_IV_9,
    &EDGE_III_TO_MINOR_IV_M7,
    &EDGE_III_7_TO_MINOR_IV,
    &EDGE_III_7_TO_MINOR_IV_7,
    &EDGE_III_7_TO_MINOR_IV_9,
    &EDGE_III_7_TO_MINOR_IV_M7,
    &EDGE_III_9_TO_MINOR_IV,
    &EDGE_III_9_TO_MINOR_IV_7,
    &EDGE_III_9_TO_MINOR_IV_9,
    &EDGE_III_9_TO_MINOR_IV_M7,
    &EDGE_III_MAJ7_TO_MINOR_IV,
    &EDGE_III_MAJ7_TO_MINOR_IV_7,
    &EDGE_III_MAJ7_TO_MINOR_IV_9,
    &EDGE_III_MAJ7_TO_MINOR_IV_M7,
    &EDGE_MINOR_IV_TO_MINOR_I,
    &EDGE_MINOR_IV_TO_MINOR_I_7,
    &EDGE_MINOR_IV_TO_MINOR_I_9,
    &EDGE_MINOR_IV_TO_MINOR_I_M7,
    &EDGE_MINOR_IV_TO_MINOR_I_M9,
    &EDGE_MINOR_IV_7_TO_MINOR_I,
    &EDGE_MINOR_IV_7_TO_MINOR_I_7,
    &EDGE_MINOR_IV_7_TO_MINOR_I_9,
    &EDGE_MINOR_IV_7_TO_MINOR_I_M7,
    &EDGE_MINOR_IV_7_TO_MINOR_I_M9,
    &EDGE_MINOR_IV_9_TO_MINOR_I,
    &EDGE_MINOR_IV_9_TO_MINOR_I_7,
    &EDGE_MINOR_IV_9_TO_MINOR_I_9,
    &EDGE_MINOR_IV_9_TO_MINOR_I_M7,
    &EDGE_MINOR_IV_9_TO_MINOR_I_M9,
    &EDGE_MINOR_IV_M7_TO_MINOR_I,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_7,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_9,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_M7,
    &EDGE_MINOR_IV_M7_TO_MINOR_I_M9,
    &EDGE_MINOR_IV_TO_MINOR_II,
    &EDGE_MINOR_IV_TO_MINOR_II_FLAT_5,
    &EDGE_MINOR_IV_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_7_TO_MINOR_II,
    &EDGE_MINOR_IV_7_TO_MINOR_II_FLAT_5,
    &EDGE_MINOR_IV_7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_7_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_9_TO_MINOR_II,
    &EDGE_MINOR_IV_9_TO_MINOR_II_FLAT_5,
    &EDGE_MINOR_IV_9_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_9_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_M7_TO_MINOR_II,
    &EDGE_MINOR_IV_M7_TO_MINOR_II_FLAT_5,
    &EDGE_MINOR_IV_M7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_M7_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_TO_V,
    &EDGE_MINOR_IV_TO_V_7,
    &EDGE_MINOR_IV_TO_V_9,
    &EDGE_MINOR_IV_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_IV_7_TO_V,
    &EDGE_MINOR_IV_7_TO_V_7,
    &EDGE_MINOR_IV_7_TO_V_9,
    &EDGE_MINOR_IV_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_IV_9_TO_V,
    &EDGE_MINOR_IV_9_TO_V_7,
    &EDGE_MINOR_IV_9_TO_V_9,
    &EDGE_MINOR_IV_9_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_IV_M7_TO_V,
    &EDGE_MINOR_IV_M7_TO_V_7,
    &EDGE_MINOR_IV_M7_TO_V_9,
    &EDGE_MINOR_IV_M7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_IV_TO_VII,
    &EDGE_MINOR_IV_TO_VII_7,
    &EDGE_MINOR_IV_TO_VII_9,
    &EDGE_MINOR_IV_7_TO_VII,
    &EDGE_MINOR_IV_7_TO_VII_7,
    &EDGE_MINOR_IV_7_TO_VII_9,
    &EDGE_MINOR_IV_9_TO_VII,
    &EDGE_MINOR_IV_9_TO_VII_7,
    &EDGE_MINOR_IV_9_TO_VII_9,
    &EDGE_MINOR_IV_M7_TO_VII,
    &EDGE_MINOR_IV_M7_TO_VII_7,
    &EDGE_MINOR_IV_M7_TO_VII_9,
    &EDGE_MINOR_V_TO_MINOR_I,
    &EDGE_MINOR_V_TO_MINOR_I_7,
    &EDGE_MINOR_V_TO_MINOR_I_9,
    &EDGE_MINOR_V_TO_MINOR_I_M7,
    &EDGE_MINOR_V_TO_MINOR_I_M9,
    &EDGE_MINOR_V_7_TO_MINOR_I,
    &EDGE_MINOR_V_7_TO_MINOR_I_7,
    &EDGE_MINOR_V_7_TO_MINOR_I_9,
    &EDGE_MINOR_V_7_TO_MINOR_I_M7,
    &EDGE_MINOR_V_7_TO_MINOR_I_M9,
    &EDGE_MINOR_V_M7_TO_MINOR_I,
    &EDGE_MINOR_V_M7_TO_MINOR_I_7,
    &EDGE_MINOR_V_M7_TO_MINOR_I_9,
    &EDGE_MINOR_V_M7_TO_MINOR_I_M7,
    &EDGE_MINOR_V_M7_TO_MINOR_I_M9,
    &EDGE_V_TO_MINOR_I,
    &EDGE_V_TO_MINOR_I_7,
    &EDGE_V_TO_MINOR_I_9,
    &EDGE_V_TO_MINOR_I_M7,
    &EDGE_V_TO_MINOR_I_M9,
    &EDGE_V_7_TO_MINOR_I,
    &EDGE_V_7_TO_MINOR_I_7,
    &EDGE_V_7_TO_MINOR_I_9,
    &EDGE_V_7_TO_MINOR_I_M7,
    &EDGE_V_7_TO_MINOR_I_M9,
    &EDGE_V_9_TO_MINOR_I,
    &EDGE_V_9_TO_MINOR_I_7,
    &EDGE_V_9_TO_MINOR_I_9,
    &EDGE_V_9_TO_MINOR_I_M7,
    &EDGE_V_9_TO_MINOR_I_M9,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_7,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_9,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_M7,
    &EDGE_V_7_PLUS_FLAT_9_TO_MINOR_I_M9,
    &EDGE_V_TO_VI,
    &EDGE_V_TO_VI_7,
    &EDGE_V_TO_VI_9,
    &EDGE_V_TO_VI_MAJ7,
    &EDGE_V_7_TO_VI,
    &EDGE_V_7_TO_VI_7,
    &EDGE_V_7_TO_VI_9,
    &EDGE_V_7_TO_VI_MAJ7,
    &EDGE_V_9_TO_VI,
    &EDGE_V_9_TO_VI_7,
    &EDGE_V_9_TO_VI_9,
    &EDGE_V_9_TO_VI_MAJ7,
    &EDGE_V_7_PLUS_FLAT_9_TO_VI,
    &EDGE_V_7_PLUS_FLAT_9_TO_VI_7,
    &EDGE_V_7_PLUS_FLAT_9_TO_VI_9,
    &EDGE_V_7_PLUS_FLAT_9_TO_VI_MAJ7,
    &EDGE_VI_TO_MINOR_II,
    &EDGE_VI_TO_MINOR_II_FLAT_5,
    &EDGE_VI_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_VI_7_TO_MINOR_II,
    &EDGE_VI_7_TO_MINOR_II_FLAT_5,
    &EDGE_VI_7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_7_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_VI_9_TO_MINOR_II,
    &EDGE_VI_9_TO_MINOR_II_FLAT_5,
    &EDGE_VI_9_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_9_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_VI_MAJ7_TO_MINOR_II,
    &EDGE_VI_MAJ7_TO_MINOR_II_FLAT_5,
    &EDGE_VI_MAJ7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_MAJ7_TO_MINOR_II_FLAT_5_PLUS_7,
    &EDGE_VI_TO_MINOR_IV,
    &EDGE_VI_TO_MINOR_IV_7,
    &EDGE_VI_TO_MINOR_IV_9,
    &EDGE_VI_TO_MINOR_IV_M7,
    &EDGE_VI_7_TO_MINOR_IV,
    &EDGE_VI_7_TO_MINOR_IV_7,
    &EDGE_VI_7_TO_MINOR_IV_9,
    &EDGE_VI_7_TO_MINOR_IV_M7,
    &EDGE_VI_9_TO_MINOR_IV,
    &EDGE_VI_9_TO_MINOR_IV_7,
    &EDGE_VI_9_TO_MINOR_IV_9,
    &EDGE_VI_9_TO_MINOR_IV_M7,
    &EDGE_VI_MAJ7_TO_MINOR_IV,
    &EDGE_VI_MAJ7_TO_MINOR_IV_7,
    &EDGE_VI_MAJ7_TO_MINOR_IV_9,
    &EDGE_VI_MAJ7_TO_MINOR_IV_M7,
    &EDGE_VII_TO_III,
    &EDGE_VII_TO_III_7,
    &EDGE_VII_TO_III_9,
    &EDGE_VII_TO_III_MAJ7,
    &EDGE_VII_7_TO_III,
    &EDGE_VII_7_TO_III_7,
    &EDGE_VII_7_TO_III_9,
    &EDGE_VII_7_TO_III_MAJ7,
    &EDGE_VII_9_TO_III,
    &EDGE_VII_9_TO_III_7,
    &EDGE_VII_9_TO_III_9,
    &EDGE_VII_9_TO_III_MAJ7,
    &EDGE_MINOR_FLAT_II_7_TO_V,
    &EDGE_MINOR_FLAT_II_7_TO_V_7,
    &EDGE_MINOR_FLAT_II_7_TO_V_9,
    &EDGE_MINOR_FLAT_II_7_TO_V_7_PLUS_FLAT_9,
    &EDGE_MINOR_FLAT_VI_7_TO_VII,
    &EDGE_MINOR_FLAT_VI_7_TO_VII_7,
    &EDGE_MINOR_FLAT_VI_7_TO_VII_9,
    &EDGE_MINOR_FLAT_VI_9_TO_VII,
    &EDGE_MINOR_FLAT_VI_9_TO_VII_7,
    &EDGE_MINOR_FLAT_VI_9_TO_VII_9,
];

/// NodeType mapping for all progression chords in minor keys
/// 
/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).
pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {
    let mut map = HashMap::new();
    map.insert(&MINOR_I, NodeType::Primary);
    map.insert(&MINOR_I_7, NodeType::Primary);
    map.insert(&MINOR_I_9, NodeType::Primary);
    map.insert(&MINOR_I_M7, NodeType::Primary);
    map.insert(&MINOR_I_M9, NodeType::Primary);
    map.insert(&MINOR_II, NodeType::Primary);
    map.insert(&MINOR_II_FLAT_5, NodeType::Primary);
    map.insert(&MINOR_II_M7_FLAT_5, NodeType::Primary);
    map.insert(&MINOR_II_FLAT_5_PLUS_7, NodeType::Primary);
    map.insert(&III, NodeType::Primary);
    map.insert(&III_7, NodeType::Primary);
    map.insert(&III_9, NodeType::Primary);
    map.insert(&III_MAJ7, NodeType::Primary);
    map.insert(&MINOR_IV, NodeType::Primary);
    map.insert(&MINOR_IV_7, NodeType::Primary);
    map.insert(&MINOR_IV_9, NodeType::Primary);
    map.insert(&MINOR_IV_M7, NodeType::Primary);
    map.insert(&MINOR_V, NodeType::Primary);
    map.insert(&MINOR_V_7, NodeType::Primary);
    map.insert(&MINOR_V_M7, NodeType::Primary);
    map.insert(&V, NodeType::Primary);
    map.insert(&V_7, NodeType::Primary);
    map.insert(&V_9, NodeType::Primary);
    map.insert(&V_7_PLUS_FLAT_9, NodeType::Primary);
    map.insert(&VI, NodeType::Primary);
    map.insert(&VI_7, NodeType::Primary);
    map.insert(&VI_9, NodeType::Primary);
    map.insert(&VI_MAJ7, NodeType::Primary);
    map.insert(&VII, NodeType::Primary);
    map.insert(&VII_7, NodeType::Primary);
    map.insert(&VII_9, NodeType::Primary);
    map.insert(&MINOR_FLAT_II_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_7, NodeType::Secondary);
    map.insert(&MINOR_FLAT_VI_9, NodeType::Secondary);
    map
}

/// Look up a progression chord by its display name for minor keys
/// 
/// Returns the corresponding `RomanChord` for chord symbols like "I", "V7", "ii9", etc.
/// Supports 34 different chord variants.
pub fn get_node(name: &str) -> Option<&'static RomanChord> {
    match name {
        "i" => Some(&MINOR_I),
        "i7" => Some(&MINOR_I_7),
        "i9" => Some(&MINOR_I_9),
        "im7" => Some(&MINOR_I_M7),
        "im9" => Some(&MINOR_I_M9),
        "ii" => Some(&MINOR_II),
        "iib5" => Some(&MINOR_II_FLAT_5),
        "iim7b5" => Some(&MINOR_II_M7_FLAT_5),
        "iib5+7" => Some(&MINOR_II_FLAT_5_PLUS_7),
        "III" => Some(&III),
        "III7" => Some(&III_7),
        "III9" => Some(&III_9),
        "IIImaj7" => Some(&III_MAJ7),
        "iv" => Some(&MINOR_IV),
        "iv7" => Some(&MINOR_IV_7),
        "iv9" => Some(&MINOR_IV_9),
        "ivm7" => Some(&MINOR_IV_M7),
        "v" => Some(&MINOR_V),
        "v7" => Some(&MINOR_V_7),
        "vm7" => Some(&MINOR_V_M7),
        "V" => Some(&V),
        "V7" => Some(&V_7),
        "V9" => Some(&V_9),
        "V7+b9" => Some(&V_7_PLUS_FLAT_9),
        "VI" => Some(&VI),
        "VI7" => Some(&VI_7),
        "VI9" => Some(&VI_9),
        "VImaj7" => Some(&VI_MAJ7),
        "VII" => Some(&VII),
        "VII7" => Some(&VII_7),
        "VII9" => Some(&VII_9),
        "bII7" => Some(&MINOR_FLAT_II_7),
        "bVI7" => Some(&MINOR_FLAT_VI_7),
        "bVI9" => Some(&MINOR_FLAT_VI_9),
        _ => None,
    }
}
