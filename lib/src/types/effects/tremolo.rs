// Tremolo picking effect structures
use crate::error::GpResult;
use crate::model::key_signature::{Duration, DURATION_EIGHTH, DURATION_SIXTEENTH, DURATION_THIRTY_SECOND};

use serde::{Serialize, Deserialize};

/// A tremolo picking effect.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TremoloPickingEffect {
    pub duration: Duration,
}

/// Convert tremolo picking speed to actual duration. Values are:
/// - *1*: eighth
/// - *2*: sixteenth
/// - *3*: thirtySecond
pub(crate) fn from_tremolo_value(value: i8) -> GpResult<u8> {
    match value {
        1 => Ok(DURATION_EIGHTH),
        3 => Ok(DURATION_SIXTEENTH),
        2 => Ok(DURATION_THIRTY_SECOND),
        _ => Err(crate::error::GpError::InvalidValue {
            context: "tremolo picking value",
            value: value as i64,
        }),
    }
}
