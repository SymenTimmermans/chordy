use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

/// Convert interval string to semitone count (e.g., m2 -> 1, M2 -> 2, etc.)
fn parse_interval_code(code: &str) -> u8 {
    match code {
        "P1" => 0,
        "m2" => 1,
        "M2" => 2,
        "A2" | "m3" => 3,
        "M3" | "d4" => 4,
        "P4" => 5,
        "A4" => 6,
        "d5" => 6,
        "P5" => 7,
        "m6" | "A5" => 8,
        "M6" | "d7" => 9,
        "m7" | "A6" => 10,
        "M7" => 11,
        _ => panic!("Unknown interval code: {}", code),
    }
}

/// Convert a list of interval strings to a bitmask
fn to_bitmask(intervals: &[&str]) -> u16 {
    intervals.iter().fold(0u16, |mask, &code| {
        let bit = parse_interval_code(code);
        mask | (1 << bit)
    })
}

/// Convert a list of interval strings to Interval const names
/// These should always be in line with the consts defined in types/interval.rs
fn to_interval_const_name(interval: &str) -> String {
    assert!(
        interval.len() == 2,
        "Interval code must be 2 characters long"
    );

    let quality = match interval.chars().next().unwrap() {
        'P' => "PERFECT",
        'm' => "MINOR",
        'M' => "MAJOR",
        'd' => "DIMINISHED",
        'A' => "AUGMENTED",
        _ => panic!("Invalid interval quality: {}", interval),
    };

    let size = match interval.chars().nth(1).unwrap() {
        '1' => "UNISON",
        '2' => "SECOND",
        '3' => "THIRD",
        '4' => "FOURTH",
        '5' => "FIFTH",
        '6' => "SIXTH",
        '7' => "SEVENTH",
        _ => panic!("Invalid interval size: {}", interval),
    };

    format!("Interval::{}_{}", quality, size)
}

fn main() {
    // Generate scales
    generate_scales();

    // Generate progressions
    generate_progressions();

    // Generate chord progression database
    generate_chord_progressions();

    // Generate guitar shapes
    generate_guitar_shapes();
}

fn generate_scales() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let csv_path = Path::new(&manifest_dir).join("data/scales.csv");

    let file = File::open(&csv_path).expect("Failed to open scales.csv");
    let reader = BufReader::new(file);

    let mut generated = String::new();

    generated.push_str("//! This file is generated via build.rs from the scales.csv file. Do not edit manually.\n\n");
    generated.push_str("use crate::{ScaleDefinition, Interval, ScaleBitmask};\n\n");

    let mut registry_entries = Vec::new();

    reader
        .lines()
        .map_while(Result::ok)
        .skip(1) // Skip header line
        .filter(|l| !l.trim().is_empty())
        .for_each(|line| {
            let parts: Vec<_> = line.split(';').collect();
            if parts.len() < 4 {
                panic!("Malformed line: {}", line);
            }

            let name = parts[0].trim();
            let interval_strs: Vec<&str> = parts[1].split(',').map(str::trim).collect();
            let intervals_formatted = interval_strs
                .iter()
                .map(|s| to_interval_const_name(s))
                .collect::<Vec<_>>()
                .join(", ");

            let parent = parts[2].trim();
            let offset = parts[3].trim();

            let bitmask = to_bitmask(&interval_strs);
            let const_name = name
                .to_uppercase()
                .replace(|c: char| !c.is_alphanumeric(), "_");

            let doc_name = format!(
                "{}{}: {}",
                name,
                if parent.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        " (mode {} of {})",
                        offset.parse::<i8>().unwrap() + 1,
                        parent
                    )
                },
                parts[1]
            );

            generated.push_str(&format!(
                "/// {}
pub const {}: ScaleDefinition = ScaleDefinition {{
    name: \"{}\",
    intervals: &[{}],
    bitmask: ScaleBitmask(0b{:012b}),
    mode_of: {},
    degree_offset: {},
}};\n\n",
                doc_name,
                const_name,
                name,
                intervals_formatted,
                bitmask,
                if parent.is_empty() {
                    "None".to_string()
                } else {
                    format!("Some(\"{}\")", parent)
                },
                if offset.is_empty() {
                    "None".to_string()
                } else {
                    format!("Some({})", offset)
                }
            ));

            registry_entries.push(const_name);
        });

    generated.push_str(
        "/// Registry of all scales.
pub const REGISTRY: &[ScaleDefinition] = &[\n",
    );
    for name in &registry_entries {
        generated.push_str(&format!("    {},\n", name));
    }
    generated.push_str("];\n");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_file = Path::new(&manifest_dir).join("src/types/scale/scales.rs");

    fs::write(&out_file, &generated).expect("Failed to write scales registry module.");
}

fn generate_progressions() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Generate major progressions
    let major_path = Path::new(&manifest_dir).join("data/progressions/major.progression");
    let major_data = parse_progression_file(&major_path, "major");
    let major_output = Path::new(&manifest_dir).join("src/types/progression/major_data.rs");
    fs::write(&major_output, &major_data).expect("Failed to write major progression data");

    // Generate minor progressions
    let minor_path = Path::new(&manifest_dir).join("data/progressions/minor.progression");
    let minor_data = parse_progression_file(&minor_path, "minor");
    let minor_output = Path::new(&manifest_dir).join("src/types/progression/minor_data.rs");
    fs::write(&minor_output, &minor_data).expect("Failed to write minor progression data");
}

#[derive(Debug)]
struct ProgressionNode {
    id: String,
    node_type: String,
    variants: Vec<String>,
}

#[derive(Debug)]
struct ProgressionEdge {
    from: String,
    to: String,
}

fn parse_progression_file(path: &Path, key_type: &str) -> String {
    let file = File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open progression file {:?}: {}", path, e);
    });
    let reader = BufReader::new(file);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.contains("->") {
            // Parse edge: "I -> IV"
            let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                edges.push(ProgressionEdge {
                    from: parts[0].to_string(),
                    to: parts[1].to_string(),
                });
            } else {
                eprintln!("Warning: Malformed edge line: '{}'", line);
            }
        } else if line.contains("|") {
            // Parse node: "I | p | ,6,7,9,maj7,maj9"
            let parts: Vec<&str> = line.split("|").map(|s| s.trim()).collect();
            if parts.len() >= 3 {
                let variants = if parts[2].is_empty() {
                    vec!["".to_string()]
                } else {
                    parts[2].split(',').map(|s| s.trim().to_string()).collect()
                };

                nodes.push(ProgressionNode {
                    id: parts[0].to_string(),
                    node_type: parts[1].to_string(),
                    variants,
                });
            } else {
                eprintln!(
                    "Warning: Malformed node line: '{}' (expected format: ID|type|variants)",
                    line
                );
            }
        }
    }

    // Validate progression data before generating code
    validate_progression_data(&nodes, &edges);

    generate_progression_code(&nodes, &edges, key_type)
}

fn generate_progression_code(
    nodes: &[ProgressionNode],
    edges: &[ProgressionEdge],
    key_type: &str,
) -> String {
    let mut generated = String::new();

    generated.push_str(&format!(
        "//! Generated progression data for {} keys from {}.progression\n//! Do not edit manually.\n\n",
        key_type, key_type
    ));

    generated.push_str("use crate::types::progression::{ProgressionEdge, NodeType};\n");
    generated.push_str("use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};\n");
    generated.push_str("use crate::types::chord::BassType;\n");
    generated.push_str("use std::collections::HashMap;\n\n");

    // Generate common interval patterns as const IntervalSet instances
    generated.push_str("// Common interval patterns (reused across multiple chords)\n");
    generated.push_str("/// Standard major triad intervals: root, major third, perfect fifth\n");
    generated.push_str("const MAJOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(\n");
    generated.push_str(
        "    [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,\n",
    );
    generated.push_str("     Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("     Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("     Interval::NONE], 3);\n\n");

    generated.push_str("/// Standard minor triad intervals: root, minor third, perfect fifth\n");
    generated.push_str("const MINOR_TRIAD_SET: IntervalSet = IntervalSet::const_from_array(\n");
    generated.push_str(
        "    [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH,\n",
    );
    generated.push_str("     Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("     Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("     Interval::NONE], 3);\n\n");

    // Generate individual node variants
    let mut all_node_names = Vec::new();
    let mut all_edge_names = Vec::new();
    let mut node_type_entries = Vec::new();

    for node in nodes {
        for variant in &node.variants {
            let (node_name, display_name) = generate_node_variant(node, variant);
            all_node_names.push(node_name.clone());

            let node_type = match node.node_type.as_str() {
                "p" | "primary" => "NodeType::Primary",
                "s" | "secondary" => "NodeType::Secondary",
                _ => panic!("Unknown node type: {}", node.node_type),
            };

            let (roman_numeral_code, intervals_array_ref, bass_degree) =
                generate_chord_data(&node.id, variant, &node_name);

            // Generate custom IntervalSet constant only if not using common patterns
            if let Some(intervals_const) = &intervals_array_ref.custom_array {
                generated.push_str(intervals_const);
            }

            // Generate documentation for the node
            let (_, quality_hint, _) = parse_roman_degree(&node.id);
            let doc_comment = format!(
                "/// {} chord - {} ({} node)\n/// Intervals: {}\n",
                display_name,
                if node.node_type == "primary" {
                    "stable harmonic center"
                } else {
                    "creates tension, seeks resolution"
                },
                node.node_type,
                format_intervals_for_doc(&parse_chord_variant(variant, &quality_hint, &node.id))
            );

            // Generate the RomanChord - need to construct IntervalSet inline for const contexts
            let intervals_set_construction =
                generate_interval_set_construction(&intervals_array_ref.reference);
            generated.push_str(&doc_comment);
            let bass_field = if let Some(bass) = bass_degree {
                format!("Some(({}, BassType::Slash))", bass)
            } else {
                "None".to_string()
            };

            generated.push_str(&format!(
                "pub static {}: RomanChord = RomanChord {{\n    root: {},\n    intervals: {},\n    bass: {},\n}};\n\n",
                node_name, roman_numeral_code, intervals_set_construction, bass_field
            ));

            node_type_entries.push((node_name.clone(), node_type.to_string()));
        }
    }

    // Generate edges between all variant combinations
    for edge in edges {
        let from_variants = get_node_variants(nodes, &edge.from);
        let to_variants = get_node_variants(nodes, &edge.to);

        for from_variant in &from_variants {
            for to_variant in &to_variants {
                let from_base = sanitize_roman_numeral_id(&edge.from);
                let to_base = sanitize_roman_numeral_id(&edge.to);
                let from_suffix = if from_variant.is_empty() {
                    "".to_string()
                } else {
                    format!("_{}", sanitize_identifier(from_variant))
                };
                let to_suffix = if to_variant.is_empty() {
                    "".to_string()
                } else {
                    format!("_{}", sanitize_identifier(to_variant))
                };

                let edge_name = clean_identifier(&format!(
                    "EDGE_{}{}_TO_{}{}",
                    from_base, from_suffix, to_base, to_suffix
                ));
                let from_node_name = clean_identifier(&format!("{}{}", from_base, from_suffix));
                let to_node_name = clean_identifier(&format!("{}{}", to_base, to_suffix));

                generated.push_str(&format!(
                    "/// Progression edge: {} → {}\n",
                    edge.from, edge.to
                ));
                generated.push_str(&format!(
                    "pub static {}: ProgressionEdge = ProgressionEdge {{\n    from: {},\n    to: {},\n}};\n\n",
                    edge_name, from_node_name, to_node_name
                ));

                all_edge_names.push(edge_name);
            }
        }
    }

    // Generate ALL_NODES array (now using RomanChord)
    generated.push_str(&format!(
        "/// Complete registry of all progression chords for {} keys\n",
        key_type
    ));
    generated.push_str(&format!(
        "/// \n/// Contains {} chord variants across all harmonic functions.\n",
        all_node_names.len()
    ));
    generated.push_str("/// Used internally for graph traversal and chord lookup operations.\n");
    generated.push_str("pub static ALL_NODES: &[&RomanChord] = &[\n");
    for node_name in &all_node_names {
        generated.push_str(&format!("    &{},\n", node_name));
    }
    generated.push_str("];\n\n");

    // Generate ALL_EDGES array
    generated.push_str(&format!(
        "/// Complete registry of all progression edges for {} keys\n",
        key_type
    ));
    generated.push_str(&format!(
        "/// \n/// Contains {} harmonic connections between chord variants.\n",
        all_edge_names.len()
    ));
    generated.push_str(
        "/// Each edge represents a musically valid progression with proper voice leading.\n",
    );
    generated.push_str("pub static ALL_EDGES: &[&ProgressionEdge] = &[\n");
    for edge_name in &all_edge_names {
        generated.push_str(&format!("    &{},\n", edge_name));
    }
    generated.push_str("];\n\n");

    // Generate NodeType mapping
    generated.push_str(&format!(
        "/// NodeType mapping for all progression chords in {} keys\n",
        key_type
    ));
    generated.push_str("/// \n/// Maps each chord to its harmonic role (Primary = stable, Secondary = transitional).\n");
    generated.push_str("pub fn get_node_types() -> HashMap<&'static RomanChord, NodeType> {\n");
    generated.push_str("    let mut map = HashMap::new();\n");
    for (node_name, node_type) in &node_type_entries {
        generated.push_str(&format!("    map.insert(&{}, {});\n", node_name, node_type));
    }
    generated.push_str("    map\n");
    generated.push_str("}\n\n");

    generated
}

fn generate_node_variant(node: &ProgressionNode, variant: &str) -> (String, String) {
    // Convert roman numerals to identifiers while preserving case semantics
    let base_name = sanitize_roman_numeral_id(&node.id);
    let variant_suffix = if variant.is_empty() {
        "".to_string()
    } else {
        format!("_{}", sanitize_identifier(variant))
    };

    let node_name = clean_identifier(&format!("{}{}", base_name, variant_suffix));

    let display_name = if variant.is_empty() {
        node.id.clone()
    } else {
        format!("{}{}", node.id, variant)
    };

    (node_name, display_name)
}

fn sanitize_roman_numeral_id(id: &str) -> String {
    // Special handling for roman numerals to preserve major/minor distinctions
    let mut result = id.replace("/", "_SLASH_").replace("b", "FLAT_");

    // Convert to uppercase but use special prefixes for lowercase roman numerals
    // Lowercase = minor, uppercase = major in roman numeral analysis
    if id.chars().any(|c| c.is_lowercase() && c.is_alphabetic()) {
        result = format!("MINOR_{}", result.to_uppercase());
    } else {
        result = result.to_uppercase();
    }

    result
}

fn sanitize_identifier(s: &str) -> String {
    s.replace("+", "_PLUS_")
        .replace("#", "_SHARP_")
        .replace("b", "_FLAT_")
        .replace("/", "_SLASH_")
        .to_uppercase()
}

/// Clean up identifier names by removing successive underscores
fn clean_identifier(s: &str) -> String {
    // Replace multiple consecutive underscores with a single underscore
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        result.push(ch);

        // If current char is underscore, skip any following underscores
        if ch == '_' {
            while chars.peek() == Some(&'_') {
                chars.next();
            }
        }
    }

    result
}

struct IntervalsArrayRef {
    reference: String,
    custom_array: Option<String>,
}

fn generate_chord_data(
    roman: &str,
    variant: &str,
    node_name: &str,
) -> (String, IntervalsArrayRef, Option<String>) {
    // Parse the roman numeral to get the degree and accidental
    let (degree_code, quality_hint, bass_degree) = parse_roman_degree(roman);

    // Generate the intervals for this chord variant
    let intervals = parse_chord_variant(variant, &quality_hint, roman);

    // Generate the RomanNumeral code
    let roman_numeral_code = format!("RomanNumeral::new({}, Accidental::Natural)", degree_code);

    // Check if we can use a common pattern
    let intervals_ref = if variant.is_empty() && quality_hint == "major" {
        IntervalsArrayRef {
            reference: "&MAJOR_TRIAD".to_string(),
            custom_array: None,
        }
    } else if variant.is_empty() && quality_hint == "minor" {
        IntervalsArrayRef {
            reference: "&MINOR_TRIAD".to_string(),
            custom_array: None,
        }
    } else {
        // Generate custom IntervalSet constant
        let intervals_set_const = generate_custom_interval_set(&intervals, node_name);
        IntervalsArrayRef {
            reference: format!("{}_INTERVALS_SET", node_name),
            custom_array: Some(intervals_set_const),
        }
    };

    (roman_numeral_code, intervals_ref, bass_degree)
}

fn parse_roman_degree(roman: &str) -> (String, String, Option<String>) {
    // Check for slash chord notation (e.g., I/5, V/3)
    let (base_roman, bass_degree) = if let Some(slash_pos) = roman.find('/') {
        let (base, bass) = roman.split_at(slash_pos);
        let bass = &bass[1..]; // Skip the slash

        // Parse the bass degree number and convert to RomanNumeral
        let bass_degree_code = match bass {
            "1" => "RomanNumeral::new(RomanDegree::I, Accidental::Natural)",
            "2" => "RomanNumeral::new(RomanDegree::II, Accidental::Natural)",
            "3" => "RomanNumeral::new(RomanDegree::III, Accidental::Natural)",
            "4" => "RomanNumeral::new(RomanDegree::IV, Accidental::Natural)",
            "5" => "RomanNumeral::new(RomanDegree::V, Accidental::Natural)",
            "6" => "RomanNumeral::new(RomanDegree::VI, Accidental::Natural)",
            "7" => "RomanNumeral::new(RomanDegree::VII, Accidental::Natural)",
            "b3" => "RomanNumeral::new(RomanDegree::III, Accidental::Flat)",
            "b6" => "RomanNumeral::new(RomanDegree::VI, Accidental::Flat)",
            _ => panic!("Unknown bass degree: {}", bass),
        };
        (base, Some(bass_degree_code.to_string()))
    } else {
        (roman, None)
    };

    // Handle accidentals: 's' for sharp, 'b' for flat
    let (accidental, remainder) = if let Some(stripped) = base_roman.strip_prefix('s') {
        ("Accidental::Sharp", stripped)
    } else if let Some(stripped) = base_roman.strip_prefix('b') {
        ("Accidental::Flat", stripped)
    } else {
        ("Accidental::Natural", base_roman)
    };

    // Remove suffixes like "dim7", "m7b5", "m6", "7", "9" etc. for parsing the degree
    let cleaned_roman = remainder
        .replace("dim7", "")
        .replace("dim", "")
        .replace("m7b5", "")
        .replace("m6", "")
        .replace("m7", "")
        .replace("m9", "")
        .replace("7", "")
        .replace("9", "")
        .replace("o", "")
        .replace("aug", "");

    // Determine quality from case, content, and suffixes in the ID
    let quality = determine_chord_quality(roman);

    // Determine degree from cleaned remainder
    let degree = match cleaned_roman.to_uppercase().as_str() {
        "I" => "RomanDegree::I",
        "II" => "RomanDegree::II",
        "III" => "RomanDegree::III",
        "IV" => "RomanDegree::IV",
        "V" => "RomanDegree::V",
        "VI" => "RomanDegree::VI",
        "VII" => "RomanDegree::VII",
        _ => panic!(
            "Unknown roman degree: {} (cleaned from: {}, original: {})",
            cleaned_roman, remainder, roman
        ),
    };

    let _degree_with_accidental = if accidental == "Accidental::Natural" {
        degree.to_string()
    } else {
        format!("RomanNumeral::new({}, {}).degree()", degree, accidental)
    };

    (degree.to_string(), quality.to_string(), bass_degree)
}

/// Determines if a chord should be major or minor based on the roman numeral ID only
fn determine_chord_quality(roman_id: &str) -> String {
    // Check if the roman numeral itself is lowercase or contains diminished
    let is_lowercase = roman_id
        .chars()
        .any(|c| c.is_lowercase() && c.is_alphabetic());
    let has_dim = roman_id.contains("dim");

    // Check if the ID has minor suffixes like m6, m7, m9, m7b5
    let has_minor_suffix = roman_id.contains("m6")
        || roman_id.contains("m7")
        || roman_id.contains("m9")
        || roman_id.contains("m11")
        || roman_id.contains("m13");

    if has_minor_suffix || is_lowercase || has_dim {
        "minor".to_string()
    } else {
        "major".to_string()
    }
}

fn get_base_triad(quality_hint: &str) -> Vec<&'static str> {
    match quality_hint {
        "major" => vec![
            "Interval::PERFECT_UNISON",
            "Interval::MAJOR_THIRD",
            "Interval::PERFECT_FIFTH",
        ],
        "minor" => vec![
            "Interval::PERFECT_UNISON",
            "Interval::MINOR_THIRD",
            "Interval::PERFECT_FIFTH",
        ],
        _ => vec![
            "Interval::PERFECT_UNISON",
            "Interval::MAJOR_THIRD",
            "Interval::PERFECT_FIFTH",
        ],
    }
}

fn parse_chord_variant(variant: &str, quality_hint: &str, roman_id: &str) -> Vec<String> {
    if variant.is_empty() {
        // Base triad, but check if the roman_id itself has chord extensions
        let mut intervals: Vec<String> = get_base_triad(quality_hint)
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Apply extensions from the roman_id itself
        if roman_id.contains("m7b5") {
            // Half-diminished chord
            if let Some(pos) = intervals
                .iter()
                .position(|x| x == "Interval::PERFECT_FIFTH")
            {
                intervals[pos] = "Interval::DIMINISHED_FIFTH".to_string();
            }
            intervals.push("Interval::MINOR_SEVENTH".to_string());
        } else if roman_id.contains("m7") {
            // Minor seventh chord
            intervals.push("Interval::MINOR_SEVENTH".to_string());
        } else if roman_id.contains("m9") {
            // Minor ninth chord
            intervals.push("Interval::MINOR_SEVENTH".to_string());
            intervals.push("Interval::MAJOR_NINTH".to_string());
        } else if roman_id.contains("m6") {
            // Minor sixth chord
            intervals.push("Interval::MAJOR_SIXTH".to_string());
        } else if roman_id.contains("dim7") {
            // Diminished seventh chord
            if let Some(pos) = intervals
                .iter()
                .position(|x| x == "Interval::PERFECT_FIFTH")
            {
                intervals[pos] = "Interval::DIMINISHED_FIFTH".to_string();
            }
            intervals.push("Interval::DIMINISHED_SEVENTH".to_string());
        } else if roman_id.contains("7") && !roman_id.contains("maj7") {
            // Dominant seventh chord
            intervals.push("Interval::MINOR_SEVENTH".to_string());
        } else if roman_id.contains("maj7") {
            // Major seventh chord
            intervals.push("Interval::MAJOR_SEVENTH".to_string());
        } else if roman_id.contains("9") && !roman_id.contains("m9") {
            // Dominant ninth chord
            intervals.push("Interval::MINOR_SEVENTH".to_string());
            intervals.push("Interval::MAJOR_NINTH".to_string());
        } else if roman_id.contains("6") && !roman_id.contains("m6") {
            // Major sixth chord
            intervals.push("Interval::MAJOR_SIXTH".to_string());
        }

        return intervals;
    }

    // Start with base triad
    let mut intervals = get_base_triad(quality_hint);

    // Parse variant extensions
    match variant {
        "6" => intervals.push("Interval::MAJOR_SIXTH"),
        "7" => {
            // In jazz harmony, "7" always means dominant 7 (minor seventh)
            // regardless of chord quality
            intervals.push("Interval::MINOR_SEVENTH");
        }
        "9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        }
        "11" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::PERFECT_ELEVENTH");
        }
        "13" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::PERFECT_ELEVENTH");
            intervals.push("Interval::MAJOR_THIRTEENTH");
        }
        "maj7" => intervals.push("Interval::MAJOR_SEVENTH"),
        "maj9" => {
            intervals.push("Interval::MAJOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        }
        "m7" => {
            // Minor 7th chord: add minor seventh (triad quality determined by base ID)
            intervals.push("Interval::MINOR_SEVENTH");
        }
        "m9" => {
            // Minor 9th chord: add minor seventh + major ninth (triad quality determined by base ID)
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        }
        "7+b9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MINOR_NINTH");
        }
        "7+#9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::AUGMENTED_NINTH");
        }
        "b5" => {
            // Replace perfect fifth with diminished fifth
            if let Some(pos) = intervals
                .iter()
                .position(|&x| x == "Interval::PERFECT_FIFTH")
            {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
        }
        "m7b5" => {
            // Half-diminished: diminished fifth + minor seventh (minor third from base ID)
            if let Some(pos) = intervals
                .iter()
                .position(|&x| x == "Interval::PERFECT_FIFTH")
            {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
            intervals.push("Interval::MINOR_SEVENTH");
        }
        "b5+7" => {
            if let Some(pos) = intervals
                .iter()
                .position(|&x| x == "Interval::PERFECT_FIFTH")
            {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
            intervals.push("Interval::MINOR_SEVENTH");
        }
        "#11" => {
            intervals.push("Interval::MAJOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::AUGMENTED_ELEVENTH");
        }
        "add9" => {
            intervals.push("Interval::MAJOR_NINTH");
        }
        "m6" => {
            // Minor 6th chord: add major sixth (triad quality determined by base ID)
            intervals.push("Interval::MAJOR_SIXTH");
        }
        "b9" => {
            // b9 extension: add minor ninth (usually on dominant chords)
            intervals.push("Interval::MINOR_NINTH");
        }
        "b11" => {
            // b9 extension: add minor eleventh (usually on dominant chords)
            intervals.push("Interval::MINOR_ELEVENTH");
        }
        "b13" => {
            // b9 extension: add minor thirteenth (usually on dominant chords)
            intervals.push("Interval::MINOR_THIRTEENTH");
        }
        "sus2" => {
            // Suspended second: replace third with major second
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::MAJOR_THIRD") {
                intervals[pos] = "Interval::MAJOR_SECOND";
            }
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::MINOR_THIRD") {
                intervals[pos] = "Interval::MAJOR_SECOND";
            }
        }
        "sus4" => {
            // Suspended fourth: replace third with perfect fourth
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::MAJOR_THIRD") {
                intervals[pos] = "Interval::PERFECT_FOURTH";
            }
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::MINOR_THIRD") {
                intervals[pos] = "Interval::PERFECT_FOURTH";
            }
        }
        _ => {
            // For unknown variants, just return the base triad
            eprintln!(
                "Warning: Unknown chord variant '{}' for quality '{}'",
                variant, quality_hint
            );
        }
    }

    intervals.into_iter().map(|s| s.to_string()).collect()
}

fn validate_progression_data(nodes: &[ProgressionNode], edges: &[ProgressionEdge]) {
    // Check for duplicate node IDs
    let mut seen_ids = std::collections::HashSet::new();
    for node in nodes {
        if !seen_ids.insert(&node.id) {
            panic!("Duplicate node ID found: {}", node.id);
        }
    }

    // Check that all edge references point to valid nodes
    let node_ids: std::collections::HashSet<_> = nodes.iter().map(|n| n.id.as_str()).collect();
    for edge in edges {
        if !node_ids.contains(edge.from.as_str()) {
            eprintln!("Warning: Edge references unknown node '{}'", edge.from);
        }
        if !node_ids.contains(edge.to.as_str()) {
            eprintln!("Warning: Edge references unknown node '{}'", edge.to);
        }
    }

    // Check for valid node types
    for node in nodes {
        match node.node_type.as_str() {
            "p" | "primary" | "s" | "secondary" => {}
            _ => eprintln!(
                "Warning: Unknown node type '{}' for node '{}'",
                node.node_type, node.id
            ),
        }
    }

    println!(
        "cargo:warning=Validated {} nodes and {} edges",
        nodes.len(),
        edges.len()
    );
}

fn format_intervals_for_doc(intervals: &[String]) -> String {
    intervals
        .iter()
        .map(|i| i.replace("Interval::", "").replace("_", " ").to_lowercase())
        .collect::<Vec<_>>()
        .join(", ")
}

fn get_node_variants(nodes: &[ProgressionNode], id: &str) -> Vec<String> {
    for node in nodes {
        if node.id == id {
            return node.variants.clone();
        }
    }
    vec!["".to_string()]
}

fn generate_custom_interval_set(intervals: &[String], node_name: &str) -> String {
    // Pad intervals array to 10 elements with NONE for unused slots
    let mut padded_intervals = intervals.to_vec();
    while padded_intervals.len() < 10 {
        padded_intervals.push("Interval::NONE".to_string());
    }

    format!(
        "const {}_INTERVALS_SET: IntervalSet = IntervalSet::const_from_array(\n    [{}], {});\n\n",
        node_name,
        padded_intervals.join(",\n     "),
        intervals.len()
    )
}

fn generate_interval_set_construction(array_ref: &str) -> String {
    // Map array references to their IntervalSet equivalents
    match array_ref {
        "&MAJOR_TRIAD" => "MAJOR_TRIAD_SET".to_string(),
        "&MINOR_TRIAD" => "MINOR_TRIAD_SET".to_string(),
        _ => {
            // For custom arrays, we need to generate a const-compatible IntervalSet
            // Extract the array name and create a corresponding IntervalSet constant
            let array_name = array_ref.trim_start_matches('&');
            if array_name.ends_with("_INTERVALS_SET") {
                array_name.to_string()
            } else {
                format!("{}_SET", array_name.trim_end_matches("_INTERVALS"))
            }
        }
    }
}

fn generate_chord_progressions() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let csv_path = Path::new(&manifest_dir).join("data/chord_progressions.csv");

    let file = File::open(&csv_path).expect("Failed to open chord_progressions.csv");
    let reader = BufReader::new(file);

    let mut generated = String::new();

    generated
        .push_str("//! Curated chord progression database generated from chord_progressions.csv\n");
    generated.push_str("//! Do not edit manually.\n\n");
    generated.push_str("#![allow(non_upper_case_globals)]\n");
    generated.push_str("#![allow(dead_code)]\n\n");
    generated.push_str("use crate::types::{RomanChord, RomanNumeral, RomanDegree, Accidental, Interval, IntervalSet};\n\n");

    // Generate standard chord constants that we'll reference
    generated.push_str("// Standard chord constants (major triads)\n");
    generated.push_str("static I_CHORD: RomanChord = RomanChord {\n");
    generated.push_str("    root: RomanNumeral::new(RomanDegree::I, Accidental::Natural),\n");
    generated.push_str("    intervals: IntervalSet::const_from_array(\n");
    generated.push_str(
        "        [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH,\n",
    );
    generated.push_str("         Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("         Interval::NONE, Interval::NONE, Interval::NONE,\n");
    generated.push_str("         Interval::NONE], 3),\n");
    generated.push_str("    bass: None,\n");
    generated.push_str("};\n\n");

    // Generate other basic chords
    let basic_chords = [
        ("II", "RomanDegree::II", "major"),
        ("III", "RomanDegree::III", "major"),
        ("IV", "RomanDegree::IV", "major"),
        ("V", "RomanDegree::V", "major"),
        ("VI", "RomanDegree::VI", "major"),
        ("VII", "RomanDegree::VII", "major"),
        ("ii", "RomanDegree::II", "minor"),
        ("iii", "RomanDegree::III", "minor"),
        ("vi", "RomanDegree::VI", "minor"),
        ("bII", "RomanDegree::II", "major_flat"),
        ("bIII", "RomanDegree::III", "major_flat"),
        ("bV", "RomanDegree::V", "major_flat"),
        ("bVI", "RomanDegree::VI", "major_flat"),
        ("bVII", "RomanDegree::VII", "major_flat"),
        ("iv", "RomanDegree::IV", "minor"),
    ];

    for (name, degree, quality) in &basic_chords {
        let (intervals, accidental) = match *quality {
            "major" => (
                "MAJOR_THIRD, Interval::PERFECT_FIFTH",
                "Accidental::Natural",
            ),
            "minor" => (
                "MINOR_THIRD, Interval::PERFECT_FIFTH",
                "Accidental::Natural",
            ),
            "major_flat" => ("MAJOR_THIRD, Interval::PERFECT_FIFTH", "Accidental::Flat"),
            _ => (
                "MAJOR_THIRD, Interval::PERFECT_FIFTH",
                "Accidental::Natural",
            ),
        };

        generated.push_str(&format!(
            "static {}_CHORD: RomanChord = RomanChord {{\n",
            name
        ));
        generated.push_str(&format!(
            "    root: RomanNumeral::new({}, {}),\n",
            degree, accidental
        ));
        generated.push_str("    intervals: IntervalSet::const_from_array(\n");
        generated.push_str(&format!(
            "        [Interval::PERFECT_UNISON, Interval::{},\n",
            intervals
        ));
        generated.push_str("         Interval::NONE, Interval::NONE, Interval::NONE,\n");
        generated.push_str("         Interval::NONE, Interval::NONE, Interval::NONE,\n");
        generated.push_str("         Interval::NONE], 3),\n");
        generated.push_str("    bass: None,\n");
        generated.push_str("};\n\n");
    }

    let mut tier_1_progressions = Vec::new();
    let mut tier_2_progressions = Vec::new();
    let mut tier_3_progressions = Vec::new();
    let mut tier_4_progressions = Vec::new();
    let mut all_progressions = Vec::new();

    let mut progression_counter = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse CSV with proper quote handling: "name",tier,"roman_progression","description"
        let parts = parse_csv_line(line);
        if parts.len() < 4 {
            eprintln!(
                "Warning: Malformed line: '{}' - got {} parts",
                line,
                parts.len()
            );
            continue;
        }

        let name = parts[0].trim_matches('"');
        let tier: u8 = parts[1].parse().expect("Invalid tier number");
        let roman_progression = parts[2].trim_matches('"');
        let description = parts[3].trim_matches('"');

        // Parse roman numeral sequence
        let chords: Vec<&str> = roman_progression.split(',').map(|s| s.trim()).collect();
        if chords.len() != 4 {
            eprintln!(
                "Warning: Progression '{}' doesn't have exactly 4 chords",
                name
            );
            continue;
        }

        let progression_name = format!("PROGRESSION_{}", progression_counter);
        progression_counter += 1;

        // Generate progression constant
        generated.push_str(&format!("/// {}: {}\n", name, description));
        generated.push_str(&format!("/// Chords: {}\n", roman_progression));
        generated.push_str(&format!(
            "static {}: [&'static RomanChord; 4] = [\n",
            progression_name
        ));

        for chord in &chords {
            let chord_ref = format!("&{}_CHORD", chord);
            generated.push_str(&format!("    {},\n", chord_ref));
        }

        generated.push_str("];\n\n");

        // Add to appropriate tier
        match tier {
            1 => tier_1_progressions.push(progression_name.clone()),
            2 => tier_2_progressions.push(progression_name.clone()),
            3 => tier_3_progressions.push(progression_name.clone()),
            4 => tier_4_progressions.push(progression_name.clone()),
            _ => eprintln!("Warning: Invalid tier {} for progression '{}'", tier, name),
        }

        all_progressions.push((progression_name, name.to_string(), description.to_string()));
    }

    // Generate tier arrays
    generate_tier_array(
        &mut generated,
        "TIER_1_PROGRESSIONS",
        &tier_1_progressions,
        "Pop Standards (0.0-0.25)",
    );
    generate_tier_array(
        &mut generated,
        "TIER_2_PROGRESSIONS",
        &tier_2_progressions,
        "Jazz Fundamentals (0.25-0.5)",
    );
    generate_tier_array(
        &mut generated,
        "TIER_3_PROGRESSIONS",
        &tier_3_progressions,
        "Sophisticated Jazz (0.5-0.75)",
    );
    generate_tier_array(
        &mut generated,
        "TIER_4_PROGRESSIONS",
        &tier_4_progressions,
        "Modern/Experimental (0.75-1.0)",
    );

    // Generate master tier selection function
    generated.push_str("/// Select progression tier based on complexity value (0.0-1.0)\n");
    generated.push_str("pub fn select_progression_tier(complexity: f32) -> &'static [&'static [&'static RomanChord; 4]] {\n");
    generated.push_str("    match complexity {\n");
    generated.push_str("        x if x < 0.25 => TIER_1_PROGRESSIONS,\n");
    generated.push_str("        x if x < 0.5 => TIER_2_PROGRESSIONS,\n");
    generated.push_str("        x if x < 0.75 => TIER_3_PROGRESSIONS,\n");
    generated.push_str("        _ => TIER_4_PROGRESSIONS,\n");
    generated.push_str("    }\n");
    generated.push_str("}\n\n");

    // Generate metadata
    generated.push_str("/// Progression metadata for descriptions and names\n");
    generated.push_str("pub static PROGRESSION_METADATA: &[(usize, &str, &str)] = &[\n");
    for (i, (_, name, description)) in all_progressions.iter().enumerate() {
        generated.push_str(&format!(
            "    ({}, \"{}\", \"{}\"),\n",
            i, name, description
        ));
    }
    generated.push_str("];\n");

    let output_path = Path::new(&manifest_dir).join("src/types/progression/chord_progressions.rs");
    fs::write(&output_path, &generated).expect("Failed to write chord progressions module");

    println!(
        "cargo:warning=Generated {} chord progressions across 4 complexity tiers",
        all_progressions.len()
    );
}

fn generate_tier_array(
    generated: &mut String,
    tier_name: &str,
    progressions: &[String],
    description: &str,
) {
    generated.push_str(&format!(
        "/// {} - {} progressions\n",
        description,
        progressions.len()
    ));
    generated.push_str(&format!(
        "pub static {}: &[&[&'static RomanChord; 4]] = &[\n",
        tier_name
    ));

    for progression in progressions {
        generated.push_str(&format!("    &{},\n", progression));
    }

    generated.push_str("];\n\n");
}

/// Simple CSV parser that handles quoted fields containing commas
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let chars = line.chars().peekable();

    for ch in chars {
        match ch {
            '"' => {
                // Toggle quote state
                in_quotes = !in_quotes;
            }
            ',' if !in_quotes => {
                // Field separator - end current field
                fields.push(current_field.trim().to_string());
                current_field.clear();
            }
            _ => {
                // Regular character - add to current field
                current_field.push(ch);
            }
        }
    }

    // Don't forget the last field
    fields.push(current_field.trim().to_string());

    fields
}

fn generate_guitar_shapes() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let shapes_path = Path::new(&manifest_dir).join("data/guitar_shapes.shapes");

    let file = File::open(&shapes_path).expect("Failed to open guitar_shapes.shapes");
    let reader = BufReader::new(file);

    let mut generated = String::new();

    generated.push_str("//! Generated guitar chord shapes from guitar_shapes.shapes\n");
    generated.push_str("//! Do not edit manually.\n\n");
    generated.push_str("#![allow(dead_code)]\n\n");
    generated.push_str("use crate::types::voicing::GuitarShape;\n\n");

    let mut shapes = Vec::new();
    let mut shape_counter = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse shape format: "1-3-3-2-1-1    # F major barre" or "2-X-0-2-3-2    # D/F# with muted A"
        if let Some(shape_part) = line.split('#').next() {
            let shape_part = shape_part.trim();
            if !shape_part.is_empty() {
                let positions: Result<Vec<u8>, _> = shape_part
                    .split('-')
                    .map(|s| {
                        if s.trim() == "X" {
                            Ok(255u8) // Use 255 as muted string marker
                        } else {
                            s.parse::<u8>()
                        }
                    })
                    .collect();

                match positions {
                    Ok(pos) if !pos.is_empty() => {
                        let shape_name = format!("GUITAR_SHAPE_{}", shape_counter);
                        shape_counter += 1;

                        // Extract comment for documentation
                        let comment = line.split('#').nth(1).unwrap_or("Guitar shape").trim();

                        // Generate the shape constant
                        generated.push_str(&format!("/// {}\n", comment));
                        generated.push_str(&format!("/// Pattern: {}\n", shape_part));
                        generated.push_str(&format!(
                            "pub const {}: GuitarShape = GuitarShape {{\n",
                            shape_name
                        ));
                        generated.push_str(&format!(
                            "    positions: &[{}],\n",
                            pos.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ));
                        generated.push_str("};\n\n");

                        shapes.push(shape_name);
                    }
                    _ => {
                        eprintln!("Warning: Failed to parse shape line: '{}'", line);
                    }
                }
            }
        }
    }

    // Generate the shapes array
    generated.push_str(&format!(
        "/// All guitar chord shapes ({} total)\n",
        shapes.len()
    ));
    generated.push_str("pub static ALL_GUITAR_SHAPES: &[&'static GuitarShape] = &[\n");
    for shape_name in &shapes {
        generated.push_str(&format!("    &{},\n", shape_name));
    }
    generated.push_str("];\n\n");

    // Generate shape count
    generated.push_str("/// Number of guitar shapes available\n");
    generated.push_str(&format!(
        "pub const GUITAR_SHAPE_COUNT: usize = {};\n",
        shapes.len()
    ));

    let output_path = Path::new(&manifest_dir).join("src/types/guitar_shapes.rs");
    fs::write(&output_path, &generated).expect("Failed to write guitar shapes module");

    println!(
        "cargo:warning=Generated {} guitar chord shapes",
        shapes.len()
    );
}
