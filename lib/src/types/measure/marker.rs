// Marker structure for Guitar Pro measures

use serde::{Serialize, Deserialize};

/// A marker annotation for beats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Marker {
    pub title: String,
    pub color: i32,
}

impl Default for Marker {
    fn default() -> Self {
        Marker {
            title: "Section".to_owned(),
            color: 0xff0000,
        }
    }
}
