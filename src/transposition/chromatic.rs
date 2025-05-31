use super::Transposer;
use crate::types::{Accidental, Letter};
use crate::Pitch;

/// Chromatic (semitone-based) transposition with smart enharmonic spelling
pub struct ChromaticTransposer;

impl Transposer for ChromaticTransposer {
    fn transpose(pitch: Pitch, semitone_offset: i8) -> Pitch {
        if semitone_offset == 0 {
            return pitch;
        }

        let target_midi = pitch.midi_number() + semitone_offset;

        let mut best_pitch = None;
        let mut best_score = i32::MAX;

        for letter in Letter::all() {
            let base_index = letter.base_midi_number();
            let candidate_octave = (target_midi / 12) - 2;

            for accidental in Accidental::all() {
                let semitone =
                    base_index + accidental.semitone_offset() + 12 * (candidate_octave + 2);
                let mut pitch_guess = Pitch::new(letter, accidental, candidate_octave);

                let diff = semitone - target_midi;
                if diff > 6 {
                    pitch_guess.octave -= 1;
                } else if diff < -6 {
                    pitch_guess.octave += 1;
                }

                let final_midi = pitch_guess.midi_number();
                if final_midi != target_midi {
                    continue;
                }

                let interval_class_penalty = interval_penalty(&pitch, &pitch_guess);
                let spelling_penalty = if pitch.name.accidental() != accidental {
                    accidental.penalty()
                } else {
                    0
                };

                let letter_distance =
                    (pitch_guess.name.letter() as i8 - pitch.name.letter() as i8).rem_euclid(7);
                let unnatural_motion_penalty = if letter_distance == 6 || letter_distance == 0 {
                    2
                } else {
                    0
                };
                let enharmonic_suspicion_penalty = if pitch_guess.is_suspicious_spelling() {
                    3
                } else {
                    0
                };

                let expected = expected_letter(pitch.name.letter(), semitone_offset);
                let letter_bias_penalty = if letter != expected { 2 } else { 0 };
                let direction_bias_penalty = flat_vs_sharp_bias(accidental, semitone_offset);
                let total_penalty = interval_class_penalty
                    + spelling_penalty
                    + unnatural_motion_penalty
                    + enharmonic_suspicion_penalty
                    + letter_bias_penalty
                    + direction_bias_penalty;

                if total_penalty <= best_score {
                    if semitone_offset < 0
                        && (!accidental.is_sharp() || pitch.name.accidental().is_sharp())
                    {
                        best_pitch = Some(pitch_guess);
                    }
                    if semitone_offset > 0
                        && (!accidental.is_flat() || pitch.name.accidental().is_flat())
                    {
                        best_pitch = Some(pitch_guess);
                    }
                    best_score = total_penalty;
                }
            }
        }

        best_pitch.unwrap_or_else(|| {
            panic!(
                "No valid pitch spelling found transposing {} {}",
                pitch, semitone_offset
            )
        })
    }

    fn name() -> &'static str {
        "ChromaticTransposer"
    }
}

fn interval_penalty(from: &Pitch, to: &Pitch) -> i32 {
    let from_letter = from.name.letter() as i8;
    let to_letter = to.name.letter() as i8;

    let letter_diff = (to_letter - from_letter).rem_euclid(7);
    let semitone_diff = to.midi_number() - from.midi_number();

    match (letter_diff, semitone_diff) {
        (0, 0) => 0,
        (1, 1) | (6, -1) => 0,
        (1, 2) | (6, -2) => 0,
        (2, 3) | (5, -3) => 0,
        (2, 4) | (5, -4) => 0,
        (3, 5) | (4, -5) => 0,
        (4, 6) | (3, -6) => 1,
        (4, 7) | (3, -7) => 0,
        (5, 8) | (2, -8) => 0,
        (5, 9) | (2, -9) => 0,
        (6, 10) | (1, -10) => 0,
        (6, 11) | (1, -11) => 0,
        (0, 12) | (0, -12) => 0,
        (_, 6) | (_, -6) => 1,
        (_, 1 | -1 | 2 | -2) => 2,
        _ => 4,
    }
}

fn expected_letter(from: Letter, semitone_offset: i8) -> Letter {
    let direction = if semitone_offset >= 0 { 1 } else { -1 };
    let steps = match semitone_offset.abs() {
        0 => 0,
        1 | 2 => 1,
        3 | 4 => 2,
        5 => 3,
        6 | 7 => 4,
        8 | 9 => 5,
        10 | 11 => 6,
        _ => 7,
    };

    let mut letter = from;
    for _ in 0..steps {
        letter = if direction > 0 {
            letter.next()
        } else {
            letter.prev()
        };
    }

    letter
}

fn flat_vs_sharp_bias(accidental: Accidental, semitone_offset: i8) -> i32 {
    match (semitone_offset, accidental) {
        (x, Accidental::Flat | Accidental::DoubleFlat) if x < 0 => 0,
        (x, Accidental::Sharp | Accidental::DoubleSharp) if x > 0 => 0,
        (_, Accidental::Natural) => 0,
        _ => 2,
    }
}
