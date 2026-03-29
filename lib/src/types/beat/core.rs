// Beat structure
use crate::model::key_signature::Duration;
use crate::model::note::Note;
use crate::types::enums::{BeatStatus, Octave};
use super::display::BeatDisplay;
use super::effects::BeatEffects;

use serde::{Serialize, Deserialize};

/// A beat contains multiple notes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Beat {
    pub notes: Vec<Note>,
    pub duration: Duration,
    pub text: String,
    pub start: Option<i64>,
    pub effect: BeatEffects,
    pub octave: Octave,
    pub display: BeatDisplay,
    pub status: BeatStatus,
}

impl Default for Beat {
    fn default() -> Self {
        Beat {
            //voice
            notes: Vec::with_capacity(12),
            duration: Duration::default(),
            text: String::new(),
            start: None,
            effect: BeatEffects::default(),
            octave: Octave::None,
            display: BeatDisplay::default(),
            status: BeatStatus::Normal,
        }
    }
}

impl Beat {
    //pub(crate) fn start_in_measure(&self) -> u16 {self.start - self.voice.measure.start}
    pub(crate) fn has_vibrato(&self) -> bool {
        for i in 0..self.notes.len() {
            if self.notes[i].effect.vibrato {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_harmonic(&self) -> bool {
        for i in 0..self.notes.len() {
            if self.notes[i].effect.is_harmonic() {
                return true;
            }
        }
        false
    }
}
