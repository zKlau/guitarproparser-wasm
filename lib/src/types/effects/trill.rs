// Trill effect structures
use fraction::ToPrimitive;
use crate::error::GpResult;
use crate::model::key_signature::{Duration, DURATION_SIXTEENTH, DURATION_SIXTY_FOURTH, DURATION_THIRTY_SECOND};

use serde::{Serialize, Deserialize};

/// A trill effect.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TrillEffect {
    pub fret: i8,
    pub duration: Duration,
}

/// Convert trill period to duration value
pub(crate) fn from_trill_period(period: i8) -> GpResult<u16> {
    match period {
        1 => Ok(DURATION_SIXTEENTH),
        2 => Ok(DURATION_THIRTY_SECOND),
        3 => Ok(DURATION_SIXTY_FOURTH),
        _ => Err(crate::error::GpError::InvalidValue {
            context: "trill period",
            value: period as i64,
        }),
    }
    .map(|v| v.to_u16().unwrap())
}
