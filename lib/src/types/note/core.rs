// Note structure
use fraction::ToPrimitive;
use serde::{Serialize, Deserialize};

use crate::error::{GpError, GpResult};
use crate::types::effects::DEFAULT_VELOCITY;
use crate::types::enums::NoteType;
use super::effects::NoteEffect;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub value: i16,
    pub velocity: i16,
    pub string: i8,
    pub effect: NoteEffect,
    pub duration_percent: f32,
    pub swap_accidentals: bool,
    pub kind: NoteType,
    pub duration: Option<i8>,
    pub tuplet: Option<i8>,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            value: 0,
            velocity: DEFAULT_VELOCITY,
            string: 0,
            effect: NoteEffect::default(),
            duration_percent: 1.0,
            swap_accidentals: false,
            kind: NoteType::Rest,
            duration: None,
            tuplet: None,
        }
    }
}

impl Note {
    pub(crate) fn real_value(&self, strings: &[(i8, i8)]) -> GpResult<i8> {
        if self.string > 0 {
            let index = (self.string as usize).saturating_sub(1);
            if index < strings.len() {
                return Ok(self.value.to_i8().unwrap_or(0) + strings[index].1);
            }
        }
        Err(GpError::InvalidValue {
            context: "real note value",
            value: self.string as i64,
        })
    }
}
