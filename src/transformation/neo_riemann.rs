use crate::{Chord, Interval, NoteName};

// Transform to parallel major or minor chord
pub fn transform_p(chord: &Chord) -> Chord {
    // The axis is the root and the fifth
    reflect_across_axis(chord, chord.root, chord.root + Interval::PERFECT_FIFTH)
}

pub fn transform_r(chord: &Chord) -> Chord {
    if chord.is_major() {
        // if the chord is major, the axis is the root and the major third
        reflect_across_axis(chord, chord.root, chord.root + Interval::MAJOR_THIRD)
    } else {
        // if the chord is minor, the axis is the the minor third and the fifth
        reflect_across_axis(chord, chord.root + Interval::MINOR_THIRD, chord.root + Interval::PERFECT_FIFTH)
    }
}

fn reflect_across_axis(chord: &Chord, axis_note1: NoteName, axis_note2: NoteName) -> Chord {
    // The axis is the line between axis_note1 and axis_note2
    // For each note not on the axis, reflect it across this line

    let notes: Vec<NoteName> = chord
        .notes()
        .iter()
        .map(|&note| {
            if note == axis_note1 || note == axis_note2 {
                println!("Note {:?} is on the axis, not reflecting", note);
                note // Notes on the axis stay fixed
            } else {

                // Calculate reflection of note across the axis
                let new_note = reflect_point_across_line(note, axis_note1, axis_note2);
                println!("Reflecting note {:?} across axis {:?} - {:?} ==> {:?}", note, axis_note1, axis_note2, new_note);
                new_note
            }
        })
        .collect();
    println!("Reflected notes: {:?}", notes);

    Chord::from_notes(&notes)
}

fn reflect_point_across_line(
    note: NoteName,
    axis_note1: NoteName,
    axis_note2: NoteName,
) -> NoteName {
    // In the Tonnetz, reflections work differently because of the triangular lattice
    // The three types of reflections correspond to P, L, and R transformations

    let note_fifths = note.fifths() as f32;
    let axis1_fifths = axis_note1.fifths() as f32;
    let axis2_fifths = axis_note2.fifths() as f32;

    // Calculate the interval between the axis notes
    let axis_interval = axis2_fifths - axis1_fifths;
    println!(
        "Reflecting note {:?} across axis {:?} - {:?} with interval {}",
        note, axis2_fifths, axis1_fifths, axis_interval
    );

    match axis_interval.abs() as i8 {
        1 => {
            // Perfect fifth interval - this is a P-type reflection
            // The axis is the perfect fifth edge of the triangle
            let midpoint = (axis1_fifths + axis2_fifths) / 2f32;
            let reflection = 2f32 * midpoint - note_fifths;
            println!(
                "Perfect fifth reflection: midpoint = {}, reflection = {}",
                midpoint, reflection
            );
            NoteName::from_fifths(reflection as i8)
        }
        4 => {
            // Major third interval - this is an R-type reflection
            // The axis is the major third edge of the triangle
            let midpoint = (axis1_fifths + axis2_fifths) / 2f32;
            let reflection = 2f32 * midpoint - note_fifths;
            NoteName::from_fifths(reflection as i8)
        }
        3 => {
            // Minor third interval - this is an L-type reflection
            // The axis is the minor third edge of the triangle
            let midpoint = (axis1_fifths + axis2_fifths) / 2f32;
            let reflection = 2f32 * midpoint - note_fifths;
            NoteName::from_fifths(reflection as i8)
        }
        _ => {
            // For other intervals, use general geometric reflection
            // This handles extended chords and non-standard transformations
            general_tonnetz_reflection(note, axis_note1, axis_note2)
        }
    }
}

fn general_tonnetz_reflection(
    note: NoteName,
    axis_note1: NoteName,
    axis_note2: NoteName,
) -> NoteName {
    // For non-standard axes, we need to work in the actual Tonnetz coordinate system
    // Convert fifth-space to Tonnetz coordinates (x = fifths, y = major thirds)

    let note_x = note.fifths() as f64;
    let note_y = (note.fifths() * 4) as f64 / 7.0; // Approximate major third coordinate

    let axis1_x = axis_note1.fifths() as f64;
    let axis1_y = (axis_note1.fifths() * 4) as f64 / 7.0;

    let axis2_x = axis_note2.fifths() as f64;
    let axis2_y = (axis_note2.fifths() * 4) as f64 / 7.0;

    // Standard 2D line reflection formula
    let line_dx = axis2_x - axis1_x;
    let line_dy = axis2_y - axis1_y;

    if line_dx.abs() < 1e-10 && line_dy.abs() < 1e-10 {
        return note; // Degenerate case
    }

    let line_length_sq = line_dx * line_dx + line_dy * line_dy;

    let point_to_axis1_x = note_x - axis1_x;
    let point_to_axis1_y = note_y - axis1_y;

    let projection_scalar =
        (point_to_axis1_x * line_dx + point_to_axis1_y * line_dy) / line_length_sq;

    let closest_x = axis1_x + projection_scalar * line_dx;
    let closest_y = axis1_y + projection_scalar * line_dy;

    let reflected_x: f64 = 2.0 * closest_x - note_x;
    let _reflected_y = 2.0 * closest_y - note_y;

    // Convert back to fifth-space (this is approximate)
    let reflected_fifths = reflected_x.round() as i8;

    NoteName::from_fifths(reflected_fifths)
}
