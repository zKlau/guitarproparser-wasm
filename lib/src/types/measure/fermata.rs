// Fermata structures for Guitar Pro measures

use serde::{Serialize, Deserialize};

/// Type of fermata hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FermataType {
    Short,
    Medium,
    Long,
}

/// A fermata annotation on a measure, with its beat position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeasureFermata {
    pub fermata_type: FermataType,
    /// Beat position as a fraction (numerator, denominator). E.g. (0, 1) for beat 0, (1, 1) for beat 1.
    pub offset: (i32, i32),
}
