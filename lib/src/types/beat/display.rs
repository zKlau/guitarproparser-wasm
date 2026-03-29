// Beat display parameters
use crate::types::enums::{TupletBracket, VoiceDirection};

use serde::{Serialize, Deserialize};

/// Parameters of beat display
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatDisplay {
    pub(crate) break_beam: bool,
    pub(crate) force_beam: bool,
    pub(crate) beam_direction: VoiceDirection,
    pub(crate) tuplet_bracket: TupletBracket,
    pub(crate) break_secondary: u8,
    pub(crate) break_secondary_tuplet: bool,
    pub(crate) force_bracket: bool,
}

impl Default for BeatDisplay {
    fn default() -> Self {
        BeatDisplay {
            break_beam: false,
            force_beam: false,
            beam_direction: VoiceDirection::None,
            tuplet_bracket: TupletBracket::None,
            break_secondary: 0,
            break_secondary_tuplet: false,
            force_bracket: false,
        }
    }
}
