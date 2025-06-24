use crate::error::TypeError;
use crate::types::{Accidental, Interval};

/// A scale degree represents a specific step in a scale, optionally with an alteration
/// (accidental).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScaleDegree {
    /// The step in the scale (1-7)
    pub step: u8,
    /// The alteration of the scale degree, if any
    pub alteration: Option<Accidental>,
}

impl ScaleDegree {
    /// Create a new scale degree with the given step and optional alteration
    pub const fn new(step: u8, alteration: Option<Accidental>) -> Self {
        // Note: const fn can't use assert! before Rust 1.57
        // Use runtime validation in a separate function if needed
        ScaleDegree { step, alteration }
    }

    /// Validate the scale degree
    pub fn validate(&self) -> Result<(), TypeError> {
        if self.step < 1 || self.step > 7 {
            return Err(TypeError::InvalidScaleDegree(self.step));
        }
        Ok(())
    }

    /// The tonic (1st scale degree)
    pub const TONIC: Self = Self::new(1, None);
    /// The supertonic (2nd scale degree)
    pub const SUPERTONIC: Self = Self::new(2, None);
    /// The mediant (3rd scale degree)
    pub const MEDIANT: Self = Self::new(3, None);
    /// The subdominant (4th scale degree)
    pub const SUBDOMINANT: Self = Self::new(4, None);
    /// The dominant (5th scale degree)
    pub const DOMINANT: Self = Self::new(5, None);
    /// The submediant (6th scale degree)
    pub const SUBMEDIANT: Self = Self::new(6, None);
    /// The leading tone (7th scale degree in major scales)
    pub const LEADING_TONE: Self = Self::new(7, None);
    /// The subtonic, unaltered 7th scale degree in minor scales
    pub const SUBTONIC: Self = Self::new(7, None);

    // Altered scale degrees
    #[allow(missing_docs)]
    pub const FLAT_SECOND: Self = Self::new(2, Some(Accidental::Flat));
    #[allow(missing_docs)]
    pub const FLAT_THIRD: Self = Self::new(3, Some(Accidental::Flat));
    #[allow(missing_docs)]
    pub const SHARP_FOURTH: Self = Self::new(4, Some(Accidental::Sharp));
    #[allow(missing_docs)]
    pub const FLAT_SIXTH: Self = Self::new(6, Some(Accidental::Flat));
    #[allow(missing_docs)]
    pub const FLAT_SEVENTH: Self = Self::new(7, Some(Accidental::Flat));

    // Special scale degrees with traditional names
    /// The Neapolitan flat second scale degree (♭II)
    pub const NEAPOLITAN: Self = Self::new(2, Some(Accidental::Flat)); // ♭II
}

impl From<Interval> for ScaleDegree {
    fn from(interval: Interval) -> Self {
        match interval {
            // Unisons
            Interval::DIMINISHED_UNISON => Self::new(1, Some(Accidental::Flat)),
            Interval::PERFECT_UNISON => Self::TONIC,
            Interval::AUGMENTED_UNISON => Self::new(1, Some(Accidental::Sharp)),

            // Seconds  
            Interval::DIMINISHED_SECOND => Self::new(2, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SECOND => Self::new(2, Some(Accidental::Flat)),
            Interval::MAJOR_SECOND => Self::new(2, None),
            Interval::AUGMENTED_SECOND => Self::new(2, Some(Accidental::Sharp)),

            // Thirds
            Interval::DIMINISHED_THIRD => Self::new(3, Some(Accidental::DoubleFlat)),
            Interval::MINOR_THIRD => Self::new(3, Some(Accidental::Flat)),
            Interval::MAJOR_THIRD => Self::new(3, None),
            Interval::AUGMENTED_THIRD => Self::new(3, Some(Accidental::Sharp)),

            // Fourths
            Interval::DIMINISHED_FOURTH => Self::new(4, Some(Accidental::Flat)),
            Interval::PERFECT_FOURTH => Self::new(4, None),
            Interval::AUGMENTED_FOURTH => Self::new(4, Some(Accidental::Sharp)),

            // Fifths
            Interval::DIMINISHED_FIFTH => Self::new(5, Some(Accidental::Flat)),
            Interval::PERFECT_FIFTH => Self::new(5, None),
            Interval::AUGMENTED_FIFTH => Self::new(5, Some(Accidental::Sharp)),

            // Sixths
            Interval::DIMINISHED_SIXTH => Self::new(6, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SIXTH => Self::new(6, Some(Accidental::Flat)),
            Interval::MAJOR_SIXTH => Self::new(6, None),
            Interval::AUGMENTED_SIXTH => Self::new(6, Some(Accidental::Sharp)),

            // Sevenths
            Interval::DIMINISHED_SEVENTH => Self::new(7, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SEVENTH => Self::new(7, Some(Accidental::Flat)),
            Interval::MAJOR_SEVENTH => Self::new(7, None),
            Interval::AUGMENTED_SEVENTH => Self::new(7, Some(Accidental::Sharp)),

            // Octaves (wrap back to 1)
            Interval::DIMINISHED_OCTAVE => Self::new(1, Some(Accidental::Flat)),
            Interval::OCTAVE => Self::TONIC,
            Interval::AUGMENTED_OCTAVE => Self::new(1, Some(Accidental::Sharp)),

            // Compound intervals (reduce to simple intervals)
            // Ninths -> Seconds
            Interval::DIMINISHED_NINTH => Self::new(2, Some(Accidental::DoubleFlat)),
            Interval::MINOR_NINTH => Self::new(2, Some(Accidental::Flat)),
            Interval::MAJOR_NINTH => Self::new(2, None),
            Interval::AUGMENTED_NINTH => Self::new(2, Some(Accidental::Sharp)),

            // Tenths -> Thirds  
            Interval::DIMINISHED_TENTH => Self::new(3, Some(Accidental::DoubleFlat)),
            Interval::MINOR_TENTH => Self::new(3, Some(Accidental::Flat)),
            Interval::MAJOR_TENTH => Self::new(3, None),
            Interval::AUGMENTED_TENTH => Self::new(3, Some(Accidental::Sharp)),

            // Elevenths -> Fourths
            Interval::DIMINISHED_ELEVENTH => Self::new(4, Some(Accidental::Flat)),
            Interval::PERFECT_ELEVENTH => Self::new(4, None),
            Interval::AUGMENTED_ELEVENTH => Self::new(4, Some(Accidental::Sharp)),

            // Twelfths -> Fifths
            Interval::DIMINISHED_TWELFTH => Self::new(5, Some(Accidental::Flat)),
            Interval::PERFECT_TWELFTH => Self::new(5, None),
            Interval::AUGMENTED_TWELFTH => Self::new(5, Some(Accidental::Sharp)),

            // Thirteenths -> Sixths
            Interval::DIMINISHED_THIRTEENTH => Self::new(6, Some(Accidental::DoubleFlat)),
            Interval::MINOR_THIRTEENTH => Self::new(6, Some(Accidental::Flat)),
            Interval::MAJOR_THIRTEENTH => Self::new(6, None),
            Interval::AUGMENTED_THIRTEENTH => Self::new(6, Some(Accidental::Sharp)),

            // Fourteenths -> Sevenths
            Interval::DIMINISHED_FOURTEENTH => Self::new(7, Some(Accidental::DoubleFlat)),
            Interval::MINOR_FOURTEENTH => Self::new(7, Some(Accidental::Flat)),
            Interval::MAJOR_FOURTEENTH => Self::new(7, None),
            Interval::AUGMENTED_FOURTEENTH => Self::new(7, Some(Accidental::Sharp)),

            // For any other intervals, calculate the scale degree from the fifths position
            _ => {
                // Calculate generic interval from fifths using the same formula as the private method
                let base_generic = ((interval.fifths * 4) % 7 + 7) % 7;
                let octave_generics = interval.octaves * 7;
                let generic_num = base_generic + octave_generics + 1;  // +1 because intervals start at 1
                let reduced_degree = ((generic_num - 1) % 7) + 1;
                Self::new(reduced_degree as u8, None)
            }
        }
    }
}