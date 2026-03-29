// Beat effects structures
use crate::model::chord::Chord;
use crate::model::mix_table::MixTableChange;
use crate::types::effects::BendEffect;
use crate::types::enums::{BeatStrokeDirection, SlapEffect};
use super::stroke::BeatStroke;

use serde::{Serialize, Deserialize};

/// This class contains all beat effects
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeatEffects {
    pub stroke: BeatStroke,
    pub has_rasgueado: bool,
    pub pick_stroke: BeatStrokeDirection,
    pub chord: Option<Chord>,
    pub fade_in: bool,
    pub tremolo_bar: Option<BendEffect>,
    pub mix_table_change: Option<MixTableChange>,
    pub slap_effect: SlapEffect,
    pub vibrato: bool,
}

impl Default for BeatEffects {
    fn default() -> Self {
        BeatEffects {
            stroke: BeatStroke::default(),
            has_rasgueado: false,
            pick_stroke: BeatStrokeDirection::None,
            chord: None,
            fade_in: false,
            tremolo_bar: None,
            mix_table_change: None,
            slap_effect: SlapEffect::None,
            vibrato: false,
        }
    }
}

impl BeatEffects {
    pub(crate) fn is_chord(&self) -> bool {
        self.chord.is_some()
    }

    pub(crate) fn is_tremolo_bar(&self) -> bool {
        self.tremolo_bar.is_some()
    }

    pub(crate) fn is_slap_effect(&self) -> bool {
        self.slap_effect != SlapEffect::None
    }

    pub(crate) fn has_pick_stroke(&self) -> bool {
        self.pick_stroke != BeatStrokeDirection::None
    }

    pub(crate) fn is_default(&self) -> bool {
        let d = BeatEffects::default();
        self.stroke == d.stroke
            && self.has_rasgueado == d.has_rasgueado
            && self.pick_stroke == d.pick_stroke
            && self.fade_in == d.fade_in
            && self.vibrato == d.vibrato
            && self.tremolo_bar == d.tremolo_bar
            && self.slap_effect == d.slap_effect
    }
}
