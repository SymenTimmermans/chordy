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

    // Altered scale degrees - sharps
    /// Sharp tonic (♯1)
    pub const SHARP_TONIC: Self = Self::new(1, Some(Accidental::Sharp));
    /// Sharp supertonic (♯2)  
    pub const SHARP_SUPERTONIC: Self = Self::new(2, Some(Accidental::Sharp));
    /// Sharp mediant (♯3)
    pub const SHARP_MEDIANT: Self = Self::new(3, Some(Accidental::Sharp));
    /// Sharp subdominant (♯4)
    pub const SHARP_FOURTH: Self = Self::new(4, Some(Accidental::Sharp));
    /// Sharp subdominant alias
    pub const SHARP_SUBDOMINANT: Self = Self::SHARP_FOURTH;
    /// Sharp dominant (♯5)
    pub const SHARP_DOMINANT: Self = Self::new(5, Some(Accidental::Sharp));
    /// Sharp submediant (♯6)
    pub const SHARP_SUBMEDIANT: Self = Self::new(6, Some(Accidental::Sharp));
    /// Sharp leading tone (♯7)
    pub const SHARP_LEADING_TONE: Self = Self::new(7, Some(Accidental::Sharp));

    // Altered scale degrees - flats
    /// Flat tonic (♭1)
    pub const FLAT_TONIC: Self = Self::new(1, Some(Accidental::Flat));
    /// Flat supertonic (♭2)
    pub const FLAT_SECOND: Self = Self::new(2, Some(Accidental::Flat));
    /// Flat supertonic alias
    pub const FLAT_SUPERTONIC: Self = Self::FLAT_SECOND;
    /// Flat mediant (♭3)
    pub const FLAT_THIRD: Self = Self::new(3, Some(Accidental::Flat));
    /// Flat mediant alias
    pub const FLAT_MEDIANT: Self = Self::FLAT_THIRD;
    /// Flat subdominant (♭4)
    pub const FLAT_SUBDOMINANT: Self = Self::new(4, Some(Accidental::Flat));
    /// Flat dominant (♭5)
    pub const FLAT_DOMINANT: Self = Self::new(5, Some(Accidental::Flat));
    /// Flat submediant (♭6)
    pub const FLAT_SIXTH: Self = Self::new(6, Some(Accidental::Flat));
    /// Flat submediant alias
    pub const FLAT_SUBMEDIANT: Self = Self::FLAT_SIXTH;
    /// Flat subtonic (♭7)
    pub const FLAT_SEVENTH: Self = Self::new(7, Some(Accidental::Flat));
    /// Flat subtonic alias
    pub const FLAT_SUBTONIC: Self = Self::FLAT_SEVENTH;

    // Special scale degrees with traditional names
    /// The Neapolitan flat second scale degree (♭II)
    pub const NEAPOLITAN: Self = Self::FLAT_SECOND;
}

impl From<Interval> for ScaleDegree {
    fn from(interval: Interval) -> Self {
        match interval {
            // Unisons
            Interval::DOUBLY_DIMINISHED_UNISON => Self::new(1, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_UNISON => Self::new(1, Some(Accidental::Flat)),
            Interval::PERFECT_UNISON => Self::TONIC,
            Interval::AUGMENTED_UNISON => Self::new(1, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_UNISON => Self::new(1, Some(Accidental::DoubleSharp)),

            // Seconds  
            Interval::DOUBLY_DIMINISHED_SECOND => Self::new(2, Some(Accidental::DoubleFlat)),  // ♭♭2 = 2 with double flat
            Interval::DIMINISHED_SECOND => Self::new(2, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SECOND => Self::new(2, Some(Accidental::Flat)),
            Interval::MAJOR_SECOND => Self::new(2, None),
            Interval::AUGMENTED_SECOND => Self::new(2, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_SECOND => Self::new(2, Some(Accidental::DoubleSharp)),

            // Thirds
            Interval::DOUBLY_DIMINISHED_THIRD => Self::new(3, Some(Accidental::DoubleFlat)),  // ♭♭3
            Interval::DIMINISHED_THIRD => Self::new(3, Some(Accidental::DoubleFlat)),
            Interval::MINOR_THIRD => Self::new(3, Some(Accidental::Flat)),
            Interval::MAJOR_THIRD => Self::new(3, None),
            Interval::AUGMENTED_THIRD => Self::new(3, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_THIRD => Self::new(3, Some(Accidental::DoubleSharp)),

            // Fourths
            Interval::DOUBLY_DIMINISHED_FOURTH => Self::new(4, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_FOURTH => Self::new(4, Some(Accidental::Flat)),
            Interval::PERFECT_FOURTH => Self::new(4, None),
            Interval::AUGMENTED_FOURTH => Self::new(4, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_FOURTH => Self::new(4, Some(Accidental::DoubleSharp)),

            // Fifths
            Interval::DOUBLY_DIMINISHED_FIFTH => Self::new(5, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_FIFTH => Self::new(5, Some(Accidental::Flat)),
            Interval::PERFECT_FIFTH => Self::new(5, None),
            Interval::AUGMENTED_FIFTH => Self::new(5, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_FIFTH => Self::new(5, Some(Accidental::DoubleSharp)),

            // Sixths
            Interval::DOUBLY_DIMINISHED_SIXTH => Self::new(6, Some(Accidental::DoubleFlat)),  // ♭♭6
            Interval::DIMINISHED_SIXTH => Self::new(6, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SIXTH => Self::new(6, Some(Accidental::Flat)),
            Interval::MAJOR_SIXTH => Self::new(6, None),
            Interval::AUGMENTED_SIXTH => Self::new(6, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_SIXTH => Self::new(6, Some(Accidental::DoubleSharp)),

            // Sevenths
            Interval::DOUBLY_DIMINISHED_SEVENTH => Self::new(7, Some(Accidental::DoubleFlat)),  // ♭♭7
            Interval::DIMINISHED_SEVENTH => Self::new(7, Some(Accidental::DoubleFlat)),
            Interval::MINOR_SEVENTH => Self::new(7, Some(Accidental::Flat)),
            Interval::MAJOR_SEVENTH => Self::new(7, None),
            Interval::AUGMENTED_SEVENTH => Self::new(7, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_SEVENTH => Self::new(7, Some(Accidental::DoubleSharp)),

            // Octaves (wrap back to 1)
            Interval::DOUBLY_DIMINISHED_OCTAVE => Self::new(1, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_OCTAVE => Self::new(1, Some(Accidental::Flat)),
            Interval::OCTAVE => Self::TONIC,
            Interval::AUGMENTED_OCTAVE => Self::new(1, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_OCTAVE => Self::new(1, Some(Accidental::DoubleSharp)),

            // Compound intervals (reduce to simple intervals)
            // Ninths -> Seconds
            Interval::DOUBLY_DIMINISHED_NINTH => Self::new(2, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_NINTH => Self::new(2, Some(Accidental::DoubleFlat)),
            Interval::MINOR_NINTH => Self::new(2, Some(Accidental::Flat)),
            Interval::MAJOR_NINTH => Self::new(2, None),
            Interval::AUGMENTED_NINTH => Self::new(2, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_NINTH => Self::new(2, Some(Accidental::DoubleSharp)),

            // Tenths -> Thirds  
            Interval::DOUBLY_DIMINISHED_TENTH => Self::new(3, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_TENTH => Self::new(3, Some(Accidental::DoubleFlat)),
            Interval::MINOR_TENTH => Self::new(3, Some(Accidental::Flat)),
            Interval::MAJOR_TENTH => Self::new(3, None),
            Interval::AUGMENTED_TENTH => Self::new(3, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_TENTH => Self::new(3, Some(Accidental::DoubleSharp)),

            // Elevenths -> Fourths
            Interval::DOUBLY_DIMINISHED_ELEVENTH => Self::new(4, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_ELEVENTH => Self::new(4, Some(Accidental::Flat)),
            Interval::PERFECT_ELEVENTH => Self::new(4, None),
            Interval::AUGMENTED_ELEVENTH => Self::new(4, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_ELEVENTH => Self::new(4, Some(Accidental::DoubleSharp)),

            // Twelfths -> Fifths
            Interval::DOUBLY_DIMINISHED_TWELFTH => Self::new(5, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_TWELFTH => Self::new(5, Some(Accidental::Flat)),
            Interval::PERFECT_TWELFTH => Self::new(5, None),
            Interval::AUGMENTED_TWELFTH => Self::new(5, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_TWELFTH => Self::new(5, Some(Accidental::DoubleSharp)),

            // Thirteenths -> Sixths
            Interval::DOUBLY_DIMINISHED_THIRTEENTH => Self::new(6, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_THIRTEENTH => Self::new(6, Some(Accidental::DoubleFlat)),
            Interval::MINOR_THIRTEENTH => Self::new(6, Some(Accidental::Flat)),
            Interval::MAJOR_THIRTEENTH => Self::new(6, None),
            Interval::AUGMENTED_THIRTEENTH => Self::new(6, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_THIRTEENTH => Self::new(6, Some(Accidental::DoubleSharp)),

            // Fourteenths -> Sevenths
            Interval::DOUBLY_DIMINISHED_FOURTEENTH => Self::new(7, Some(Accidental::DoubleFlat)),
            Interval::DIMINISHED_FOURTEENTH => Self::new(7, Some(Accidental::DoubleFlat)),
            Interval::MINOR_FOURTEENTH => Self::new(7, Some(Accidental::Flat)),
            Interval::MAJOR_FOURTEENTH => Self::new(7, None),
            Interval::AUGMENTED_FOURTEENTH => Self::new(7, Some(Accidental::Sharp)),
            Interval::DOUBLY_AUGMENTED_FOURTEENTH => Self::new(7, Some(Accidental::DoubleSharp)),

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

impl ScaleDegree {
    /// Convert this scale degree to its corresponding interval from the tonic
    ///
    /// Maps scale degrees to intervals considering both the step (1-7) and any accidental.
    /// This is the reverse operation of `From<Interval> for ScaleDegree`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordy::{ScaleDegree, Interval, Accidental};
    ///
    /// let major_third = ScaleDegree::new(3, None);
    /// assert_eq!(major_third.to_interval(), Interval::MAJOR_THIRD);
    ///
    /// let minor_third = ScaleDegree::new(3, Some(Accidental::Flat));
    /// assert_eq!(minor_third.to_interval(), Interval::MINOR_THIRD);
    ///
    /// let augmented_fourth = ScaleDegree::new(4, Some(Accidental::Sharp));
    /// assert_eq!(augmented_fourth.to_interval(), Interval::AUGMENTED_FOURTH);
    /// ```
    pub fn to_interval(&self) -> Interval {
        // Get the base interval for the scale degree
        let base_interval = match self.step {
            1 => Interval::PERFECT_UNISON,
            2 => Interval::MAJOR_SECOND,
            3 => Interval::MAJOR_THIRD,
            4 => Interval::PERFECT_FOURTH,
            5 => Interval::PERFECT_FIFTH,
            6 => Interval::MAJOR_SIXTH,
            7 => Interval::MAJOR_SEVENTH,
            _ => panic!("Invalid scale degree step: {}", self.step),
        };

        // Apply accidental modifications
        match self.alteration {
            None | Some(Accidental::Natural) => base_interval,
            Some(Accidental::Flat) => match base_interval {
                Interval::MAJOR_SECOND => Interval::MINOR_SECOND,
                Interval::MAJOR_THIRD => Interval::MINOR_THIRD,
                Interval::PERFECT_FOURTH => Interval::DIMINISHED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::DIMINISHED_FIFTH,
                Interval::MAJOR_SIXTH => Interval::MINOR_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::MINOR_SEVENTH,
                Interval::PERFECT_UNISON => Interval::DIMINISHED_UNISON,
                _ => base_interval, // Shouldn't happen with valid scale degrees
            },
            Some(Accidental::Sharp) => match base_interval {
                Interval::PERFECT_UNISON => Interval::AUGMENTED_UNISON,
                Interval::MAJOR_SECOND => Interval::AUGMENTED_SECOND,
                Interval::MAJOR_THIRD => Interval::AUGMENTED_THIRD,
                Interval::PERFECT_FOURTH => Interval::AUGMENTED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::AUGMENTED_FIFTH,
                Interval::MAJOR_SIXTH => Interval::AUGMENTED_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::AUGMENTED_SEVENTH,
                _ => base_interval, // Shouldn't happen with valid scale degrees
            },
            Some(Accidental::DoubleFlat) => match base_interval {
                Interval::MAJOR_SECOND => Interval::DIMINISHED_SECOND,
                Interval::MAJOR_THIRD => Interval::DIMINISHED_THIRD,
                Interval::MAJOR_SIXTH => Interval::DIMINISHED_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::DIMINISHED_SEVENTH,
                // Perfect intervals become doubly diminished when double-flatted
                Interval::PERFECT_UNISON => Interval::DOUBLY_DIMINISHED_UNISON,
                Interval::PERFECT_FOURTH => Interval::DOUBLY_DIMINISHED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::DOUBLY_DIMINISHED_FIFTH,
                _ => base_interval,
            },
            Some(Accidental::DoubleSharp) => match base_interval {
                // Major intervals become doubly augmented when double-sharped
                Interval::MAJOR_SECOND => Interval::DOUBLY_AUGMENTED_SECOND,
                Interval::MAJOR_THIRD => Interval::DOUBLY_AUGMENTED_THIRD,
                Interval::MAJOR_SIXTH => Interval::DOUBLY_AUGMENTED_SIXTH,
                Interval::MAJOR_SEVENTH => Interval::DOUBLY_AUGMENTED_SEVENTH,
                // Perfect intervals become doubly augmented when double-sharped
                Interval::PERFECT_UNISON => Interval::DOUBLY_AUGMENTED_UNISON,
                Interval::PERFECT_FOURTH => Interval::DOUBLY_AUGMENTED_FOURTH,
                Interval::PERFECT_FIFTH => Interval::DOUBLY_AUGMENTED_FIFTH,
                _ => base_interval,
            },
        }
    }
}

impl From<u8> for ScaleDegree {
    /// Create a natural scale degree from a step number (1-7)
    /// 
    /// # Panics
    /// Panics if the number is not in the range 1-7
    fn from(step: u8) -> Self {
        if !(1..=7).contains(&step) {
            panic!("Scale degree step must be in range 1-7, got {}", step);
        }
        ScaleDegree::new(step, None)
    }
}