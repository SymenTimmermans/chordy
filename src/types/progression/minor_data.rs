//! Generated progression data for minor keys from minor.progression
//! Do not edit manually.

use crate::types::progression::{ProgressionNode, ProgressionEdge, NodeType};
use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval};

static MINOR_I_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];

pub static MINOR_I: ProgressionNode = ProgressionNode {
    id: "i",
    display_name: "i",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MINOR_I_INTERVALS,
    base_function: "i",
};

static MINOR_I_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_I_7: ProgressionNode = ProgressionNode {
    id: "i7",
    display_name: "i7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MINOR_I_7_INTERVALS,
    base_function: "i",
};

static MINOR_I_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_I_9: ProgressionNode = ProgressionNode {
    id: "i9",
    display_name: "i9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MINOR_I_9_INTERVALS,
    base_function: "i",
};

static MINOR_I_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_I_M7: ProgressionNode = ProgressionNode {
    id: "im7",
    display_name: "im7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MINOR_I_M7_INTERVALS,
    base_function: "i",
};

static MINOR_I_M9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_I_M9: ProgressionNode = ProgressionNode {
    id: "im9",
    display_name: "im9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::I, Accidental::Natural),
    intervals: &MINOR_I_M9_INTERVALS,
    base_function: "i",
};

static MINOR_II_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];

pub static MINOR_II: ProgressionNode = ProgressionNode {
    id: "ii",
    display_name: "ii",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_INTERVALS,
    base_function: "ii",
};

static MINOR_II__FLAT_5_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH];

pub static MINOR_II__FLAT_5: ProgressionNode = ProgressionNode {
    id: "iib5",
    display_name: "iib5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II__FLAT_5_INTERVALS,
    base_function: "ii",
};

static MINOR_II_M7_FLAT_5_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_II_M7_FLAT_5: ProgressionNode = ProgressionNode {
    id: "iim7b5",
    display_name: "iim7b5",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II_M7_FLAT_5_INTERVALS,
    base_function: "ii",
};

static MINOR_II__FLAT_5_PLUS_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::DIMINISHED_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_II__FLAT_5_PLUS_7: ProgressionNode = ProgressionNode {
    id: "iib5+7",
    display_name: "iib5+7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_II__FLAT_5_PLUS_7_INTERVALS,
    base_function: "ii",
};

static III_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];

pub static III: ProgressionNode = ProgressionNode {
    id: "III",
    display_name: "III",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &III_INTERVALS,
    base_function: "III",
};

static III_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static III_7: ProgressionNode = ProgressionNode {
    id: "III7",
    display_name: "III7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &III_7_INTERVALS,
    base_function: "III",
};

static III_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static III_9: ProgressionNode = ProgressionNode {
    id: "III9",
    display_name: "III9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &III_9_INTERVALS,
    base_function: "III",
};

static III_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

pub static III_MAJ7: ProgressionNode = ProgressionNode {
    id: "IIImaj7",
    display_name: "IIImaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::III, Accidental::Natural),
    intervals: &III_MAJ7_INTERVALS,
    base_function: "III",
};

static MINOR_IV_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];

pub static MINOR_IV: ProgressionNode = ProgressionNode {
    id: "iv",
    display_name: "iv",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &MINOR_IV_INTERVALS,
    base_function: "iv",
};

static MINOR_IV_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_IV_7: ProgressionNode = ProgressionNode {
    id: "iv7",
    display_name: "iv7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &MINOR_IV_7_INTERVALS,
    base_function: "iv",
};

static MINOR_IV_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static MINOR_IV_9: ProgressionNode = ProgressionNode {
    id: "iv9",
    display_name: "iv9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &MINOR_IV_9_INTERVALS,
    base_function: "iv",
};

static MINOR_IV_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_IV_M7: ProgressionNode = ProgressionNode {
    id: "ivm7",
    display_name: "ivm7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::IV, Accidental::Natural),
    intervals: &MINOR_IV_M7_INTERVALS,
    base_function: "iv",
};

static MINOR_V_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];

pub static MINOR_V: ProgressionNode = ProgressionNode {
    id: "v",
    display_name: "v",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &MINOR_V_INTERVALS,
    base_function: "v",
};

static MINOR_V_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_V_7: ProgressionNode = ProgressionNode {
    id: "v7",
    display_name: "v7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &MINOR_V_7_INTERVALS,
    base_function: "v",
};

static MINOR_V_M7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_V_M7: ProgressionNode = ProgressionNode {
    id: "vm7",
    display_name: "vm7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &MINOR_V_M7_INTERVALS,
    base_function: "v",
};

static V_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];

pub static V: ProgressionNode = ProgressionNode {
    id: "V",
    display_name: "V",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_INTERVALS,
    base_function: "V",
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

static V_7_PLUS__FLAT_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MINOR_NINTH];

pub static V_7_PLUS__FLAT_9: ProgressionNode = ProgressionNode {
    id: "V7+b9",
    display_name: "V7+b9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::V, Accidental::Natural),
    intervals: &V_7_PLUS__FLAT_9_INTERVALS,
    base_function: "V",
};

static VI_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];

pub static VI: ProgressionNode = ProgressionNode {
    id: "VI",
    display_name: "VI",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &VI_INTERVALS,
    base_function: "VI",
};

static VI_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static VI_7: ProgressionNode = ProgressionNode {
    id: "VI7",
    display_name: "VI7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &VI_7_INTERVALS,
    base_function: "VI",
};

static VI_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static VI_9: ProgressionNode = ProgressionNode {
    id: "VI9",
    display_name: "VI9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &VI_9_INTERVALS,
    base_function: "VI",
};

static VI_MAJ7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MAJOR_SEVENTH];

pub static VI_MAJ7: ProgressionNode = ProgressionNode {
    id: "VImaj7",
    display_name: "VImaj7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VI, Accidental::Natural),
    intervals: &VI_MAJ7_INTERVALS,
    base_function: "VI",
};

static VII_INTERVALS: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];

pub static VII: ProgressionNode = ProgressionNode {
    id: "VII",
    display_name: "VII",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &VII_INTERVALS,
    base_function: "VII",
};

static VII_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static VII_7: ProgressionNode = ProgressionNode {
    id: "VII7",
    display_name: "VII7",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &VII_7_INTERVALS,
    base_function: "VII",
};

static VII_9_INTERVALS: [Interval; 5] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH, Interval::MAJOR_NINTH];

pub static VII_9: ProgressionNode = ProgressionNode {
    id: "VII9",
    display_name: "VII9",
    node_type: NodeType::Primary,
    roman_numeral: RomanNumeral::new(RomanDegree::VII, Accidental::Natural),
    intervals: &VII_9_INTERVALS,
    base_function: "VII",
};

static MINOR_FLAT_II_7_INTERVALS: [Interval; 4] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH, Interval::MINOR_SEVENTH];

pub static MINOR_FLAT_II_7: ProgressionNode = ProgressionNode {
    id: "bII7",
    display_name: "bII7",
    node_type: NodeType::Secondary,
    roman_numeral: RomanNumeral::new(RomanDegree::II, Accidental::Natural),
    intervals: &MINOR_FLAT_II_7_INTERVALS,
    base_function: "bII",
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

pub static EDGE_MINOR_I_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &MINOR_IV,
};

pub static EDGE_MINOR_I_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &MINOR_IV_7,
};

pub static EDGE_MINOR_I_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &MINOR_IV_9,
};

pub static EDGE_MINOR_I_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_I_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &MINOR_IV,
};

pub static EDGE_MINOR_I_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &MINOR_IV_7,
};

pub static EDGE_MINOR_I_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &MINOR_IV_9,
};

pub static EDGE_MINOR_I_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_I_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &MINOR_IV,
};

pub static EDGE_MINOR_I_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &MINOR_IV_7,
};

pub static EDGE_MINOR_I_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &MINOR_IV_9,
};

pub static EDGE_MINOR_I_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_I_M7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &MINOR_IV,
};

pub static EDGE_MINOR_I_M7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &MINOR_IV_7,
};

pub static EDGE_MINOR_I_M7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &MINOR_IV_9,
};

pub static EDGE_MINOR_I_M7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_I_M9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &MINOR_IV,
};

pub static EDGE_MINOR_I_M9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &MINOR_IV_7,
};

pub static EDGE_MINOR_I_M9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &MINOR_IV_9,
};

pub static EDGE_MINOR_I_M9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_I_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &V,
};

pub static EDGE_MINOR_I_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &V_7,
};

pub static EDGE_MINOR_I_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &V_9,
};

pub static EDGE_MINOR_I_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_I_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &V,
};

pub static EDGE_MINOR_I_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &V_7,
};

pub static EDGE_MINOR_I_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &V_9,
};

pub static EDGE_MINOR_I_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_I_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &V,
};

pub static EDGE_MINOR_I_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &V_7,
};

pub static EDGE_MINOR_I_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &V_9,
};

pub static EDGE_MINOR_I_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_I_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &V,
};

pub static EDGE_MINOR_I_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &V_7,
};

pub static EDGE_MINOR_I_M7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &V_9,
};

pub static EDGE_MINOR_I_M7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_I_M9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &V,
};

pub static EDGE_MINOR_I_M9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &V_7,
};

pub static EDGE_MINOR_I_M9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &V_9,
};

pub static EDGE_MINOR_I_M9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_I_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &VI,
};

pub static EDGE_MINOR_I_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &VI_7,
};

pub static EDGE_MINOR_I_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &VI_9,
};

pub static EDGE_MINOR_I_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &VI_MAJ7,
};

pub static EDGE_MINOR_I_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &VI,
};

pub static EDGE_MINOR_I_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &VI_7,
};

pub static EDGE_MINOR_I_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &VI_9,
};

pub static EDGE_MINOR_I_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &VI_MAJ7,
};

pub static EDGE_MINOR_I_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &VI,
};

pub static EDGE_MINOR_I_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &VI_7,
};

pub static EDGE_MINOR_I_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &VI_9,
};

pub static EDGE_MINOR_I_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &VI_MAJ7,
};

pub static EDGE_MINOR_I_M7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &VI,
};

pub static EDGE_MINOR_I_M7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &VI_7,
};

pub static EDGE_MINOR_I_M7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &VI_9,
};

pub static EDGE_MINOR_I_M7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &VI_MAJ7,
};

pub static EDGE_MINOR_I_M9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &VI,
};

pub static EDGE_MINOR_I_M9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &VI_7,
};

pub static EDGE_MINOR_I_M9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &VI_9,
};

pub static EDGE_MINOR_I_M9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &VI_MAJ7,
};

pub static EDGE_MINOR_I_TO_III: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &III,
};

pub static EDGE_MINOR_I_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &III_7,
};

pub static EDGE_MINOR_I_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &III_9,
};

pub static EDGE_MINOR_I_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_I_7_TO_III: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &III,
};

pub static EDGE_MINOR_I_7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &III_7,
};

pub static EDGE_MINOR_I_7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &III_9,
};

pub static EDGE_MINOR_I_7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_7,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_I_9_TO_III: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &III,
};

pub static EDGE_MINOR_I_9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &III_7,
};

pub static EDGE_MINOR_I_9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &III_9,
};

pub static EDGE_MINOR_I_9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_9,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_I_M7_TO_III: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &III,
};

pub static EDGE_MINOR_I_M7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &III_7,
};

pub static EDGE_MINOR_I_M7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &III_9,
};

pub static EDGE_MINOR_I_M7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M7,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_I_M9_TO_III: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &III,
};

pub static EDGE_MINOR_I_M9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &III_7,
};

pub static EDGE_MINOR_I_M9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &III_9,
};

pub static EDGE_MINOR_I_M9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_I_M9,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_II_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V,
};

pub static EDGE_MINOR_II_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_7,
};

pub static EDGE_MINOR_II_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_9,
};

pub static EDGE_MINOR_II_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II__FLAT_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &V,
};

pub static EDGE_MINOR_II__FLAT_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &V_7,
};

pub static EDGE_MINOR_II__FLAT_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &V_9,
};

pub static EDGE_MINOR_II__FLAT_5_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &V,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &V_7,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &V_9,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &V,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &V_7,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &V_9,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_II_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &VII,
};

pub static EDGE_MINOR_II_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &VII_7,
};

pub static EDGE_MINOR_II_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II,
    to: &VII_9,
};

pub static EDGE_MINOR_II__FLAT_5_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &VII,
};

pub static EDGE_MINOR_II__FLAT_5_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &VII_7,
};

pub static EDGE_MINOR_II__FLAT_5_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5,
    to: &VII_9,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &VII,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &VII_7,
};

pub static EDGE_MINOR_II_M7_FLAT_5_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II_M7_FLAT_5,
    to: &VII_9,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &VII,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &VII_7,
};

pub static EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_II__FLAT_5_PLUS_7,
    to: &VII_9,
};

pub static EDGE_III_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &VI,
};

pub static EDGE_III_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &VI_7,
};

pub static EDGE_III_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &VI_9,
};

pub static EDGE_III_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &VI_MAJ7,
};

pub static EDGE_III_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &VI,
};

pub static EDGE_III_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &VI_7,
};

pub static EDGE_III_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &VI_9,
};

pub static EDGE_III_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &VI_MAJ7,
};

pub static EDGE_III_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &VI,
};

pub static EDGE_III_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &VI_7,
};

pub static EDGE_III_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &VI_9,
};

pub static EDGE_III_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &VI_MAJ7,
};

pub static EDGE_III_MAJ7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &VI,
};

pub static EDGE_III_MAJ7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &VI_7,
};

pub static EDGE_III_MAJ7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &VI_9,
};

pub static EDGE_III_MAJ7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &VI_MAJ7,
};

pub static EDGE_III_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &MINOR_IV,
};

pub static EDGE_III_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &MINOR_IV_7,
};

pub static EDGE_III_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &MINOR_IV_9,
};

pub static EDGE_III_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &III,
    to: &MINOR_IV_M7,
};

pub static EDGE_III_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &MINOR_IV,
};

pub static EDGE_III_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &MINOR_IV_7,
};

pub static EDGE_III_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &MINOR_IV_9,
};

pub static EDGE_III_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &III_7,
    to: &MINOR_IV_M7,
};

pub static EDGE_III_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &MINOR_IV,
};

pub static EDGE_III_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &MINOR_IV_7,
};

pub static EDGE_III_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &MINOR_IV_9,
};

pub static EDGE_III_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &III_9,
    to: &MINOR_IV_M7,
};

pub static EDGE_III_MAJ7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &MINOR_IV,
};

pub static EDGE_III_MAJ7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &MINOR_IV_7,
};

pub static EDGE_III_MAJ7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &MINOR_IV_9,
};

pub static EDGE_III_MAJ7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &III_MAJ7,
    to: &MINOR_IV_M7,
};

pub static EDGE_MINOR_IV_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_I,
};

pub static EDGE_MINOR_IV_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_IV_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_IV_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_IV_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_I,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_I,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_I,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_IV_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_II,
};

pub static EDGE_MINOR_IV_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_MINOR_IV_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_MINOR_IV_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_II,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_MINOR_IV_7_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_II,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_MINOR_IV_9_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_II,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_MINOR_IV_M7_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_MINOR_IV_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &V,
};

pub static EDGE_MINOR_IV_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &V_7,
};

pub static EDGE_MINOR_IV_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &V_9,
};

pub static EDGE_MINOR_IV_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_IV_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &V,
};

pub static EDGE_MINOR_IV_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &V_7,
};

pub static EDGE_MINOR_IV_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &V_9,
};

pub static EDGE_MINOR_IV_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_IV_9_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &V,
};

pub static EDGE_MINOR_IV_9_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &V_7,
};

pub static EDGE_MINOR_IV_9_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &V_9,
};

pub static EDGE_MINOR_IV_9_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_IV_M7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &V,
};

pub static EDGE_MINOR_IV_M7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &V_7,
};

pub static EDGE_MINOR_IV_M7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &V_9,
};

pub static EDGE_MINOR_IV_M7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_IV_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &VII,
};

pub static EDGE_MINOR_IV_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &VII_7,
};

pub static EDGE_MINOR_IV_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV,
    to: &VII_9,
};

pub static EDGE_MINOR_IV_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &VII,
};

pub static EDGE_MINOR_IV_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &VII_7,
};

pub static EDGE_MINOR_IV_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_7,
    to: &VII_9,
};

pub static EDGE_MINOR_IV_9_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &VII,
};

pub static EDGE_MINOR_IV_9_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &VII_7,
};

pub static EDGE_MINOR_IV_9_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_9,
    to: &VII_9,
};

pub static EDGE_MINOR_IV_M7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &VII,
};

pub static EDGE_MINOR_IV_M7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &VII_7,
};

pub static EDGE_MINOR_IV_M7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_IV_M7,
    to: &VII_9,
};

pub static EDGE_MINOR_V_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V,
    to: &MINOR_I,
};

pub static EDGE_MINOR_V_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_V_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_V_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_V_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_V_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_7,
    to: &MINOR_I,
};

pub static EDGE_MINOR_V_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_7,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_V_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_7,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_V_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_7,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_V_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_7,
    to: &MINOR_I_M9,
};

pub static EDGE_MINOR_V_M7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_M7,
    to: &MINOR_I,
};

pub static EDGE_MINOR_V_M7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_M7,
    to: &MINOR_I_7,
};

pub static EDGE_MINOR_V_M7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_M7,
    to: &MINOR_I_9,
};

pub static EDGE_MINOR_V_M7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_M7,
    to: &MINOR_I_M7,
};

pub static EDGE_MINOR_V_M7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_V_M7,
    to: &MINOR_I_M9,
};

pub static EDGE_V_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_I,
};

pub static EDGE_V_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_I_7,
};

pub static EDGE_V_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_I_9,
};

pub static EDGE_V_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_I_M7,
};

pub static EDGE_V_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &MINOR_I_M9,
};

pub static EDGE_V_7_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_I,
};

pub static EDGE_V_7_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_I_7,
};

pub static EDGE_V_7_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_I_9,
};

pub static EDGE_V_7_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_I_M7,
};

pub static EDGE_V_7_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &MINOR_I_M9,
};

pub static EDGE_V_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_I,
};

pub static EDGE_V_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_I_7,
};

pub static EDGE_V_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_I_9,
};

pub static EDGE_V_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_I_M7,
};

pub static EDGE_V_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &MINOR_I_M9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_I,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_I_7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_I_9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_M7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_I_M7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_M9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &MINOR_I_M9,
};

pub static EDGE_V_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &VI,
};

pub static EDGE_V_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &VI_7,
};

pub static EDGE_V_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &VI_9,
};

pub static EDGE_V_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V,
    to: &VI_MAJ7,
};

pub static EDGE_V_7_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &VI,
};

pub static EDGE_V_7_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &VI_7,
};

pub static EDGE_V_7_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &VI_9,
};

pub static EDGE_V_7_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7,
    to: &VI_MAJ7,
};

pub static EDGE_V_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &VI,
};

pub static EDGE_V_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &VI_7,
};

pub static EDGE_V_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &VI_9,
};

pub static EDGE_V_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_9,
    to: &VI_MAJ7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_VI: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &VI,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_VI_7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &VI_7,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_VI_9: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &VI_9,
};

pub static EDGE_V_7_PLUS__FLAT_9_TO_VI_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &V_7_PLUS__FLAT_9,
    to: &VI_MAJ7,
};

pub static EDGE_VI_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_II,
};

pub static EDGE_VI_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_VI_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_VI_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_VI_7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_II,
};

pub static EDGE_VI_7_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_VI_7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_VI_7_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_VI_9_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_II,
};

pub static EDGE_VI_9_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_VI_9_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_VI_9_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_VI_MAJ7_TO_MINOR_II: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_II,
};

pub static EDGE_VI_MAJ7_TO_MINOR_II__FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_II__FLAT_5,
};

pub static EDGE_VI_MAJ7_TO_MINOR_II_M7_FLAT_5: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_II_M7_FLAT_5,
};

pub static EDGE_VI_MAJ7_TO_MINOR_II__FLAT_5_PLUS_7: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_II__FLAT_5_PLUS_7,
};

pub static EDGE_VI_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_IV,
};

pub static EDGE_VI_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_IV_7,
};

pub static EDGE_VI_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_IV_9,
};

pub static EDGE_VI_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &VI,
    to: &MINOR_IV_M7,
};

pub static EDGE_VI_7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_IV,
};

pub static EDGE_VI_7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_IV_7,
};

pub static EDGE_VI_7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_IV_9,
};

pub static EDGE_VI_7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &VI_7,
    to: &MINOR_IV_M7,
};

pub static EDGE_VI_9_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_IV,
};

pub static EDGE_VI_9_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_IV_7,
};

pub static EDGE_VI_9_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_IV_9,
};

pub static EDGE_VI_9_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &VI_9,
    to: &MINOR_IV_M7,
};

pub static EDGE_VI_MAJ7_TO_MINOR_IV: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_IV,
};

pub static EDGE_VI_MAJ7_TO_MINOR_IV_7: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_IV_7,
};

pub static EDGE_VI_MAJ7_TO_MINOR_IV_9: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_IV_9,
};

pub static EDGE_VI_MAJ7_TO_MINOR_IV_M7: ProgressionEdge = ProgressionEdge {
    from: &VI_MAJ7,
    to: &MINOR_IV_M7,
};

pub static EDGE_VII_TO_III: ProgressionEdge = ProgressionEdge {
    from: &VII,
    to: &III,
};

pub static EDGE_VII_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &VII,
    to: &III_7,
};

pub static EDGE_VII_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &VII,
    to: &III_9,
};

pub static EDGE_VII_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &VII,
    to: &III_MAJ7,
};

pub static EDGE_VII_7_TO_III: ProgressionEdge = ProgressionEdge {
    from: &VII_7,
    to: &III,
};

pub static EDGE_VII_7_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &VII_7,
    to: &III_7,
};

pub static EDGE_VII_7_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &VII_7,
    to: &III_9,
};

pub static EDGE_VII_7_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &VII_7,
    to: &III_MAJ7,
};

pub static EDGE_VII_9_TO_III: ProgressionEdge = ProgressionEdge {
    from: &VII_9,
    to: &III,
};

pub static EDGE_VII_9_TO_III_7: ProgressionEdge = ProgressionEdge {
    from: &VII_9,
    to: &III_7,
};

pub static EDGE_VII_9_TO_III_9: ProgressionEdge = ProgressionEdge {
    from: &VII_9,
    to: &III_9,
};

pub static EDGE_VII_9_TO_III_MAJ7: ProgressionEdge = ProgressionEdge {
    from: &VII_9,
    to: &III_MAJ7,
};

pub static EDGE_MINOR_FLAT_II_7_TO_V: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_II_7,
    to: &V,
};

pub static EDGE_MINOR_FLAT_II_7_TO_V_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_II_7,
    to: &V_7,
};

pub static EDGE_MINOR_FLAT_II_7_TO_V_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_II_7,
    to: &V_9,
};

pub static EDGE_MINOR_FLAT_II_7_TO_V_7_PLUS__FLAT_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_II_7,
    to: &V_7_PLUS__FLAT_9,
};

pub static EDGE_MINOR_FLAT_VI_7_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &VII,
};

pub static EDGE_MINOR_FLAT_VI_7_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &VII_7,
};

pub static EDGE_MINOR_FLAT_VI_7_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_7,
    to: &VII_9,
};

pub static EDGE_MINOR_FLAT_VI_9_TO_VII: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &VII,
};

pub static EDGE_MINOR_FLAT_VI_9_TO_VII_7: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &VII_7,
};

pub static EDGE_MINOR_FLAT_VI_9_TO_VII_9: ProgressionEdge = ProgressionEdge {
    from: &MINOR_FLAT_VI_9,
    to: &VII_9,
};

pub static ALL_NODES: &[&ProgressionNode] = &[
    &MINOR_I,
    &MINOR_I_7,
    &MINOR_I_9,
    &MINOR_I_M7,
    &MINOR_I_M9,
    &MINOR_II,
    &MINOR_II__FLAT_5,
    &MINOR_II_M7_FLAT_5,
    &MINOR_II__FLAT_5_PLUS_7,
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
    &V_7_PLUS__FLAT_9,
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
    &EDGE_MINOR_I_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_I_7_TO_V,
    &EDGE_MINOR_I_7_TO_V_7,
    &EDGE_MINOR_I_7_TO_V_9,
    &EDGE_MINOR_I_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_I_9_TO_V,
    &EDGE_MINOR_I_9_TO_V_7,
    &EDGE_MINOR_I_9_TO_V_9,
    &EDGE_MINOR_I_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_I_M7_TO_V,
    &EDGE_MINOR_I_M7_TO_V_7,
    &EDGE_MINOR_I_M7_TO_V_9,
    &EDGE_MINOR_I_M7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_I_M9_TO_V,
    &EDGE_MINOR_I_M9_TO_V_7,
    &EDGE_MINOR_I_M9_TO_V_9,
    &EDGE_MINOR_I_M9_TO_V_7_PLUS__FLAT_9,
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
    &EDGE_MINOR_II_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II__FLAT_5_TO_V,
    &EDGE_MINOR_II__FLAT_5_TO_V_7,
    &EDGE_MINOR_II__FLAT_5_TO_V_9,
    &EDGE_MINOR_II__FLAT_5_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_7,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_7,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_9,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_II_TO_VII,
    &EDGE_MINOR_II_TO_VII_7,
    &EDGE_MINOR_II_TO_VII_9,
    &EDGE_MINOR_II__FLAT_5_TO_VII,
    &EDGE_MINOR_II__FLAT_5_TO_VII_7,
    &EDGE_MINOR_II__FLAT_5_TO_VII_9,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII_7,
    &EDGE_MINOR_II_M7_FLAT_5_TO_VII_9,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII_7,
    &EDGE_MINOR_II__FLAT_5_PLUS_7_TO_VII_9,
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
    &EDGE_MINOR_IV_TO_MINOR_II__FLAT_5,
    &EDGE_MINOR_IV_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_7_TO_MINOR_II,
    &EDGE_MINOR_IV_7_TO_MINOR_II__FLAT_5,
    &EDGE_MINOR_IV_7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_7_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_9_TO_MINOR_II,
    &EDGE_MINOR_IV_9_TO_MINOR_II__FLAT_5,
    &EDGE_MINOR_IV_9_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_9_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_M7_TO_MINOR_II,
    &EDGE_MINOR_IV_M7_TO_MINOR_II__FLAT_5,
    &EDGE_MINOR_IV_M7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_MINOR_IV_M7_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_MINOR_IV_TO_V,
    &EDGE_MINOR_IV_TO_V_7,
    &EDGE_MINOR_IV_TO_V_9,
    &EDGE_MINOR_IV_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_IV_7_TO_V,
    &EDGE_MINOR_IV_7_TO_V_7,
    &EDGE_MINOR_IV_7_TO_V_9,
    &EDGE_MINOR_IV_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_IV_9_TO_V,
    &EDGE_MINOR_IV_9_TO_V_7,
    &EDGE_MINOR_IV_9_TO_V_9,
    &EDGE_MINOR_IV_9_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_IV_M7_TO_V,
    &EDGE_MINOR_IV_M7_TO_V_7,
    &EDGE_MINOR_IV_M7_TO_V_9,
    &EDGE_MINOR_IV_M7_TO_V_7_PLUS__FLAT_9,
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
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_9,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_M7,
    &EDGE_V_7_PLUS__FLAT_9_TO_MINOR_I_M9,
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
    &EDGE_V_7_PLUS__FLAT_9_TO_VI,
    &EDGE_V_7_PLUS__FLAT_9_TO_VI_7,
    &EDGE_V_7_PLUS__FLAT_9_TO_VI_9,
    &EDGE_V_7_PLUS__FLAT_9_TO_VI_MAJ7,
    &EDGE_VI_TO_MINOR_II,
    &EDGE_VI_TO_MINOR_II__FLAT_5,
    &EDGE_VI_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_VI_7_TO_MINOR_II,
    &EDGE_VI_7_TO_MINOR_II__FLAT_5,
    &EDGE_VI_7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_7_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_VI_9_TO_MINOR_II,
    &EDGE_VI_9_TO_MINOR_II__FLAT_5,
    &EDGE_VI_9_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_9_TO_MINOR_II__FLAT_5_PLUS_7,
    &EDGE_VI_MAJ7_TO_MINOR_II,
    &EDGE_VI_MAJ7_TO_MINOR_II__FLAT_5,
    &EDGE_VI_MAJ7_TO_MINOR_II_M7_FLAT_5,
    &EDGE_VI_MAJ7_TO_MINOR_II__FLAT_5_PLUS_7,
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
    &EDGE_MINOR_FLAT_II_7_TO_V_7_PLUS__FLAT_9,
    &EDGE_MINOR_FLAT_VI_7_TO_VII,
    &EDGE_MINOR_FLAT_VI_7_TO_VII_7,
    &EDGE_MINOR_FLAT_VI_7_TO_VII_9,
    &EDGE_MINOR_FLAT_VI_9_TO_VII,
    &EDGE_MINOR_FLAT_VI_9_TO_VII_7,
    &EDGE_MINOR_FLAT_VI_9_TO_VII_9,
];

/// Look up a progression node by its display name
pub fn get_node(name: &str) -> Option<&'static ProgressionNode> {
    match name {
        "i" => Some(&MINOR_I),
        "i7" => Some(&MINOR_I_7),
        "i9" => Some(&MINOR_I_9),
        "im7" => Some(&MINOR_I_M7),
        "im9" => Some(&MINOR_I_M9),
        "ii" => Some(&MINOR_II),
        "iib5" => Some(&MINOR_II__FLAT_5),
        "iim7b5" => Some(&MINOR_II_M7_FLAT_5),
        "iib5+7" => Some(&MINOR_II__FLAT_5_PLUS_7),
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
        "V7+b9" => Some(&V_7_PLUS__FLAT_9),
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
