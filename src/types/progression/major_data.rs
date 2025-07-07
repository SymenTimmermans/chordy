//! Generated progression data for major keys from major.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionNode, ProgressionEdge, NodeType};
use crate::types::{RomanNumeral, RomanDegree, Accidental, Interval};

// Common interval patterns (reused across multiple chords)
/// Standard major triad intervals: root, major third, perfect fifth
static MAJOR_TRIAD: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];
/// Standard minor triad intervals: root, minor third, perfect fifth
static MINOR_TRIAD: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];

/// I chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static I: ProgressionNode = ProgressionNode {
    id: "I",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MAJOR_TRIAD,
};

static I_6_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH];

/// I6 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static I_6: ProgressionNode = ProgressionNode {
    id: "I6",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_6_INTERVALS,
};

static I_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// I7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static I_7: ProgressionNode = ProgressionNode {
    id: "I7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_7_INTERVALS,
};

static I_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// I9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static I_9: ProgressionNode = ProgressionNode {
    id: "I9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_9_INTERVALS,
};

static I_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

/// Imaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static I_MAJ7: ProgressionNode = ProgressionNode {
    id: "Imaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_MAJ7_INTERVALS,
};

static I_MAJ9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH, Interval::MAJOR_NINTH];

/// Imaj9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh, major ninth
pub static I_MAJ9: ProgressionNode = ProgressionNode {
    id: "Imaj9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_MAJ9_INTERVALS,
};

/// ii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_II: ProgressionNode = ProgressionNode {
    id: "ii",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_TRIAD,
};

static MINOR_II_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// ii7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_II_7: ProgressionNode = ProgressionNode {
    id: "ii7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_7_INTERVALS,
};

static MINOR_II_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// ii9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_II_9: ProgressionNode = ProgressionNode {
    id: "ii9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_9_INTERVALS,
};

static MINOR_II_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH];

/// ii11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth, perfect eleventh
pub static MINOR_II_11: ProgressionNode = ProgressionNode {
    id: "ii11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_11_INTERVALS,
};

static MINOR_II_7_PLUS__FLAT_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MINOR_NINTH];

/// ii7+b9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, minor ninth
pub static MINOR_II_7_PLUS__FLAT_9: ProgressionNode = ProgressionNode {
    id: "ii7+b9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_7_PLUS__FLAT_9_INTERVALS,
};

/// iii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_III: ProgressionNode = ProgressionNode {
    id: "iii",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_TRIAD,
};

static MINOR_III_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// iii7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_III_7: ProgressionNode = ProgressionNode {
    id: "iii7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_III_7_INTERVALS,
};

static MINOR_III_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// iiim7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_III_M7: ProgressionNode = ProgressionNode {
    id: "iiim7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_III_M7_INTERVALS,
};

/// IV chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static IV: ProgressionNode = ProgressionNode {
    id: "IV",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &MAJOR_TRIAD,
};

static IV_6_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH];

/// IV6 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major sixth
pub static IV_6: ProgressionNode = ProgressionNode {
    id: "IV6",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_6_INTERVALS,
};

static IV_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// IV7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static IV_7: ProgressionNode = ProgressionNode {
    id: "IV7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_7_INTERVALS,
};

static IV_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// IV9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static IV_9: ProgressionNode = ProgressionNode {
    id: "IV9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_9_INTERVALS,
};

static IV_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

/// IVmaj7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh
pub static IV_MAJ7: ProgressionNode = ProgressionNode {
    id: "IVmaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_MAJ7_INTERVALS,
};

static IV__SHARP_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH, Interval::MAJOR_NINTH, Interval::AUGMENTED_ELEVENTH];

/// IV#11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, major seventh, major ninth, augmented eleventh
pub static IV__SHARP_11: ProgressionNode = ProgressionNode {
    id: "IV#11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV__SHARP_11_INTERVALS,
};

/// V chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth
pub static V: ProgressionNode = ProgressionNode {
    id: "V",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &MAJOR_TRIAD,
};

static V_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// V7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static V_7: ProgressionNode = ProgressionNode {
    id: "V7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_INTERVALS,
};

static V_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// V9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static V_9: ProgressionNode = ProgressionNode {
    id: "V9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_9_INTERVALS,
};

static V_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH];

/// V11 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh
pub static V_11: ProgressionNode = ProgressionNode {
    id: "V11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_11_INTERVALS,
};

static V_13_INTERVALS: [Interval; 7] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH, Interval::MAJOR_THIRTEENTH];

/// V13 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth, perfect eleventh, major thirteenth
pub static V_13: ProgressionNode = ProgressionNode {
    id: "V13",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_13_INTERVALS,
};

static V_7_PLUS__FLAT_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MINOR_NINTH];

/// V7+b9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, minor ninth
pub static V_7_PLUS__FLAT_9: ProgressionNode = ProgressionNode {
    id: "V7+b9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_PLUS__FLAT_9_INTERVALS,
};

static V_7_PLUS__SHARP_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::AUGMENTED_NINTH];

/// V7+#9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, augmented ninth
pub static V_7_PLUS__SHARP_9: ProgressionNode = ProgressionNode {
    id: "V7+#9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_PLUS__SHARP_9_INTERVALS,
};

/// vi chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VI: ProgressionNode = ProgressionNode {
    id: "vi",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_TRIAD,
};

static MINOR_VI_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// vi7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VI_7: ProgressionNode = ProgressionNode {
    id: "vi7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_7_INTERVALS,
};

static MINOR_VI_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// vi9 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh, major ninth
pub static MINOR_VI_9: ProgressionNode = ProgressionNode {
    id: "vi9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_9_INTERVALS,
};

static MINOR_VI_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// vim7 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth, minor seventh
pub static MINOR_VI_M7: ProgressionNode = ProgressionNode {
    id: "vim7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_M7_INTERVALS,
};

/// vii chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, perfect fifth
pub static MINOR_VII: ProgressionNode = ProgressionNode {
    id: "vii",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_TRIAD,
};

static MINOR_VII__FLAT_5_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH];

/// viib5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth
pub static MINOR_VII__FLAT_5: ProgressionNode = ProgressionNode {
    id: "viib5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_VII__FLAT_5_INTERVALS,
};

static MINOR_VII_M7_FLAT_5_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH];

/// viim7b5 chord - stable harmonic center (primary node)
/// Intervals: perfect unison, minor third, diminished fifth, minor seventh
pub static MINOR_VII_M7_FLAT_5: ProgressionNode = ProgressionNode {
    id: "viim7b5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_VII_M7_FLAT_5_INTERVALS,
};

static MINOR_FLAT_III_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// bIII7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_III_7: ProgressionNode = ProgressionNode {
    id: "bIII7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_FLAT_III_7_INTERVALS,
};

static MINOR_FLAT_III_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// bIII9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_III_9: ProgressionNode = ProgressionNode {
    id: "bIII9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_FLAT_III_9_INTERVALS,
};

static MINOR_FLAT_VI_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// bVI7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_VI_7: ProgressionNode = ProgressionNode {
    id: "bVI7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_FLAT_VI_7_INTERVALS,
};

static MINOR_FLAT_VI_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// bVI9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VI_9: ProgressionNode = ProgressionNode {
    id: "bVI9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_FLAT_VI_9_INTERVALS,
};

static MINOR_FLAT_VII_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

/// bVII7 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh
pub static MINOR_FLAT_VII_7: ProgressionNode = ProgressionNode {
    id: "bVII7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_FLAT_VII_7_INTERVALS,
};

static MINOR_FLAT_VII_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

/// bVII9 chord - creates tension, seeks resolution (secondary node)
/// Intervals: perfect unison, major third, perfect fifth, minor seventh, major ninth
pub static MINOR_FLAT_VII_9: ProgressionNode = ProgressionNode {
    id: "bVII9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_FLAT_VII_9_INTERVALS,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV__SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_6_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV__SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV__SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV__SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV__SHARP_11,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_6,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_9,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_MAJ7,
};

/// Progression edge: I → IV
pub static EDGE_I_MAJ9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV__SHARP_11,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_6_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_11,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_13,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: I → V
pub static EDGE_I_MAJ9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_6_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_M7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_7,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_9,
};

/// Progression edge: I → vi
pub static EDGE_I_MAJ9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_M7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_11_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_11,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_13,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: ii → V
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &MINOR_VII__FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &MINOR_VII__FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &MINOR_VII__FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &MINOR_VII__FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &MINOR_VII_M7_FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &MINOR_VII,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &MINOR_VII__FLAT_5,
};

/// Progression edge: ii → vii
pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &MINOR_VII_M7_FLAT_5,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_M7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_7,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_9,
};

/// Progression edge: iii → vi
pub static EDGE_MINOR_III_M7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_M7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III,
    to: &IV__SHARP_11,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV__SHARP_11,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_6,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_9,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_MAJ7,
};

/// Progression edge: iii → IV
pub static EDGE_MINOR_III_M7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV__SHARP_11,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_6_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV_MAJ7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_MAJ9,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_6,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_7,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_9,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_MAJ7,
};

/// Progression edge: IV → I
pub static EDGE_IV__SHARP_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_MAJ9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_6_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_9_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV__SHARP_11_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II,
};

/// Progression edge: IV → ii
pub static EDGE_IV__SHARP_11_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_7,
};

/// Progression edge: IV → ii
pub static EDGE_IV__SHARP_11_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_9,
};

/// Progression edge: IV → ii
pub static EDGE_IV__SHARP_11_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_11,
};

/// Progression edge: IV → ii
pub static EDGE_IV__SHARP_11_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_6_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV_MAJ7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_9,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_11,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_13,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7_PLUS__FLAT_9,
};

/// Progression edge: IV → V
pub static EDGE_IV__SHARP_11_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7_PLUS__SHARP_9,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_13_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_MAJ9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_6,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_9,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_MAJ7,
};

/// Progression edge: V → I
pub static EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_MAJ9,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_11_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_13_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_M7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_7,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_9,
};

/// Progression edge: V → vi
pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_M7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_7,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_9,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_11,
};

/// Progression edge: vi → ii
pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI,
    to: &IV__SHARP_11,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV__SHARP_11,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV__SHARP_11,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_6,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_9,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_MAJ7,
};

/// Progression edge: vi → IV
pub static EDGE_MINOR_VI_M7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV__SHARP_11,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII,
    to: &I_MAJ9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII__FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_MAJ9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_6,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_9,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_MAJ7,
};

/// Progression edge: vii → I
pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_MAJ9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_6,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_MAJ7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV__SHARP_11,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_6,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_9,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_MAJ7,
};

/// Progression edge: bIII → IV
pub static EDGE_MINOR_FLAT_III_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV__SHARP_11,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &MINOR_FLAT_VII_7,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &MINOR_FLAT_VII_9,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &MINOR_FLAT_VII_7,
};

/// Progression edge: bVI → bVII
pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &MINOR_FLAT_VII_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_6,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_MAJ7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_MAJ9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_6,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_9,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_MAJ7,
};

/// Progression edge: bVII → I
pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_MAJ9,
};

/// Complete registry of all progression nodes for major keys
/// 
/// Contains 40 chord variants across all harmonic functions.
/// Used internally for graph traversal and chord lookup operations.
pub static ALL_NODES: &[&ProgressionNode] = &[
    &I,
    &I_6,
    &I_7,
    &I_9,
    &I_MAJ7,
    &I_MAJ9,
    &MINOR_II,
    &MINOR_II_7,
    &MINOR_II_9,
    &MINOR_II_11,
    &MINOR_II_7_PLUS__FLAT_9,
    &MINOR_III,
    &MINOR_III_7,
    &MINOR_III_M7,
    &IV,
    &IV_6,
    &IV_7,
    &IV_9,
    &IV_MAJ7,
    &IV__SHARP_11,
    &V,
    &V_7,
    &V_9,
    &V_11,
    &V_13,
    &V_7_PLUS__FLAT_9,
    &V_7_PLUS__SHARP_9,
    &MINOR_VI,
    &MINOR_VI_7,
    &MINOR_VI_9,
    &MINOR_VI_M7,
    &MINOR_VII,
    &MINOR_VII__FLAT_5,
    &MINOR_VII_M7_FLAT_5,
    &MINOR_FLAT_III_7,
    &MINOR_FLAT_III_9,
    &MINOR_FLAT_VI_7,
    &MINOR_FLAT_VI_9,
    &MINOR_FLAT_VII_7,
    &MINOR_FLAT_VII_9,
];

/// Complete registry of all progression edges for major keys
/// 
/// Contains 450 harmonic connections between chord variants.
/// Each edge represents a musically valid progression with proper voice leading.
pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_I_TO_IV,
    &EDGE_I_TO_IV_6,
    &EDGE_I_TO_IV_7,
    &EDGE_I_TO_IV_9,
    &EDGE_I_TO_IV_MAJ7,
    &EDGE_I_TO_IV__SHARP_11,
    &EDGE_I_6_TO_IV,
    &EDGE_I_6_TO_IV_6,
    &EDGE_I_6_TO_IV_7,
    &EDGE_I_6_TO_IV_9,
    &EDGE_I_6_TO_IV_MAJ7,
    &EDGE_I_6_TO_IV__SHARP_11,
    &EDGE_I_7_TO_IV,
    &EDGE_I_7_TO_IV_6,
    &EDGE_I_7_TO_IV_7,
    &EDGE_I_7_TO_IV_9,
    &EDGE_I_7_TO_IV_MAJ7,
    &EDGE_I_7_TO_IV__SHARP_11,
    &EDGE_I_9_TO_IV,
    &EDGE_I_9_TO_IV_6,
    &EDGE_I_9_TO_IV_7,
    &EDGE_I_9_TO_IV_9,
    &EDGE_I_9_TO_IV_MAJ7,
    &EDGE_I_9_TO_IV__SHARP_11,
    &EDGE_I_MAJ7_TO_IV,
    &EDGE_I_MAJ7_TO_IV_6,
    &EDGE_I_MAJ7_TO_IV_7,
    &EDGE_I_MAJ7_TO_IV_9,
    &EDGE_I_MAJ7_TO_IV_MAJ7,
    &EDGE_I_MAJ7_TO_IV__SHARP_11,
    &EDGE_I_MAJ9_TO_IV,
    &EDGE_I_MAJ9_TO_IV_6,
    &EDGE_I_MAJ9_TO_IV_7,
    &EDGE_I_MAJ9_TO_IV_9,
    &EDGE_I_MAJ9_TO_IV_MAJ7,
    &EDGE_I_MAJ9_TO_IV__SHARP_11,
    &EDGE_I_TO_V,
    &EDGE_I_TO_V_7,
    &EDGE_I_TO_V_9,
    &EDGE_I_TO_V_11,
    &EDGE_I_TO_V_13,
    &EDGE_I_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_6_TO_V,
    &EDGE_I_6_TO_V_7,
    &EDGE_I_6_TO_V_9,
    &EDGE_I_6_TO_V_11,
    &EDGE_I_6_TO_V_13,
    &EDGE_I_6_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_6_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_7_TO_V,
    &EDGE_I_7_TO_V_7,
    &EDGE_I_7_TO_V_9,
    &EDGE_I_7_TO_V_11,
    &EDGE_I_7_TO_V_13,
    &EDGE_I_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_9_TO_V,
    &EDGE_I_9_TO_V_7,
    &EDGE_I_9_TO_V_9,
    &EDGE_I_9_TO_V_11,
    &EDGE_I_9_TO_V_13,
    &EDGE_I_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_MAJ7_TO_V,
    &EDGE_I_MAJ7_TO_V_7,
    &EDGE_I_MAJ7_TO_V_9,
    &EDGE_I_MAJ7_TO_V_11,
    &EDGE_I_MAJ7_TO_V_13,
    &EDGE_I_MAJ7_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_MAJ7_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_MAJ9_TO_V,
    &EDGE_I_MAJ9_TO_V_7,
    &EDGE_I_MAJ9_TO_V_9,
    &EDGE_I_MAJ9_TO_V_11,
    &EDGE_I_MAJ9_TO_V_13,
    &EDGE_I_MAJ9_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_MAJ9_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_TO_MINOR_VI,
    &EDGE_I_TO_MINOR_VI_7,
    &EDGE_I_TO_MINOR_VI_9,
    &EDGE_I_TO_MINOR_VI_M7,
    &EDGE_I_6_TO_MINOR_VI,
    &EDGE_I_6_TO_MINOR_VI_7,
    &EDGE_I_6_TO_MINOR_VI_9,
    &EDGE_I_6_TO_MINOR_VI_M7,
    &EDGE_I_7_TO_MINOR_VI,
    &EDGE_I_7_TO_MINOR_VI_7,
    &EDGE_I_7_TO_MINOR_VI_9,
    &EDGE_I_7_TO_MINOR_VI_M7,
    &EDGE_I_9_TO_MINOR_VI,
    &EDGE_I_9_TO_MINOR_VI_7,
    &EDGE_I_9_TO_MINOR_VI_9,
    &EDGE_I_9_TO_MINOR_VI_M7,
    &EDGE_I_MAJ7_TO_MINOR_VI,
    &EDGE_I_MAJ7_TO_MINOR_VI_7,
    &EDGE_I_MAJ7_TO_MINOR_VI_9,
    &EDGE_I_MAJ7_TO_MINOR_VI_M7,
    &EDGE_I_MAJ9_TO_MINOR_VI,
    &EDGE_I_MAJ9_TO_MINOR_VI_7,
    &EDGE_I_MAJ9_TO_MINOR_VI_9,
    &EDGE_I_MAJ9_TO_MINOR_VI_M7,
    &EDGE_MINOR_II_TO_V,
    &EDGE_MINOR_II_TO_V_7,
    &EDGE_MINOR_II_TO_V_9,
    &EDGE_MINOR_II_TO_V_11,
    &EDGE_MINOR_II_TO_V_13,
    &EDGE_MINOR_II_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_7_TO_V,
    &EDGE_MINOR_II_7_TO_V_7,
    &EDGE_MINOR_II_7_TO_V_9,
    &EDGE_MINOR_II_7_TO_V_11,
    &EDGE_MINOR_II_7_TO_V_13,
    &EDGE_MINOR_II_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_9_TO_V,
    &EDGE_MINOR_II_9_TO_V_7,
    &EDGE_MINOR_II_9_TO_V_9,
    &EDGE_MINOR_II_9_TO_V_11,
    &EDGE_MINOR_II_9_TO_V_13,
    &EDGE_MINOR_II_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_11_TO_V,
    &EDGE_MINOR_II_11_TO_V_7,
    &EDGE_MINOR_II_11_TO_V_9,
    &EDGE_MINOR_II_11_TO_V_11,
    &EDGE_MINOR_II_11_TO_V_13,
    &EDGE_MINOR_II_11_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_11_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_11,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_13,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_TO_MINOR_VII,
    &EDGE_MINOR_II_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_7_TO_MINOR_VII,
    &EDGE_MINOR_II_7_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII,
    &EDGE_MINOR_II_9_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII,
    &EDGE_MINOR_II_11_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_III_TO_MINOR_VI,
    &EDGE_MINOR_III_TO_MINOR_VI_7,
    &EDGE_MINOR_III_TO_MINOR_VI_9,
    &EDGE_MINOR_III_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_7_TO_MINOR_VI,
    &EDGE_MINOR_III_7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_TO_IV,
    &EDGE_MINOR_III_TO_IV_6,
    &EDGE_MINOR_III_TO_IV_7,
    &EDGE_MINOR_III_TO_IV_9,
    &EDGE_MINOR_III_TO_IV_MAJ7,
    &EDGE_MINOR_III_TO_IV__SHARP_11,
    &EDGE_MINOR_III_7_TO_IV,
    &EDGE_MINOR_III_7_TO_IV_6,
    &EDGE_MINOR_III_7_TO_IV_7,
    &EDGE_MINOR_III_7_TO_IV_9,
    &EDGE_MINOR_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_III_7_TO_IV__SHARP_11,
    &EDGE_MINOR_III_M7_TO_IV,
    &EDGE_MINOR_III_M7_TO_IV_6,
    &EDGE_MINOR_III_M7_TO_IV_7,
    &EDGE_MINOR_III_M7_TO_IV_9,
    &EDGE_MINOR_III_M7_TO_IV_MAJ7,
    &EDGE_MINOR_III_M7_TO_IV__SHARP_11,
    &EDGE_IV_TO_I,
    &EDGE_IV_TO_I_6,
    &EDGE_IV_TO_I_7,
    &EDGE_IV_TO_I_9,
    &EDGE_IV_TO_I_MAJ7,
    &EDGE_IV_TO_I_MAJ9,
    &EDGE_IV_6_TO_I,
    &EDGE_IV_6_TO_I_6,
    &EDGE_IV_6_TO_I_7,
    &EDGE_IV_6_TO_I_9,
    &EDGE_IV_6_TO_I_MAJ7,
    &EDGE_IV_6_TO_I_MAJ9,
    &EDGE_IV_7_TO_I,
    &EDGE_IV_7_TO_I_6,
    &EDGE_IV_7_TO_I_7,
    &EDGE_IV_7_TO_I_9,
    &EDGE_IV_7_TO_I_MAJ7,
    &EDGE_IV_7_TO_I_MAJ9,
    &EDGE_IV_9_TO_I,
    &EDGE_IV_9_TO_I_6,
    &EDGE_IV_9_TO_I_7,
    &EDGE_IV_9_TO_I_9,
    &EDGE_IV_9_TO_I_MAJ7,
    &EDGE_IV_9_TO_I_MAJ9,
    &EDGE_IV_MAJ7_TO_I,
    &EDGE_IV_MAJ7_TO_I_6,
    &EDGE_IV_MAJ7_TO_I_7,
    &EDGE_IV_MAJ7_TO_I_9,
    &EDGE_IV_MAJ7_TO_I_MAJ7,
    &EDGE_IV_MAJ7_TO_I_MAJ9,
    &EDGE_IV__SHARP_11_TO_I,
    &EDGE_IV__SHARP_11_TO_I_6,
    &EDGE_IV__SHARP_11_TO_I_7,
    &EDGE_IV__SHARP_11_TO_I_9,
    &EDGE_IV__SHARP_11_TO_I_MAJ7,
    &EDGE_IV__SHARP_11_TO_I_MAJ9,
    &EDGE_IV_TO_MINOR_II,
    &EDGE_IV_TO_MINOR_II_7,
    &EDGE_IV_TO_MINOR_II_9,
    &EDGE_IV_TO_MINOR_II_11,
    &EDGE_IV_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_6_TO_MINOR_II,
    &EDGE_IV_6_TO_MINOR_II_7,
    &EDGE_IV_6_TO_MINOR_II_9,
    &EDGE_IV_6_TO_MINOR_II_11,
    &EDGE_IV_6_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_7_TO_MINOR_II,
    &EDGE_IV_7_TO_MINOR_II_7,
    &EDGE_IV_7_TO_MINOR_II_9,
    &EDGE_IV_7_TO_MINOR_II_11,
    &EDGE_IV_7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_9_TO_MINOR_II,
    &EDGE_IV_9_TO_MINOR_II_7,
    &EDGE_IV_9_TO_MINOR_II_9,
    &EDGE_IV_9_TO_MINOR_II_11,
    &EDGE_IV_9_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_MAJ7_TO_MINOR_II,
    &EDGE_IV_MAJ7_TO_MINOR_II_7,
    &EDGE_IV_MAJ7_TO_MINOR_II_9,
    &EDGE_IV_MAJ7_TO_MINOR_II_11,
    &EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV__SHARP_11_TO_MINOR_II,
    &EDGE_IV__SHARP_11_TO_MINOR_II_7,
    &EDGE_IV__SHARP_11_TO_MINOR_II_9,
    &EDGE_IV__SHARP_11_TO_MINOR_II_11,
    &EDGE_IV__SHARP_11_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_TO_V,
    &EDGE_IV_TO_V_7,
    &EDGE_IV_TO_V_9,
    &EDGE_IV_TO_V_11,
    &EDGE_IV_TO_V_13,
    &EDGE_IV_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_6_TO_V,
    &EDGE_IV_6_TO_V_7,
    &EDGE_IV_6_TO_V_9,
    &EDGE_IV_6_TO_V_11,
    &EDGE_IV_6_TO_V_13,
    &EDGE_IV_6_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_6_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_7_TO_V,
    &EDGE_IV_7_TO_V_7,
    &EDGE_IV_7_TO_V_9,
    &EDGE_IV_7_TO_V_11,
    &EDGE_IV_7_TO_V_13,
    &EDGE_IV_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_9_TO_V,
    &EDGE_IV_9_TO_V_7,
    &EDGE_IV_9_TO_V_9,
    &EDGE_IV_9_TO_V_11,
    &EDGE_IV_9_TO_V_13,
    &EDGE_IV_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_MAJ7_TO_V,
    &EDGE_IV_MAJ7_TO_V_7,
    &EDGE_IV_MAJ7_TO_V_9,
    &EDGE_IV_MAJ7_TO_V_11,
    &EDGE_IV_MAJ7_TO_V_13,
    &EDGE_IV_MAJ7_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_MAJ7_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV__SHARP_11_TO_V,
    &EDGE_IV__SHARP_11_TO_V_7,
    &EDGE_IV__SHARP_11_TO_V_9,
    &EDGE_IV__SHARP_11_TO_V_11,
    &EDGE_IV__SHARP_11_TO_V_13,
    &EDGE_IV__SHARP_11_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV__SHARP_11_TO_V_7_PLUS__SHARP_9,
    &EDGE_V_TO_I,
    &EDGE_V_TO_I_6,
    &EDGE_V_TO_I_7,
    &EDGE_V_TO_I_9,
    &EDGE_V_TO_I_MAJ7,
    &EDGE_V_TO_I_MAJ9,
    &EDGE_V_7_TO_I,
    &EDGE_V_7_TO_I_6,
    &EDGE_V_7_TO_I_7,
    &EDGE_V_7_TO_I_9,
    &EDGE_V_7_TO_I_MAJ7,
    &EDGE_V_7_TO_I_MAJ9,
    &EDGE_V_9_TO_I,
    &EDGE_V_9_TO_I_6,
    &EDGE_V_9_TO_I_7,
    &EDGE_V_9_TO_I_9,
    &EDGE_V_9_TO_I_MAJ7,
    &EDGE_V_9_TO_I_MAJ9,
    &EDGE_V_11_TO_I,
    &EDGE_V_11_TO_I_6,
    &EDGE_V_11_TO_I_7,
    &EDGE_V_11_TO_I_9,
    &EDGE_V_11_TO_I_MAJ7,
    &EDGE_V_11_TO_I_MAJ9,
    &EDGE_V_13_TO_I,
    &EDGE_V_13_TO_I_6,
    &EDGE_V_13_TO_I_7,
    &EDGE_V_13_TO_I_9,
    &EDGE_V_13_TO_I_MAJ7,
    &EDGE_V_13_TO_I_MAJ9,
    &EDGE_V_7_PLUS__FLAT_9_TO_I,
    &EDGE_V_7_PLUS__FLAT_9_TO_I_6,
    &EDGE_V_7_PLUS__FLAT_9_TO_I_7,
    &EDGE_V_7_PLUS__FLAT_9_TO_I_9,
    &EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ7,
    &EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ9,
    &EDGE_V_7_PLUS__SHARP_9_TO_I,
    &EDGE_V_7_PLUS__SHARP_9_TO_I_6,
    &EDGE_V_7_PLUS__SHARP_9_TO_I_7,
    &EDGE_V_7_PLUS__SHARP_9_TO_I_9,
    &EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ7,
    &EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ9,
    &EDGE_V_TO_MINOR_VI,
    &EDGE_V_TO_MINOR_VI_7,
    &EDGE_V_TO_MINOR_VI_9,
    &EDGE_V_TO_MINOR_VI_M7,
    &EDGE_V_7_TO_MINOR_VI,
    &EDGE_V_7_TO_MINOR_VI_7,
    &EDGE_V_7_TO_MINOR_VI_9,
    &EDGE_V_7_TO_MINOR_VI_M7,
    &EDGE_V_9_TO_MINOR_VI,
    &EDGE_V_9_TO_MINOR_VI_7,
    &EDGE_V_9_TO_MINOR_VI_9,
    &EDGE_V_9_TO_MINOR_VI_M7,
    &EDGE_V_11_TO_MINOR_VI,
    &EDGE_V_11_TO_MINOR_VI_7,
    &EDGE_V_11_TO_MINOR_VI_9,
    &EDGE_V_11_TO_MINOR_VI_M7,
    &EDGE_V_13_TO_MINOR_VI,
    &EDGE_V_13_TO_MINOR_VI_7,
    &EDGE_V_13_TO_MINOR_VI_9,
    &EDGE_V_13_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_M7,
    &EDGE_MINOR_VI_TO_MINOR_II,
    &EDGE_MINOR_VI_TO_MINOR_II_7,
    &EDGE_MINOR_VI_TO_MINOR_II_9,
    &EDGE_MINOR_VI_TO_MINOR_II_11,
    &EDGE_MINOR_VI_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_7_TO_MINOR_II,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7,
    &EDGE_MINOR_VI_9_TO_MINOR_II_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II_11,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_TO_IV,
    &EDGE_MINOR_VI_TO_IV_6,
    &EDGE_MINOR_VI_TO_IV_7,
    &EDGE_MINOR_VI_TO_IV_9,
    &EDGE_MINOR_VI_TO_IV_MAJ7,
    &EDGE_MINOR_VI_TO_IV__SHARP_11,
    &EDGE_MINOR_VI_7_TO_IV,
    &EDGE_MINOR_VI_7_TO_IV_6,
    &EDGE_MINOR_VI_7_TO_IV_7,
    &EDGE_MINOR_VI_7_TO_IV_9,
    &EDGE_MINOR_VI_7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_7_TO_IV__SHARP_11,
    &EDGE_MINOR_VI_9_TO_IV,
    &EDGE_MINOR_VI_9_TO_IV_6,
    &EDGE_MINOR_VI_9_TO_IV_7,
    &EDGE_MINOR_VI_9_TO_IV_9,
    &EDGE_MINOR_VI_9_TO_IV_MAJ7,
    &EDGE_MINOR_VI_9_TO_IV__SHARP_11,
    &EDGE_MINOR_VI_M7_TO_IV,
    &EDGE_MINOR_VI_M7_TO_IV_6,
    &EDGE_MINOR_VI_M7_TO_IV_7,
    &EDGE_MINOR_VI_M7_TO_IV_9,
    &EDGE_MINOR_VI_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_IV__SHARP_11,
    &EDGE_MINOR_VII_TO_I,
    &EDGE_MINOR_VII_TO_I_6,
    &EDGE_MINOR_VII_TO_I_7,
    &EDGE_MINOR_VII_TO_I_9,
    &EDGE_MINOR_VII_TO_I_MAJ7,
    &EDGE_MINOR_VII_TO_I_MAJ9,
    &EDGE_MINOR_VII__FLAT_5_TO_I,
    &EDGE_MINOR_VII__FLAT_5_TO_I_6,
    &EDGE_MINOR_VII__FLAT_5_TO_I_7,
    &EDGE_MINOR_VII__FLAT_5_TO_I_9,
    &EDGE_MINOR_VII__FLAT_5_TO_I_MAJ7,
    &EDGE_MINOR_VII__FLAT_5_TO_I_MAJ9,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_6,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_7,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_9,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ7,
    &EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_III_7_TO_IV,
    &EDGE_MINOR_FLAT_III_7_TO_IV_6,
    &EDGE_MINOR_FLAT_III_7_TO_IV_7,
    &EDGE_MINOR_FLAT_III_7_TO_IV_9,
    &EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_FLAT_III_7_TO_IV__SHARP_11,
    &EDGE_MINOR_FLAT_III_9_TO_IV,
    &EDGE_MINOR_FLAT_III_9_TO_IV_6,
    &EDGE_MINOR_FLAT_III_9_TO_IV_7,
    &EDGE_MINOR_FLAT_III_9_TO_IV_9,
    &EDGE_MINOR_FLAT_III_9_TO_IV_MAJ7,
    &EDGE_MINOR_FLAT_III_9_TO_IV__SHARP_11,
    &EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_7,
    &EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_7,
    &EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_9,
    &EDGE_MINOR_FLAT_VII_7_TO_I,
    &EDGE_MINOR_FLAT_VII_7_TO_I_6,
    &EDGE_MINOR_FLAT_VII_7_TO_I_7,
    &EDGE_MINOR_FLAT_VII_7_TO_I_9,
    &EDGE_MINOR_FLAT_VII_7_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_VII_7_TO_I_MAJ9,
    &EDGE_MINOR_FLAT_VII_9_TO_I,
    &EDGE_MINOR_FLAT_VII_9_TO_I_6,
    &EDGE_MINOR_FLAT_VII_9_TO_I_7,
    &EDGE_MINOR_FLAT_VII_9_TO_I_9,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7,
    &EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9,
];

/// Look up a progression node by its display name for major keys
/// 
/// Returns the corresponding `ProgressionNode` for chord symbols like "I", "V7", "ii9", etc.
/// Supports 40 different chord variants.
pub fn get_node(name: &str) -> Option<&'static ProgressionNode> {
    match name {
        "I" => Some(&I),
        "I6" => Some(&I_6),
        "I7" => Some(&I_7),
        "I9" => Some(&I_9),
        "Imaj7" => Some(&I_MAJ7),
        "Imaj9" => Some(&I_MAJ9),
        "ii" => Some(&MINOR_II),
        "ii7" => Some(&MINOR_II_7),
        "ii9" => Some(&MINOR_II_9),
        "ii11" => Some(&MINOR_II_11),
        "ii7+b9" => Some(&MINOR_II_7_PLUS__FLAT_9),
        "iii" => Some(&MINOR_III),
        "iii7" => Some(&MINOR_III_7),
        "iiim7" => Some(&MINOR_III_M7),
        "IV" => Some(&IV),
        "IV6" => Some(&IV_6),
        "IV7" => Some(&IV_7),
        "IV9" => Some(&IV_9),
        "IVmaj7" => Some(&IV_MAJ7),
        "IV#11" => Some(&IV__SHARP_11),
        "V" => Some(&V),
        "V7" => Some(&V_7),
        "V9" => Some(&V_9),
        "V11" => Some(&V_11),
        "V13" => Some(&V_13),
        "V7+b9" => Some(&V_7_PLUS__FLAT_9),
        "V7+#9" => Some(&V_7_PLUS__SHARP_9),
        "vi" => Some(&MINOR_VI),
        "vi7" => Some(&MINOR_VI_7),
        "vi9" => Some(&MINOR_VI_9),
        "vim7" => Some(&MINOR_VI_M7),
        "vii" => Some(&MINOR_VII),
        "viib5" => Some(&MINOR_VII__FLAT_5),
        "viim7b5" => Some(&MINOR_VII_M7_FLAT_5),
        "bIII7" => Some(&MINOR_FLAT_III_7),
        "bIII9" => Some(&MINOR_FLAT_III_9),
        "bVI7" => Some(&MINOR_FLAT_VI_7),
        "bVI9" => Some(&MINOR_FLAT_VI_9),
        "bVII7" => Some(&MINOR_FLAT_VII_7),
        "bVII9" => Some(&MINOR_FLAT_VII_9),
        _ => None,
    }
}
