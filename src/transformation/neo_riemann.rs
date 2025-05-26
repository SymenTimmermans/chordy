use crate::{Chord, ChordQuality, Interval};

// Transform to parallel major or minor chord
pub fn transform_p(chord: &Chord) -> Chord {
    match chord.quality {
        ChordQuality::Major => Chord {
            root: chord.root,
            quality: ChordQuality::Minor,
            extensions: vec![],
        },
        ChordQuality::Minor => Chord {
            root: chord.root,
            quality: ChordQuality::Major,
            extensions: vec![],
        },
        _ => chord.clone(),
    }
}

pub fn transform_r(chord: &Chord) -> Chord {
    match chord.quality {
        ChordQuality::Major => Chord {
            root: chord.root - Interval::MINOR_THIRD,
            quality: ChordQuality::Minor,
            extensions: vec![],
        },
        ChordQuality::Minor => Chord {
            root: chord.root + Interval::MINOR_THIRD,
            quality: ChordQuality::Major,
            extensions: vec![],
        },
        _ => chord.clone(),
    }
}
