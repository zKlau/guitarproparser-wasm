// Harmonic effect structures
use crate::model::chord::PitchClass;
use crate::types::enums::{HarmonicType, Octave};

use serde::{Serialize, Deserialize};

/// A harmonic note effect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarmonicEffect {
    pub kind: HarmonicType,
    //artificial harmonic
    pub pitch: Option<PitchClass>,
    pub octave: Option<Octave>,
    //tapped harmonic
    pub fret: Option<i8>,
}

impl Default for HarmonicEffect {
    fn default() -> Self {
        HarmonicEffect {
            kind: HarmonicType::Natural,
            pitch: None,
            octave: None,
            fret: None,
        }
    }
}
