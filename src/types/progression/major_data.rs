//! Generated progression data for major keys from major.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionNode, ProgressionEdge, NodeType};
use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval};

static I_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];

pub static I: ProgressionNode = ProgressionNode {
    id: "I",
    display_name: "I",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_INTERVALS,
    base_function: "I",
};

static I_6_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH];

pub static I_6: ProgressionNode = ProgressionNode {
    id: "I6",
    display_name: "I6",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_6_INTERVALS,
    base_function: "I",
};

static I_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static I_7: ProgressionNode = ProgressionNode {
    id: "I7",
    display_name: "I7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_7_INTERVALS,
    base_function: "I",
};

static I_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static I_9: ProgressionNode = ProgressionNode {
    id: "I9",
    display_name: "I9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_9_INTERVALS,
    base_function: "I",
};

static I_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

pub static I_MAJ7: ProgressionNode = ProgressionNode {
    id: "Imaj7",
    display_name: "Imaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_MAJ7_INTERVALS,
    base_function: "I",
};

static I_MAJ9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH, Interval::MAJOR_NINTH];

pub static I_MAJ9: ProgressionNode = ProgressionNode {
    id: "Imaj9",
    display_name: "Imaj9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &I_MAJ9_INTERVALS,
    base_function: "I",
};

static MINOR_II_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_II_7: ProgressionNode = ProgressionNode {
    id: "ii7",
    display_name: "ii7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_7_INTERVALS,
    base_function: "ii",
};

static MINOR_II_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_II_9: ProgressionNode = ProgressionNode {
    id: "ii9",
    display_name: "ii9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_9_INTERVALS,
    base_function: "ii",
};

static MINOR_II_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH];

pub static MINOR_II_11: ProgressionNode = ProgressionNode {
    id: "ii11",
    display_name: "ii11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_11_INTERVALS,
    base_function: "ii",
};

static MINOR_II_7_PLUS__FLAT_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MINOR_NINTH];

pub static MINOR_II_7_PLUS__FLAT_9: ProgressionNode = ProgressionNode {
    id: "ii7+b9",
    display_name: "ii7+b9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_7_PLUS__FLAT_9_INTERVALS,
    base_function: "ii",
};

static MINOR_III_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_III_7: ProgressionNode = ProgressionNode {
    id: "iii7",
    display_name: "iii7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_III_7_INTERVALS,
    base_function: "iii",
};

static MINOR_III_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_III_M7: ProgressionNode = ProgressionNode {
    id: "iiim7",
    display_name: "iiim7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_III_M7_INTERVALS,
    base_function: "iii",
};

static IV_6_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SIXTH];

pub static IV_6: ProgressionNode = ProgressionNode {
    id: "IV6",
    display_name: "IV6",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_6_INTERVALS,
    base_function: "IV",
};

static IV_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static IV_7: ProgressionNode = ProgressionNode {
    id: "IV7",
    display_name: "IV7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_7_INTERVALS,
    base_function: "IV",
};

static IV_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static IV_9: ProgressionNode = ProgressionNode {
    id: "IV9",
    display_name: "IV9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_9_INTERVALS,
    base_function: "IV",
};

static IV_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

pub static IV_MAJ7: ProgressionNode = ProgressionNode {
    id: "IVmaj7",
    display_name: "IVmaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV_MAJ7_INTERVALS,
    base_function: "IV",
};

static IV__SHARP_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH, Interval::MAJOR_NINTH, Interval::AUGMENTED_ELEVENTH];

pub static IV__SHARP_11: ProgressionNode = ProgressionNode {
    id: "IV#11",
    display_name: "IV#11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &IV__SHARP_11_INTERVALS,
    base_function: "IV",
};

static V_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static V_7: ProgressionNode = ProgressionNode {
    id: "V7",
    display_name: "V7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_INTERVALS,
    base_function: "V",
};

static V_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static V_9: ProgressionNode = ProgressionNode {
    id: "V9",
    display_name: "V9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_9_INTERVALS,
    base_function: "V",
};

static V_11_INTERVALS: [Interval; 6] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH];

pub static V_11: ProgressionNode = ProgressionNode {
    id: "V11",
    display_name: "V11",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_11_INTERVALS,
    base_function: "V",
};

static V_13_INTERVALS: [Interval; 7] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH, Interval::PERFECT_ELEVENTH, Interval::MAJOR_THIRTEENTH];

pub static V_13: ProgressionNode = ProgressionNode {
    id: "V13",
    display_name: "V13",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_13_INTERVALS,
    base_function: "V",
};

static V_7_PLUS__FLAT_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MINOR_NINTH];

pub static V_7_PLUS__FLAT_9: ProgressionNode = ProgressionNode {
    id: "V7+b9",
    display_name: "V7+b9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_PLUS__FLAT_9_INTERVALS,
    base_function: "V",
};

static V_7_PLUS__SHARP_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::AUGMENTED_NINTH];

pub static V_7_PLUS__SHARP_9: ProgressionNode = ProgressionNode {
    id: "V7+#9",
    display_name: "V7+#9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_PLUS__SHARP_9_INTERVALS,
    base_function: "V",
};

static MINOR_VI_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_VI_7: ProgressionNode = ProgressionNode {
    id: "vi7",
    display_name: "vi7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_7_INTERVALS,
    base_function: "vi",
};

static MINOR_VI_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_VI_9: ProgressionNode = ProgressionNode {
    id: "vi9",
    display_name: "vi9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_9_INTERVALS,
    base_function: "vi",
};

static MINOR_VI_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_VI_M7: ProgressionNode = ProgressionNode {
    id: "vim7",
    display_name: "vim7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_VI_M7_INTERVALS,
    base_function: "vi",
};

static MINOR_VII__FLAT_5_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH];

pub static MINOR_VII__FLAT_5: ProgressionNode = ProgressionNode {
    id: "viib5",
    display_name: "viib5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_VII__FLAT_5_INTERVALS,
    base_function: "vii",
};

static MINOR_VII_M7_FLAT_5_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_VII_M7_FLAT_5: ProgressionNode = ProgressionNode {
    id: "viim7b5",
    display_name: "viim7b5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_VII_M7_FLAT_5_INTERVALS,
    base_function: "vii",
};

static MINOR_FLAT_III_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_FLAT_III_7: ProgressionNode = ProgressionNode {
    id: "bIII7",
    display_name: "bIII7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_FLAT_III_7_INTERVALS,
    base_function: "bIII",
};

static MINOR_FLAT_III_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_FLAT_III_9: ProgressionNode = ProgressionNode {
    id: "bIII9",
    display_name: "bIII9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &MINOR_FLAT_III_9_INTERVALS,
    base_function: "bIII",
};

static MINOR_FLAT_VI_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_FLAT_VI_7: ProgressionNode = ProgressionNode {
    id: "bVI7",
    display_name: "bVI7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_FLAT_VI_7_INTERVALS,
    base_function: "bVI",
};

static MINOR_FLAT_VI_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_FLAT_VI_9: ProgressionNode = ProgressionNode {
    id: "bVI9",
    display_name: "bVI9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &MINOR_FLAT_VI_9_INTERVALS,
    base_function: "bVI",
};

static MINOR_FLAT_VII_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_FLAT_VII_7: ProgressionNode = ProgressionNode {
    id: "bVII7",
    display_name: "bVII7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_FLAT_VII_7_INTERVALS,
    base_function: "bVII",
};

static MINOR_FLAT_VII_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_FLAT_VII_9: ProgressionNode = ProgressionNode {
    id: "bVII9",
    display_name: "bVII9",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &MINOR_FLAT_VII_9_INTERVALS,
    base_function: "bVII",
};

pub static EDGE_I_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_6,
};

pub static EDGE_I_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_7,
};

pub static EDGE_I_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_9,
};

pub static EDGE_I_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV_MAJ7,
};

pub static EDGE_I_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &IV__SHARP_11,
};

pub static EDGE_I_6_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_6,
};

pub static EDGE_I_6_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_7,
};

pub static EDGE_I_6_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_9,
};

pub static EDGE_I_6_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV_MAJ7,
};

pub static EDGE_I_6_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &IV__SHARP_11,
};

pub static EDGE_I_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_6,
};

pub static EDGE_I_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_7,
};

pub static EDGE_I_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_9,
};

pub static EDGE_I_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV_MAJ7,
};

pub static EDGE_I_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &IV__SHARP_11,
};

pub static EDGE_I_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_6,
};

pub static EDGE_I_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_7,
};

pub static EDGE_I_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_9,
};

pub static EDGE_I_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV_MAJ7,
};

pub static EDGE_I_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &IV__SHARP_11,
};

pub static EDGE_I_MAJ7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_6,
};

pub static EDGE_I_MAJ7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_7,
};

pub static EDGE_I_MAJ7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_9,
};

pub static EDGE_I_MAJ7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV_MAJ7,
};

pub static EDGE_I_MAJ7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &IV__SHARP_11,
};

pub static EDGE_I_MAJ9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_6,
};

pub static EDGE_I_MAJ9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_7,
};

pub static EDGE_I_MAJ9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_9,
};

pub static EDGE_I_MAJ9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV_MAJ7,
};

pub static EDGE_I_MAJ9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &IV__SHARP_11,
};

pub static EDGE_I_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7,
};

pub static EDGE_I_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_9,
};

pub static EDGE_I_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_11,
};

pub static EDGE_I_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_13,
};

pub static EDGE_I_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7,
};

pub static EDGE_I_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_9,
};

pub static EDGE_I_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_11,
};

pub static EDGE_I_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_13,
};

pub static EDGE_I_6_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_6_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7,
};

pub static EDGE_I_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_9,
};

pub static EDGE_I_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_11,
};

pub static EDGE_I_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_13,
};

pub static EDGE_I_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7,
};

pub static EDGE_I_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_9,
};

pub static EDGE_I_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_11,
};

pub static EDGE_I_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_13,
};

pub static EDGE_I_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7,
};

pub static EDGE_I_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_9,
};

pub static EDGE_I_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_11,
};

pub static EDGE_I_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_13,
};

pub static EDGE_I_MAJ7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_MAJ7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_MAJ9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7,
};

pub static EDGE_I_MAJ9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_9,
};

pub static EDGE_I_MAJ9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_11,
};

pub static EDGE_I_MAJ9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_13,
};

pub static EDGE_I_MAJ9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_I_MAJ9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_I_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_7,
};

pub static EDGE_I_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_9,
};

pub static EDGE_I_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I,
    to: &MINOR_VI_M7,
};

pub static EDGE_I_6_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_7,
};

pub static EDGE_I_6_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_9,
};

pub static EDGE_I_6_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_6,
    to: &MINOR_VI_M7,
};

pub static EDGE_I_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_7,
};

pub static EDGE_I_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_9,
};

pub static EDGE_I_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_7,
    to: &MINOR_VI_M7,
};

pub static EDGE_I_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_7,
};

pub static EDGE_I_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_9,
};

pub static EDGE_I_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_9,
    to: &MINOR_VI_M7,
};

pub static EDGE_I_MAJ7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_7,
};

pub static EDGE_I_MAJ7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_9,
};

pub static EDGE_I_MAJ7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ7,
    to: &MINOR_VI_M7,
};

pub static EDGE_I_MAJ9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_7,
};

pub static EDGE_I_MAJ9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_9,
};

pub static EDGE_I_MAJ9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &I_MAJ9,
    to: &MINOR_VI_M7,
};

pub static EDGE_MINOR_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7,
};

pub static EDGE_MINOR_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_9,
};

pub static EDGE_MINOR_II_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_11,
};

pub static EDGE_MINOR_II_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_13,
};

pub static EDGE_MINOR_II_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_MINOR_II_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7,
};

pub static EDGE_MINOR_II_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_9,
};

pub static EDGE_MINOR_II_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_11,
};

pub static EDGE_MINOR_II_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_13,
};

pub static EDGE_MINOR_II_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_MINOR_II_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7,
};

pub static EDGE_MINOR_II_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_9,
};

pub static EDGE_MINOR_II_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_11,
};

pub static EDGE_MINOR_II_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_13,
};

pub static EDGE_MINOR_II_11_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_11_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_9,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_11,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_13,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_MINOR_II_7_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &MINOR_VII__FLAT_5,
};

pub static EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7,
    to: &MINOR_VII_M7_FLAT_5,
};

pub static EDGE_MINOR_II_9_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &MINOR_VII__FLAT_5,
};

pub static EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_9,
    to: &MINOR_VII_M7_FLAT_5,
};

pub static EDGE_MINOR_II_11_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &MINOR_VII__FLAT_5,
};

pub static EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_11,
    to: &MINOR_VII_M7_FLAT_5,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &MINOR_VII__FLAT_5,
};

pub static EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_7_PLUS__FLAT_9,
    to: &MINOR_VII_M7_FLAT_5,
};

pub static EDGE_MINOR_III_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_7,
};

pub static EDGE_MINOR_III_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_9,
};

pub static EDGE_MINOR_III_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &MINOR_VI_M7,
};

pub static EDGE_MINOR_III_M7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_7,
};

pub static EDGE_MINOR_III_M7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_9,
};

pub static EDGE_MINOR_III_M7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &MINOR_VI_M7,
};

pub static EDGE_MINOR_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_6,
};

pub static EDGE_MINOR_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_7,
};

pub static EDGE_MINOR_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_9,
};

pub static EDGE_MINOR_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_III_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_7,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_III_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_6,
};

pub static EDGE_MINOR_III_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_7,
};

pub static EDGE_MINOR_III_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_9,
};

pub static EDGE_MINOR_III_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_III_M7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_III_M7,
    to: &IV__SHARP_11,
};

pub static EDGE_IV_6_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I,
};

pub static EDGE_IV_6_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_6,
};

pub static EDGE_IV_6_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_7,
};

pub static EDGE_IV_6_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_9,
};

pub static EDGE_IV_6_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_MAJ7,
};

pub static EDGE_IV_6_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &I_MAJ9,
};

pub static EDGE_IV_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I,
};

pub static EDGE_IV_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_6,
};

pub static EDGE_IV_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_7,
};

pub static EDGE_IV_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_9,
};

pub static EDGE_IV_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_MAJ7,
};

pub static EDGE_IV_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &I_MAJ9,
};

pub static EDGE_IV_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I,
};

pub static EDGE_IV_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_6,
};

pub static EDGE_IV_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_7,
};

pub static EDGE_IV_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_9,
};

pub static EDGE_IV_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_MAJ7,
};

pub static EDGE_IV_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &I_MAJ9,
};

pub static EDGE_IV_MAJ7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I,
};

pub static EDGE_IV_MAJ7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_6,
};

pub static EDGE_IV_MAJ7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_7,
};

pub static EDGE_IV_MAJ7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_9,
};

pub static EDGE_IV_MAJ7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_MAJ7,
};

pub static EDGE_IV_MAJ7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &I_MAJ9,
};

pub static EDGE_IV__SHARP_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I,
};

pub static EDGE_IV__SHARP_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_6,
};

pub static EDGE_IV__SHARP_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_7,
};

pub static EDGE_IV__SHARP_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_9,
};

pub static EDGE_IV__SHARP_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_MAJ7,
};

pub static EDGE_IV__SHARP_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &I_MAJ9,
};

pub static EDGE_IV_6_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_7,
};

pub static EDGE_IV_6_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_9,
};

pub static EDGE_IV_6_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_11,
};

pub static EDGE_IV_6_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_IV_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_7,
};

pub static EDGE_IV_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_9,
};

pub static EDGE_IV_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_11,
};

pub static EDGE_IV_7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_IV_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_7,
};

pub static EDGE_IV_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_9,
};

pub static EDGE_IV_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_11,
};

pub static EDGE_IV_9_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_IV_MAJ7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_7,
};

pub static EDGE_IV_MAJ7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_9,
};

pub static EDGE_IV_MAJ7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_11,
};

pub static EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_IV__SHARP_11_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_7,
};

pub static EDGE_IV__SHARP_11_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_9,
};

pub static EDGE_IV__SHARP_11_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_11,
};

pub static EDGE_IV__SHARP_11_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_IV_6_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7,
};

pub static EDGE_IV_6_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_9,
};

pub static EDGE_IV_6_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_11,
};

pub static EDGE_IV_6_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_13,
};

pub static EDGE_IV_6_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_IV_6_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_6,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_IV_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7,
};

pub static EDGE_IV_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_9,
};

pub static EDGE_IV_7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_11,
};

pub static EDGE_IV_7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_13,
};

pub static EDGE_IV_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_IV_7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_7,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_IV_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7,
};

pub static EDGE_IV_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_9,
};

pub static EDGE_IV_9_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_11,
};

pub static EDGE_IV_9_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_13,
};

pub static EDGE_IV_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_IV_9_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_9,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_IV_MAJ7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7,
};

pub static EDGE_IV_MAJ7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_9,
};

pub static EDGE_IV_MAJ7_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_11,
};

pub static EDGE_IV_MAJ7_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_13,
};

pub static EDGE_IV_MAJ7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_IV_MAJ7_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV_MAJ7,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_IV__SHARP_11_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7,
};

pub static EDGE_IV__SHARP_11_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_9,
};

pub static EDGE_IV__SHARP_11_TO_V_11: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_11,
};

pub static EDGE_IV__SHARP_11_TO_V_13: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_13,
};

pub static EDGE_IV__SHARP_11_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_IV__SHARP_11_TO_V_7_PLUS__SHARP_9: ProgressionEdge = ProgressionEdge {
    from: &IV__SHARP_11,
    to: &V_7_PLUS__SHARP_9,
};

pub static EDGE_V_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I,
};

pub static EDGE_V_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_6,
};

pub static EDGE_V_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_7,
};

pub static EDGE_V_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_9,
};

pub static EDGE_V_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_MAJ7,
};

pub static EDGE_V_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &I_MAJ9,
};

pub static EDGE_V_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I,
};

pub static EDGE_V_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_6,
};

pub static EDGE_V_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_7,
};

pub static EDGE_V_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_9,
};

pub static EDGE_V_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_MAJ7,
};

pub static EDGE_V_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &I_MAJ9,
};

pub static EDGE_V_11_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I,
};

pub static EDGE_V_11_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_6,
};

pub static EDGE_V_11_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_7,
};

pub static EDGE_V_11_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_9,
};

pub static EDGE_V_11_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_MAJ7,
};

pub static EDGE_V_11_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &I_MAJ9,
};

pub static EDGE_V_13_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I,
};

pub static EDGE_V_13_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_6,
};

pub static EDGE_V_13_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_7,
};

pub static EDGE_V_13_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_9,
};

pub static EDGE_V_13_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_MAJ7,
};

pub static EDGE_V_13_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &I_MAJ9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_6,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_MAJ7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &I_MAJ9,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_6,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_7,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_9,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_MAJ7,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &I_MAJ9,
};

pub static EDGE_V_7_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_7,
};

pub static EDGE_V_7_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_9,
};

pub static EDGE_V_7_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_VI_M7,
};

pub static EDGE_V_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_7,
};

pub static EDGE_V_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_9,
};

pub static EDGE_V_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_VI_M7,
};

pub static EDGE_V_11_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_7,
};

pub static EDGE_V_11_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_9,
};

pub static EDGE_V_11_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_11,
    to: &MINOR_VI_M7,
};

pub static EDGE_V_13_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_7,
};

pub static EDGE_V_13_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_9,
};

pub static EDGE_V_13_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_13,
    to: &MINOR_VI_M7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_VI_M7,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_7,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_9,
};

pub static EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__SHARP_9,
    to: &MINOR_VI_M7,
};

pub static EDGE_MINOR_VI_7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_7,
};

pub static EDGE_MINOR_VI_7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_9,
};

pub static EDGE_MINOR_VI_7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_11,
};

pub static EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_VI_9_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_7,
};

pub static EDGE_MINOR_VI_9_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_9,
};

pub static EDGE_MINOR_VI_9_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_11,
};

pub static EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_7,
};

pub static EDGE_MINOR_VI_M7_TO_MINOR_II_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_9,
};

pub static EDGE_MINOR_VI_M7_TO_MINOR_II_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_11,
};

pub static EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &MINOR_II_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_VI_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_6,
};

pub static EDGE_MINOR_VI_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_7,
};

pub static EDGE_MINOR_VI_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_9,
};

pub static EDGE_MINOR_VI_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_VI_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_7,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_VI_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_6,
};

pub static EDGE_MINOR_VI_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_7,
};

pub static EDGE_MINOR_VI_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_9,
};

pub static EDGE_MINOR_VI_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_VI_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_9,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_VI_M7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_6,
};

pub static EDGE_MINOR_VI_M7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_7,
};

pub static EDGE_MINOR_VI_M7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_9,
};

pub static EDGE_MINOR_VI_M7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_VI_M7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VI_M7,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_6,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_7,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_9,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_MAJ7,
};

pub static EDGE_MINOR_VII__FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII__FLAT_5,
    to: &I_MAJ9,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_6,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_7,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_9,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_MAJ7,
};

pub static EDGE_MINOR_VII_M7_FLAT_5_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_VII_M7_FLAT_5,
    to: &I_MAJ9,
};

pub static EDGE_MINOR_FLAT_III_7_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_6,
};

pub static EDGE_MINOR_FLAT_III_7_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_7,
};

pub static EDGE_MINOR_FLAT_III_7_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_9,
};

pub static EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_FLAT_III_7_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_7,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_FLAT_III_9_TO_IV_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_6,
};

pub static EDGE_MINOR_FLAT_III_9_TO_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_7,
};

pub static EDGE_MINOR_FLAT_III_9_TO_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_9,
};

pub static EDGE_MINOR_FLAT_III_9_TO_IV_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV_MAJ7,
};

pub static EDGE_MINOR_FLAT_III_9_TO_IV__SHARP_11: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_III_9,
    to: &IV__SHARP_11,
};

pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &MINOR_FLAT_VII_7,
};

pub static EDGE_MINOR_FLAT_VI_7_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &MINOR_FLAT_VII_9,
};

pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &MINOR_FLAT_VII_7,
};

pub static EDGE_MINOR_FLAT_VI_9_TO_MINOR_FLAT_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &MINOR_FLAT_VII_9,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_6,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_7,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_9,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_MAJ7,
};

pub static EDGE_MINOR_FLAT_VII_7_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_7,
    to: &I_MAJ9,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I_6: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_6,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_7,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_9,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_MAJ7,
};

pub static EDGE_MINOR_FLAT_VII_9_TO_I_MAJ9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VII_9,
    to: &I_MAJ9,
};

pub static ALL_NODES: &[&ProgressionNode] = &[
    &I,
    &I_6,
    &I_7,
    &I_9,
    &I_MAJ7,
    &I_MAJ9,
    &MINOR_II_7,
    &MINOR_II_9,
    &MINOR_II_11,
    &MINOR_II_7_PLUS__FLAT_9,
    &MINOR_III_7,
    &MINOR_III_M7,
    &IV_6,
    &IV_7,
    &IV_9,
    &IV_MAJ7,
    &IV__SHARP_11,
    &V_7,
    &V_9,
    &V_11,
    &V_13,
    &V_7_PLUS__FLAT_9,
    &V_7_PLUS__SHARP_9,
    &MINOR_VI_7,
    &MINOR_VI_9,
    &MINOR_VI_M7,
    &MINOR_VII__FLAT_5,
    &MINOR_VII_M7_FLAT_5,
    &MINOR_FLAT_III_7,
    &MINOR_FLAT_III_9,
    &MINOR_FLAT_VI_7,
    &MINOR_FLAT_VI_9,
    &MINOR_FLAT_VII_7,
    &MINOR_FLAT_VII_9,
];

pub static ALL_EDGES: &[&ProgressionEdge] = &[
    &EDGE_I_TO_IV_6,
    &EDGE_I_TO_IV_7,
    &EDGE_I_TO_IV_9,
    &EDGE_I_TO_IV_MAJ7,
    &EDGE_I_TO_IV__SHARP_11,
    &EDGE_I_6_TO_IV_6,
    &EDGE_I_6_TO_IV_7,
    &EDGE_I_6_TO_IV_9,
    &EDGE_I_6_TO_IV_MAJ7,
    &EDGE_I_6_TO_IV__SHARP_11,
    &EDGE_I_7_TO_IV_6,
    &EDGE_I_7_TO_IV_7,
    &EDGE_I_7_TO_IV_9,
    &EDGE_I_7_TO_IV_MAJ7,
    &EDGE_I_7_TO_IV__SHARP_11,
    &EDGE_I_9_TO_IV_6,
    &EDGE_I_9_TO_IV_7,
    &EDGE_I_9_TO_IV_9,
    &EDGE_I_9_TO_IV_MAJ7,
    &EDGE_I_9_TO_IV__SHARP_11,
    &EDGE_I_MAJ7_TO_IV_6,
    &EDGE_I_MAJ7_TO_IV_7,
    &EDGE_I_MAJ7_TO_IV_9,
    &EDGE_I_MAJ7_TO_IV_MAJ7,
    &EDGE_I_MAJ7_TO_IV__SHARP_11,
    &EDGE_I_MAJ9_TO_IV_6,
    &EDGE_I_MAJ9_TO_IV_7,
    &EDGE_I_MAJ9_TO_IV_9,
    &EDGE_I_MAJ9_TO_IV_MAJ7,
    &EDGE_I_MAJ9_TO_IV__SHARP_11,
    &EDGE_I_TO_V_7,
    &EDGE_I_TO_V_9,
    &EDGE_I_TO_V_11,
    &EDGE_I_TO_V_13,
    &EDGE_I_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_6_TO_V_7,
    &EDGE_I_6_TO_V_9,
    &EDGE_I_6_TO_V_11,
    &EDGE_I_6_TO_V_13,
    &EDGE_I_6_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_6_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_7_TO_V_7,
    &EDGE_I_7_TO_V_9,
    &EDGE_I_7_TO_V_11,
    &EDGE_I_7_TO_V_13,
    &EDGE_I_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_9_TO_V_7,
    &EDGE_I_9_TO_V_9,
    &EDGE_I_9_TO_V_11,
    &EDGE_I_9_TO_V_13,
    &EDGE_I_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_MAJ7_TO_V_7,
    &EDGE_I_MAJ7_TO_V_9,
    &EDGE_I_MAJ7_TO_V_11,
    &EDGE_I_MAJ7_TO_V_13,
    &EDGE_I_MAJ7_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_MAJ7_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_MAJ9_TO_V_7,
    &EDGE_I_MAJ9_TO_V_9,
    &EDGE_I_MAJ9_TO_V_11,
    &EDGE_I_MAJ9_TO_V_13,
    &EDGE_I_MAJ9_TO_V_7_PLUS__FLAT_9,
    &EDGE_I_MAJ9_TO_V_7_PLUS__SHARP_9,
    &EDGE_I_TO_MINOR_VI_7,
    &EDGE_I_TO_MINOR_VI_9,
    &EDGE_I_TO_MINOR_VI_M7,
    &EDGE_I_6_TO_MINOR_VI_7,
    &EDGE_I_6_TO_MINOR_VI_9,
    &EDGE_I_6_TO_MINOR_VI_M7,
    &EDGE_I_7_TO_MINOR_VI_7,
    &EDGE_I_7_TO_MINOR_VI_9,
    &EDGE_I_7_TO_MINOR_VI_M7,
    &EDGE_I_9_TO_MINOR_VI_7,
    &EDGE_I_9_TO_MINOR_VI_9,
    &EDGE_I_9_TO_MINOR_VI_M7,
    &EDGE_I_MAJ7_TO_MINOR_VI_7,
    &EDGE_I_MAJ7_TO_MINOR_VI_9,
    &EDGE_I_MAJ7_TO_MINOR_VI_M7,
    &EDGE_I_MAJ9_TO_MINOR_VI_7,
    &EDGE_I_MAJ9_TO_MINOR_VI_9,
    &EDGE_I_MAJ9_TO_MINOR_VI_M7,
    &EDGE_MINOR_II_7_TO_V_7,
    &EDGE_MINOR_II_7_TO_V_9,
    &EDGE_MINOR_II_7_TO_V_11,
    &EDGE_MINOR_II_7_TO_V_13,
    &EDGE_MINOR_II_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_9_TO_V_7,
    &EDGE_MINOR_II_9_TO_V_9,
    &EDGE_MINOR_II_9_TO_V_11,
    &EDGE_MINOR_II_9_TO_V_13,
    &EDGE_MINOR_II_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_11_TO_V_7,
    &EDGE_MINOR_II_11_TO_V_9,
    &EDGE_MINOR_II_11_TO_V_11,
    &EDGE_MINOR_II_11_TO_V_13,
    &EDGE_MINOR_II_11_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_11_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_11,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_13,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_MINOR_II_7_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_7_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_11_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII__FLAT_5,
    &EDGE_MINOR_II_7_PLUS__FLAT_9_TO_MINOR_VII_M7_FLAT_5,
    &EDGE_MINOR_III_7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_7,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_9,
    &EDGE_MINOR_III_M7_TO_MINOR_VI_M7,
    &EDGE_MINOR_III_7_TO_IV_6,
    &EDGE_MINOR_III_7_TO_IV_7,
    &EDGE_MINOR_III_7_TO_IV_9,
    &EDGE_MINOR_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_III_7_TO_IV__SHARP_11,
    &EDGE_MINOR_III_M7_TO_IV_6,
    &EDGE_MINOR_III_M7_TO_IV_7,
    &EDGE_MINOR_III_M7_TO_IV_9,
    &EDGE_MINOR_III_M7_TO_IV_MAJ7,
    &EDGE_MINOR_III_M7_TO_IV__SHARP_11,
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
    &EDGE_IV_6_TO_MINOR_II_7,
    &EDGE_IV_6_TO_MINOR_II_9,
    &EDGE_IV_6_TO_MINOR_II_11,
    &EDGE_IV_6_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_7_TO_MINOR_II_7,
    &EDGE_IV_7_TO_MINOR_II_9,
    &EDGE_IV_7_TO_MINOR_II_11,
    &EDGE_IV_7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_9_TO_MINOR_II_7,
    &EDGE_IV_9_TO_MINOR_II_9,
    &EDGE_IV_9_TO_MINOR_II_11,
    &EDGE_IV_9_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_MAJ7_TO_MINOR_II_7,
    &EDGE_IV_MAJ7_TO_MINOR_II_9,
    &EDGE_IV_MAJ7_TO_MINOR_II_11,
    &EDGE_IV_MAJ7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV__SHARP_11_TO_MINOR_II_7,
    &EDGE_IV__SHARP_11_TO_MINOR_II_9,
    &EDGE_IV__SHARP_11_TO_MINOR_II_11,
    &EDGE_IV__SHARP_11_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_IV_6_TO_V_7,
    &EDGE_IV_6_TO_V_9,
    &EDGE_IV_6_TO_V_11,
    &EDGE_IV_6_TO_V_13,
    &EDGE_IV_6_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_6_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_7_TO_V_7,
    &EDGE_IV_7_TO_V_9,
    &EDGE_IV_7_TO_V_11,
    &EDGE_IV_7_TO_V_13,
    &EDGE_IV_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_7_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_9_TO_V_7,
    &EDGE_IV_9_TO_V_9,
    &EDGE_IV_9_TO_V_11,
    &EDGE_IV_9_TO_V_13,
    &EDGE_IV_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_9_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV_MAJ7_TO_V_7,
    &EDGE_IV_MAJ7_TO_V_9,
    &EDGE_IV_MAJ7_TO_V_11,
    &EDGE_IV_MAJ7_TO_V_13,
    &EDGE_IV_MAJ7_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV_MAJ7_TO_V_7_PLUS__SHARP_9,
    &EDGE_IV__SHARP_11_TO_V_7,
    &EDGE_IV__SHARP_11_TO_V_9,
    &EDGE_IV__SHARP_11_TO_V_11,
    &EDGE_IV__SHARP_11_TO_V_13,
    &EDGE_IV__SHARP_11_TO_V_7_PLUS__FLAT_9,
    &EDGE_IV__SHARP_11_TO_V_7_PLUS__SHARP_9,
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
    &EDGE_V_7_TO_MINOR_VI_7,
    &EDGE_V_7_TO_MINOR_VI_9,
    &EDGE_V_7_TO_MINOR_VI_M7,
    &EDGE_V_9_TO_MINOR_VI_7,
    &EDGE_V_9_TO_MINOR_VI_9,
    &EDGE_V_9_TO_MINOR_VI_M7,
    &EDGE_V_11_TO_MINOR_VI_7,
    &EDGE_V_11_TO_MINOR_VI_9,
    &EDGE_V_11_TO_MINOR_VI_M7,
    &EDGE_V_13_TO_MINOR_VI_7,
    &EDGE_V_13_TO_MINOR_VI_9,
    &EDGE_V_13_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_VI_M7,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_7,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_9,
    &EDGE_V_7_PLUS__SHARP_9_TO_MINOR_VI_M7,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7,
    &EDGE_MINOR_VI_9_TO_MINOR_II_9,
    &EDGE_MINOR_VI_9_TO_MINOR_II_11,
    &EDGE_MINOR_VI_9_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_9,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_11,
    &EDGE_MINOR_VI_M7_TO_MINOR_II_7_PLUS__FLAT_9,
    &EDGE_MINOR_VI_7_TO_IV_6,
    &EDGE_MINOR_VI_7_TO_IV_7,
    &EDGE_MINOR_VI_7_TO_IV_9,
    &EDGE_MINOR_VI_7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_7_TO_IV__SHARP_11,
    &EDGE_MINOR_VI_9_TO_IV_6,
    &EDGE_MINOR_VI_9_TO_IV_7,
    &EDGE_MINOR_VI_9_TO_IV_9,
    &EDGE_MINOR_VI_9_TO_IV_MAJ7,
    &EDGE_MINOR_VI_9_TO_IV__SHARP_11,
    &EDGE_MINOR_VI_M7_TO_IV_6,
    &EDGE_MINOR_VI_M7_TO_IV_7,
    &EDGE_MINOR_VI_M7_TO_IV_9,
    &EDGE_MINOR_VI_M7_TO_IV_MAJ7,
    &EDGE_MINOR_VI_M7_TO_IV__SHARP_11,
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
    &EDGE_MINOR_FLAT_III_7_TO_IV_6,
    &EDGE_MINOR_FLAT_III_7_TO_IV_7,
    &EDGE_MINOR_FLAT_III_7_TO_IV_9,
    &EDGE_MINOR_FLAT_III_7_TO_IV_MAJ7,
    &EDGE_MINOR_FLAT_III_7_TO_IV__SHARP_11,
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

/// Look up a progression node by its display name
pub fn get_node(name: &str) -> Option<&'static ProgressionNode> {
    match name {
        "I" => Some(&I),
        "I6" => Some(&I_6),
        "I7" => Some(&I_7),
        "I9" => Some(&I_9),
        "Imaj7" => Some(&I_MAJ7),
        "Imaj9" => Some(&I_MAJ9),
        "ii7" => Some(&MINOR_II_7),
        "ii9" => Some(&MINOR_II_9),
        "ii11" => Some(&MINOR_II_11),
        "ii7+b9" => Some(&MINOR_II_7_PLUS__FLAT_9),
        "iii7" => Some(&MINOR_III_7),
        "iiim7" => Some(&MINOR_III_M7),
        "IV6" => Some(&IV_6),
        "IV7" => Some(&IV_7),
        "IV9" => Some(&IV_9),
        "IVmaj7" => Some(&IV_MAJ7),
        "IV#11" => Some(&IV__SHARP_11),
        "V7" => Some(&V_7),
        "V9" => Some(&V_9),
        "V11" => Some(&V_11),
        "V13" => Some(&V_13),
        "V7+b9" => Some(&V_7_PLUS__FLAT_9),
        "V7+#9" => Some(&V_7_PLUS__SHARP_9),
        "vi7" => Some(&MINOR_VI_7),
        "vi9" => Some(&MINOR_VI_9),
        "vim7" => Some(&MINOR_VI_M7),
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
