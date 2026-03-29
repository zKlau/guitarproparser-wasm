// Beat stroke effect
use crate::types::enums::BeatStrokeDirection;

use serde::{Serialize, Deserialize};

/// A stroke effect for beats.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatStroke {
    pub direction: BeatStrokeDirection,
    pub value: u16,
    pub swap: bool,
}

impl Default for BeatStroke {
    fn default() -> Self {
        BeatStroke {
            direction: BeatStrokeDirection::None,
            value: 0,
            swap: false,
        }
    }
}

impl BeatStroke {
    pub(crate) fn swap_direction(&mut self) {
        if self.direction == BeatStrokeDirection::Up {
            self.direction = BeatStrokeDirection::Down
        } else if self.direction == BeatStrokeDirection::Down {
            self.direction = BeatStrokeDirection::Up
        }
    }
}
