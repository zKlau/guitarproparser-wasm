// Beat-related enumerations
use crate::error::{GpError, GpResult};
use serde::{Serialize, Deserialize};

/// Beat status enumeration
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeatStatus {
    Empty,
    Normal,
    Rest,
}

pub(crate) fn get_beat_status(value: u8) -> BeatStatus {
    match value {
        0 => BeatStatus::Empty,
        1 => BeatStatus::Normal,
        2 => BeatStatus::Rest,
        _ => BeatStatus::Normal,
    }
}

pub(crate) fn from_beat_status(value: &BeatStatus) -> u8 {
    match value {
        BeatStatus::Empty => 0,
        BeatStatus::Normal => 1,
        BeatStatus::Rest => 2,
    }
}

/// Tuplet bracket enumeration
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TupletBracket {
    None,
    Start,
    End,
}

/// All beat stroke directions
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeatStrokeDirection {
    None,
    Up,
    Down,
}

pub(crate) fn get_beat_stroke_direction(value: i8) -> GpResult<BeatStrokeDirection> {
    match value {
        0 => Ok(BeatStrokeDirection::None),
        1 => Ok(BeatStrokeDirection::Up),
        2 => Ok(BeatStrokeDirection::Down),
        _ => Err(GpError::InvalidValue {
            context: "beat stroke direction",
            value: value as i64,
        }),
    }
}

pub(crate) fn from_beat_stroke_direction(value: &BeatStrokeDirection) -> i8 {
    match value {
        BeatStrokeDirection::None => 0,
        BeatStrokeDirection::Up => 1,
        BeatStrokeDirection::Down => 2,
    }
}

/// Characteristic of articulation
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlapEffect {
    None,
    Tapping,
    Slapping,
    Popping,
}

pub(crate) fn get_slap_effect(value: u8) -> GpResult<SlapEffect> {
    match value {
        0 => Ok(SlapEffect::None),
        1 => Ok(SlapEffect::Tapping),
        2 => Ok(SlapEffect::Slapping),
        3 => Ok(SlapEffect::Popping),
        _ => Err(GpError::InvalidValue {
            context: "slap effect",
            value: value as i64,
        }),
    }
}

pub(crate) fn from_slap_effect(value: &SlapEffect) -> u8 {
    match value {
        SlapEffect::None => 0,
        SlapEffect::Tapping => 1,
        SlapEffect::Slapping => 2,
        SlapEffect::Popping => 3,
    }
}

/// Voice directions indicating the direction of beams
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceDirection {
    None,
    Up,
    Down,
}
