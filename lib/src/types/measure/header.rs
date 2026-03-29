// MeasureHeader structure for Guitar Pro measures
use fraction::ToPrimitive;

use crate::model::key_signature::{Duration, KeySignature, TimeSignature, DURATION_QUARTER_TIME};
use crate::types::enums::{DirectionSign, TripletFeel};
use super::fermata::MeasureFermata;
use super::marker::Marker;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasureHeader {
    pub number: u16,
    pub start: i64,
    pub time_signature: TimeSignature,
    pub tempo: i32,
    pub marker: Option<Marker>,
    pub repeat_open: bool,
    pub repeat_alternative: u8,
    pub repeat_close: i8,
    pub triplet_feel: TripletFeel,
    pub direction: Option<DirectionSign>,
    /// Tonality of the measure
    pub key_signature: KeySignature,
    pub double_bar: bool,
    /// Fermatas from GPIF (GP6/GP7)
    pub fermatas: Vec<MeasureFermata>,
    /// Free time (no metronome) from GPIF (GP6/GP7)
    pub free_time: bool,
}

impl Default for MeasureHeader {
    fn default() -> Self {
        MeasureHeader {
            number: 1,
            start: DURATION_QUARTER_TIME,
            tempo: 0,
            repeat_open: false,
            repeat_alternative: 0,
            repeat_close: -1,
            triplet_feel: TripletFeel::None,
            direction: None,
            key_signature: KeySignature::default(),
            double_bar: false,
            marker: None,
            time_signature: TimeSignature {
                numerator: 4,
                denominator: Duration::default(),
                beams: vec![2, 2, 2, 2],
            },
            fermatas: Vec::new(),
            free_time: false,
        }
    }
}

impl MeasureHeader {
    #[allow(dead_code)]
    pub(crate) fn length(&self) -> i64 {
        self.time_signature.numerator.to_i64().unwrap()
            * crate::model::key_signature::DURATION_QUARTER_TIME
            * 4
            / self.time_signature.denominator.value.to_i64().unwrap()
    }

    pub(crate) fn _end(&self) -> i64 {
        self.start + self.length()
    }
}

/// This class can store the information about a group of measures which are repeated.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepeatGroup {
    /// List of measure header indexes.
    pub measure_headers: Vec<usize>,
    pub closings: Vec<usize>,
    pub openings: Vec<usize>,
    pub is_closed: bool,
}
