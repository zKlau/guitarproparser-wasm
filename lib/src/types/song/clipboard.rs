// Clipboard structure for Guitar Pro files

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clipboard {
    pub start_measure: i32,
    pub stop_measure: i32,
    pub start_track: i32,
    pub stop_track: i32,
    pub start_beat: i32,
    pub stop_beat: i32,
    pub sub_bar_copy: bool,
}

impl Default for Clipboard {
    fn default() -> Self {
        Clipboard {
            start_measure: 1,
            stop_measure: 1,
            start_track: 1,
            stop_track: 1,
            start_beat: 1,
            stop_beat: 1,
            sub_bar_copy: false,
        }
    }
}
