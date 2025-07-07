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
                if parent.is_empty() { "".to_string() } else { format!(" (mode {} of {})", offset.parse::<i8>().unwrap() + 1, parent) },
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

    generated.push_str("/// Registry of all scales.
pub const REGISTRY: &[ScaleDefinition] = &[\n");
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
    let major_path = Path::new(&manifest_dir).join("data/progressions/major_simple.progression");
    let major_data = parse_progression_file(&major_path, "major");
    let major_output = Path::new(&manifest_dir).join("src/types/progression/major_data.rs");
    fs::write(&major_output, &major_data).expect("Failed to write major progression data");
    
    // Generate minor progressions  
    let minor_path = Path::new(&manifest_dir).join("data/progressions/minor_simple.progression");
    let minor_data = parse_progression_file(&minor_path, "minor");
    let minor_output = Path::new(&manifest_dir).join("src/types/progression/minor_data.rs");
    fs::write(&minor_output, &minor_data).expect("Failed to write minor progression data");
}

#[derive(Debug)]
struct ProgressionNode {
    id: String,
    node_type: String,
    roman: String,
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
            // Parse node: "I | primary | I | ,6,7,9,maj7,maj9"
            let parts: Vec<&str> = line.split("|").map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let variants = if parts[3].is_empty() {
                    vec!["".to_string()]
                } else {
                    parts[3].split(',').map(|s| s.trim().to_string()).collect()
                };
                
                nodes.push(ProgressionNode {
                    id: parts[0].to_string(),
                    node_type: parts[1].to_string(),
                    roman: parts[2].to_string(),
                    variants,
                });
            } else {
                eprintln!("Warning: Malformed node line: '{}' (expected format: ID|type|roman|variants)", line);
            }
        }
    }
    
    // Validate progression data before generating code
    validate_progression_data(&nodes, &edges);
    
    generate_progression_code(&nodes, &edges, key_type)
}

fn generate_progression_code(nodes: &[ProgressionNode], edges: &[ProgressionEdge], key_type: &str) -> String {
    let mut generated = String::new();
    
    generated.push_str(&format!(
        "//! Generated progression data for {} keys from {}.progression\n//! Do not edit manually.\n\n",
        key_type, key_type
    ));
    
    generated.push_str("use crate::types::progression::{ProgressionNode, ProgressionEdge, NodeType};\n");
    generated.push_str("use crate::types::{RomanNumeral, RomanDegree, Accidental, Interval};\n\n");
    
    // Generate common interval arrays to reduce duplication
    generated.push_str("// Common interval patterns (reused across multiple chords)\n");
    generated.push_str("/// Standard major triad intervals: root, major third, perfect fifth\n");
    generated.push_str("static MAJOR_TRIAD: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MAJOR_THIRD, Interval::PERFECT_FIFTH];\n");
    generated.push_str("/// Standard minor triad intervals: root, minor third, perfect fifth\n");
    generated.push_str("static MINOR_TRIAD: [Interval; 3] = [Interval::PERFECT_UNISON, Interval::MINOR_THIRD, Interval::PERFECT_FIFTH];\n\n");
    
    // Generate individual node variants
    let mut all_node_names = Vec::new();
    let mut all_edge_names = Vec::new();
    let mut node_map_entries = Vec::new();
    
    for node in nodes {
        for variant in &node.variants {
            let (node_name, display_name) = generate_node_variant(&node, variant);
            all_node_names.push(node_name.clone());
            
            let node_type = match node.node_type.as_str() {
                "primary" => "NodeType::Primary",
                "secondary" => "NodeType::Secondary",
                _ => panic!("Unknown node type: {}", node.node_type),
            };
            
            let (roman_numeral_code, intervals_array_ref) = generate_chord_data(&node.roman, variant, &node_name);
            
            // Generate custom intervals array only if not using common patterns
            if let Some(intervals_array) = &intervals_array_ref.custom_array {
                generated.push_str(intervals_array);
            }
            
            // Generate documentation for the node
            let (_, quality_hint) = parse_roman_degree(&node.roman);
            let doc_comment = format!(
                "/// {} chord - {} ({} node)\n/// Intervals: {}\n",
                display_name,
                if node.node_type == "primary" { "stable harmonic center" } else { "creates tension, seeks resolution" },
                node.node_type,
                format_intervals_for_doc(&parse_chord_variant(variant, &quality_hint, ""))
            );
            
            // Generate the node
            generated.push_str(&doc_comment);
            generated.push_str(&format!(
                "pub static {}: ProgressionNode = ProgressionNode {{\n    id: \"{}\",\n    display_name: \"{}\",\n    node_type: {},\n    roman_numeral: {},\n    intervals: {},\n    base_function: \"{}\",\n}};\n\n",
                node_name, display_name, display_name, node_type, roman_numeral_code, intervals_array_ref.reference, node.id
            ));
            
            node_map_entries.push((display_name.clone(), node_name.clone()));
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
                let from_suffix = if from_variant.is_empty() { "".to_string() } else { format!("_{}", sanitize_identifier(from_variant)) };
                let to_suffix = if to_variant.is_empty() { "".to_string() } else { format!("_{}", sanitize_identifier(to_variant)) };
                
                let edge_name = format!("EDGE_{}{}_TO_{}{}", 
                    from_base, from_suffix, to_base, to_suffix);
                let from_node_name = format!("{}{}", from_base, from_suffix);
                let to_node_name = format!("{}{}", to_base, to_suffix);
                
                generated.push_str(&format!("/// Progression edge: {} → {}\n", edge.from, edge.to));
                generated.push_str(&format!(
                    "pub static {}: ProgressionEdge = ProgressionEdge {{\n    from: &{},\n    to: &{},\n}};\n\n",
                    edge_name, from_node_name, to_node_name
                ));
                
                all_edge_names.push(edge_name);
            }
        }
    }
    
    // Generate ALL_NODES array
    generated.push_str(&format!("/// Complete registry of all progression nodes for {} keys\n", key_type));
    generated.push_str(&format!("/// \n/// Contains {} chord variants across all harmonic functions.\n", all_node_names.len()));
    generated.push_str(&format!("/// Used internally for graph traversal and chord lookup operations.\n"));
    generated.push_str("pub static ALL_NODES: &[&ProgressionNode] = &[\n");
    for node_name in &all_node_names {
        generated.push_str(&format!("    &{},\n", node_name));
    }
    generated.push_str("];\n\n");
    
    // Generate ALL_EDGES array
    generated.push_str(&format!("/// Complete registry of all progression edges for {} keys\n", key_type));
    generated.push_str(&format!("/// \n/// Contains {} harmonic connections between chord variants.\n", all_edge_names.len()));
    generated.push_str(&format!("/// Each edge represents a musically valid progression with proper voice leading.\n"));
    generated.push_str("pub static ALL_EDGES: &[&ProgressionEdge] = &[\n");
    for edge_name in &all_edge_names {
        generated.push_str(&format!("    &{},\n", edge_name));
    }
    generated.push_str("];\n\n");
    
    // Generate simple lookup function instead of PHF map
    generated.push_str(&format!("/// Look up a progression node by its display name for {} keys\n", key_type));
    generated.push_str(&format!("/// \n/// Returns the corresponding `ProgressionNode` for chord symbols like \"I\", \"V7\", \"ii9\", etc.\n"));
    generated.push_str(&format!("/// Supports {} different chord variants.\n", node_map_entries.len()));
    generated.push_str("pub fn get_node(name: &str) -> Option<&'static ProgressionNode> {\n");
    generated.push_str("    match name {\n");
    for (display_name, node_name) in &node_map_entries {
        generated.push_str(&format!("        \"{}\" => Some(&{}),\n", display_name, node_name));
    }
    generated.push_str("        _ => None,\n");
    generated.push_str("    }\n");
    generated.push_str("}\n");
    
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
    
    let node_name = format!("{}{}", base_name, variant_suffix);
    
    let display_name = if variant.is_empty() {
        node.id.clone()
    } else {
        format!("{}{}", node.id, variant)
    };
    
    (node_name, display_name)
}

fn sanitize_roman_numeral_id(id: &str) -> String {
    // Special handling for roman numerals to preserve major/minor distinctions
    let mut result = id.replace("/", "_SLASH_")
                      .replace("b", "FLAT_");
    
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


struct IntervalsArrayRef {
    reference: String,
    custom_array: Option<String>,
}

fn generate_chord_data(roman: &str, variant: &str, node_name: &str) -> (String, IntervalsArrayRef) {
    // Parse the roman numeral to get the degree and accidental
    let (degree_code, quality_hint) = parse_roman_degree(roman);
    
    // Generate the intervals for this chord variant
    let intervals = parse_chord_variant(variant, &quality_hint, "");
    
    // Generate the RomanNumeral code
    let roman_numeral_code = format!(
        "RomanNumeral::new({}, Accidental::Natural)",
        degree_code
    );
    
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
        // Generate custom intervals array
        let intervals_array = format!(
            "static {}_INTERVALS: [Interval; {}] = [{}];\n\n",
            node_name,
            intervals.len(),
            intervals.join(", ")
        );
        IntervalsArrayRef {
            reference: format!("&{}_INTERVALS", node_name),
            custom_array: Some(intervals_array),
        }
    };
    
    (roman_numeral_code, intervals_ref)
}

fn parse_roman_degree(roman: &str) -> (String, String) {
    // Handle flat symbols
    let (accidental, remainder) = if roman.starts_with('b') {
        ("Accidental::Flat", &roman[1..])
    } else if roman.starts_with('#') {
        ("Accidental::Sharp", &roman[1..])
    } else {
        ("Accidental::Natural", roman)
    };
    
    // Determine quality from case and convert to degree
    let (degree, quality) = match remainder.to_uppercase().as_str() {
        "I" => ("RomanDegree::I", if remainder == "i" { "minor" } else { "major" }),
        "II" => ("RomanDegree::II", if remainder == "ii" { "minor" } else { "major" }),
        "III" => ("RomanDegree::III", if remainder == "iii" { "minor" } else { "major" }),
        "IV" => ("RomanDegree::IV", if remainder == "iv" { "minor" } else { "major" }),
        "V" => ("RomanDegree::V", if remainder == "v" { "minor" } else { "major" }),
        "VI" => ("RomanDegree::VI", if remainder == "vi" { "minor" } else { "major" }),
        "VII" => ("RomanDegree::VII", if remainder == "vii" { "minor" } else { "major" }),
        _ => panic!("Unknown roman degree: {}", remainder),
    };
    
    let _degree_with_accidental = if accidental == "Accidental::Natural" {
        degree.to_string()
    } else {
        format!("RomanNumeral::new({}, {}).degree()", degree, accidental)
    };
    
    (degree.to_string(), quality.to_string())
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

fn parse_chord_variant(variant: &str, quality_hint: &str, _key_type: &str) -> Vec<String> {
    if variant.is_empty() {
        // Base triad
        return get_base_triad(quality_hint).into_iter().map(|s| s.to_string()).collect();
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
        },
        "9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        },
        "11" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::PERFECT_ELEVENTH");
        },
        "13" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::PERFECT_ELEVENTH");
            intervals.push("Interval::MAJOR_THIRTEENTH");
        },
        "maj7" => intervals.push("Interval::MAJOR_SEVENTH"),
        "maj9" => {
            intervals.push("Interval::MAJOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        },
        "m7" => intervals.push("Interval::MINOR_SEVENTH"),
        "m9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
        },
        "7+b9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::MINOR_NINTH");
        },
        "7+#9" => {
            intervals.push("Interval::MINOR_SEVENTH");
            intervals.push("Interval::AUGMENTED_NINTH");
        },
        "b5" => {
            // Replace perfect fifth with diminished fifth
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::PERFECT_FIFTH") {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
        },
        "m7b5" => {
            // Half-diminished: minor third, diminished fifth, minor seventh
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::PERFECT_FIFTH") {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
            intervals.push("Interval::MINOR_SEVENTH");
        },
        "b5+7" => {
            if let Some(pos) = intervals.iter().position(|&x| x == "Interval::PERFECT_FIFTH") {
                intervals[pos] = "Interval::DIMINISHED_FIFTH";
            }
            intervals.push("Interval::MINOR_SEVENTH");
        },
        "#11" => {
            intervals.push("Interval::MAJOR_SEVENTH");
            intervals.push("Interval::MAJOR_NINTH");
            intervals.push("Interval::AUGMENTED_ELEVENTH");
        },
        _ => {
            // For unknown variants, just return the base triad
            eprintln!("Warning: Unknown chord variant '{}' for quality '{}'", variant, quality_hint);
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
            "primary" | "secondary" => {},
            _ => eprintln!("Warning: Unknown node type '{}' for node '{}'", node.node_type, node.id),
        }
    }
    
    println!("cargo:warning=Validated {} nodes and {} edges", nodes.len(), edges.len());
}

fn format_intervals_for_doc(intervals: &[String]) -> String {
    intervals.iter()
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
