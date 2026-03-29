// Barre structure

use serde::{Serialize, Deserialize};

/// A single barre
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Barre {
    pub fret: i8,
    /// First string from the bottom of the barre
    pub start: i8,
    /// Last string on the top of the barre
    pub end: i8,
}
