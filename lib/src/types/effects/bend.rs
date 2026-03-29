// Bend effect structures and constants
use fraction::ToPrimitive;
use serde::{Serialize, Deserialize};
use crate::types::enums::BendType;

/// A single point within the BendEffect
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BendPoint {
    pub position: u8,
    pub value: i8,
    pub vibrato: bool,
}

impl BendPoint {
    /// Gets the exact time when the point need to be played (MIDI)
    /// * `duration`: the full duration of the effect
    pub(crate) fn _get_time(&self, duration: u8) -> u16 {
        (f32::from(duration) * f32::from(self.position) / f32::from(BEND_EFFECT_MAX_POSITION))
            .to_i16()
            .expect("Cannot get bend point time") as u16
    }
}

pub const BEND_EFFECT_MAX_POSITION: u8 = 12;
pub const GP_BEND_SEMITONE: f32 = 25.0;
pub const GP_BEND_POSITION: f32 = 60.0;
pub const GP_BEND_SEMITONE_LENGTH: f32 = 1.0;

/// This effect is used to describe string bends and tremolo bars
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BendEffect {
    pub kind: BendType,
    pub value: i16,
    pub points: Vec<BendPoint>,
    /// The note offset per bend point offset
    pub semitone_length: u8,
    /// The max position of the bend points (x axis)
    pub max_position: u8,
    /// The max value of the bend points (y axis)
    pub max_value: u8,
}

impl Default for BendEffect {
    fn default() -> Self {
        BendEffect {
            kind: BendType::None,
            value: 0,
            points: Vec::with_capacity(12),
            semitone_length: 1,
            max_position: BEND_EFFECT_MAX_POSITION,
            max_value: 12, /* semi_tone_length * 12 */
        }
    }
}
