// Chord structure
use crate::types::enums::chord::{ChordType, ChordExtension, ChordAlteration};
use crate::types::enums::note::Fingering;
use super::barre::Barre;
use super::pitch_class::PitchClass;

use serde::{Serialize, Deserialize};

/// A chord annotation for beats
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Chord {
    pub length: u8,
    pub sharp: Option<bool>,
    pub root: Option<PitchClass>,
    pub kind: Option<ChordType>,
    pub extension: Option<ChordExtension>,
    pub bass: Option<PitchClass>,
    pub tonality: Option<ChordAlteration>,
    pub add: Option<bool>,
    pub name: String,
    pub fifth: Option<ChordAlteration>,
    pub ninth: Option<ChordAlteration>,
    pub eleventh: Option<ChordAlteration>,
    pub first_fret: Option<u8>,
    pub strings: Vec<i8>,
    pub barres: Vec<Barre>,
    pub omissions: Vec<bool>,
    pub fingerings: Vec<Fingering>,
    pub show: Option<bool>,
    pub new_format: Option<bool>,
}
