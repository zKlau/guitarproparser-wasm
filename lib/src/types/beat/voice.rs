// Voice structure containing beats
use super::Beat;
use crate::types::enums::VoiceDirection;
use serde::{Serialize, Deserialize};

/// A voice contains multiple beats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    //pub measure: Measure, //circular depth?
    pub measure_index: i16,
    pub beats: Vec<Beat>,
    pub directions: VoiceDirection,
}

impl Default for Voice {
    fn default() -> Self {
        Voice {
            measure_index: 0,
            /*measure: Measure::default(),*/
            beats: Vec::new(),
            directions: VoiceDirection::None,
        }
    }
}
