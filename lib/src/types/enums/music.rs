// Music-related enumerations
use crate::error::{GpError, GpResult};
use serde::{Serialize, Deserialize};

/// Octave signs
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Octave {
    None,
    Ottava,
    Quindicesima,
    OttavaBassa,
    QuindicesimaBassa,
}

pub(crate) fn get_octave(value: u8) -> GpResult<Octave> {
    match value {
        0 => Ok(Octave::None),
        1 => Ok(Octave::Ottava),
        2 => Ok(Octave::Quindicesima),
        3 => Ok(Octave::OttavaBassa),
        4 => Ok(Octave::QuindicesimaBassa),
        _ => Err(GpError::InvalidValue {
            context: "octave",
            value: value as i64,
        }),
    }
}

pub(crate) fn from_octave(value: &Octave) -> u8 {
    match value {
        Octave::None => 0,
        Octave::Ottava => 1,
        Octave::Quindicesima => 2,
        Octave::OttavaBassa => 3,
        Octave::QuindicesimaBassa => 4,
    }
}

/// Values of auto-accentuation on the beat found in track RSE settings
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Accentuation {
    None,
    VerySoft,
    Soft,
    Medium,
    Strong,
    VeryStrong,
}

pub(crate) fn get_accentuation(value: u8) -> GpResult<Accentuation> {
    match value {
        0 => Ok(Accentuation::None),
        1 => Ok(Accentuation::VerySoft),
        2 => Ok(Accentuation::Soft),
        3 => Ok(Accentuation::Medium),
        4 => Ok(Accentuation::Strong),
        5 => Ok(Accentuation::VeryStrong),
        _ => Err(GpError::InvalidValue {
            context: "accentuation",
            value: value as i64,
        }),
    }
}

pub(crate) fn from_accentuation(value: &Accentuation) -> u8 {
    match value {
        Accentuation::None => 0,
        Accentuation::VerySoft => 1,
        Accentuation::Soft => 2,
        Accentuation::Medium => 3,
        Accentuation::Strong => 4,
        Accentuation::VeryStrong => 5,
    }
}
