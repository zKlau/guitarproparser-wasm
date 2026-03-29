// Grace note effect structures
use fraction::ToPrimitive;
use crate::types::enums::GraceEffectTransition;
use super::velocity::DEFAULT_VELOCITY;

use serde::{Serialize, Deserialize};

/// A grace note effect
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraceEffect {
    pub duration: u8,
    pub fret: i8,
    pub is_dead: bool,
    pub is_on_beat: bool,
    pub transition: GraceEffectTransition,
    pub velocity: i16,
}

impl Default for GraceEffect {
    fn default() -> Self {
        GraceEffect {
            duration: 1,
            fret: 0,
            is_dead: false,
            is_on_beat: false,
            transition: GraceEffectTransition::None,
            velocity: DEFAULT_VELOCITY,
        }
    }
}

impl GraceEffect {
    pub(crate) fn _duration_time(self) -> i16 {
        (f32::from(
            crate::model::key_signature::DURATION_QUARTER_TIME
                .to_i16()
                .unwrap(),
        ) / 16f32
            * f32::from(self.duration))
        .to_i16()
        .expect("Cannot get bend point time")
        .to_i16()
        .unwrap()
    }
}
