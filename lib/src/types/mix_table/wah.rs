// WahEffect structure
use serde::{Serialize, Deserialize};
use crate::error::{GpError, GpResult};

#[allow(dead_code)]
pub const WAH_EFFECT_OFF: i8 = -2;
pub const WAH_EFFECT_NONE: i8 = -1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WahEffect {
    pub value: i8,
    pub display: bool,
}

impl Default for WahEffect {
    fn default() -> Self {
        WahEffect {
            value: WAH_EFFECT_NONE,
            display: false,
        }
    }
}

impl WahEffect {
    pub(crate) fn _check_value(value: i8) -> GpResult<()> {
        if !(WAH_EFFECT_OFF..=100).contains(&value) {
            return Err(GpError::InvalidRange {
                context: "wah effect",
                value: value as i64,
                min: -2,
                max: 100,
            });
        }
        Ok(())
    }

    pub(crate) fn _is_on(&self) -> bool {
        self.value >= 0 && self.value <= 100
    }

    pub(crate) fn _is_off(&self) -> bool {
        self.value == WAH_EFFECT_OFF
    }

    pub(crate) fn _is_none(&self) -> bool {
        self.value == WAH_EFFECT_NONE
    }
}
