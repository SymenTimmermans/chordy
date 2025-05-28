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
        "m3" => 3,
        "M3" | "d4" => 4,
        "P4" => 5,
        "A4" => 6,
        "d5" => 6,
        "P5" => 7,
        "m6" | "A5" => 8,
        "M6" => 9,
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

            generated.push_str(&format!(
                "pub const {}: ScaleDefinition = ScaleDefinition {{
    name: \"{}\",
    intervals: &[{}],
    bitmask: ScaleBitmask(0b{:012b}),
    mode_of: {},
    degree_offset: {},
}};\n\n",
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

    generated.push_str("pub const REGISTRY: &[ScaleDefinition] = &[\n");
    for name in &registry_entries {
        generated.push_str(&format!("    {},\n", name));
    }
    generated.push_str("];\n");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_file = Path::new(&manifest_dir).join("src/types/scale/scales.rs");

    fs::write(&out_file, &generated).expect("Failed to write scales registry module.");
}
