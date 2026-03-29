// Note effects structure
use serde::{Serialize, Deserialize};
use crate::types::effects::{BendEffect, GraceEffect, HarmonicEffect, TremoloPickingEffect, TrillEffect};
use crate::types::enums::{Fingering, SlideType};

/// Contains all effects which can be applied to one note.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteEffect {
    pub accentuated_note: bool,
    pub bend: Option<BendEffect>,
    pub ghost_note: bool,
    pub grace: Option<GraceEffect>,
    pub hammer: bool,
    pub harmonic: Option<HarmonicEffect>,
    pub heavy_accentuated_note: bool,
    pub left_hand_finger: Fingering,
    pub let_ring: bool,
    pub palm_mute: bool,
    pub right_hand_finger: Fingering,
    pub slides: Vec<SlideType>,
    pub staccato: bool,
    pub tremolo_picking: Option<TremoloPickingEffect>,
    pub trill: Option<TrillEffect>,
    pub vibrato: bool,
    /// Ornament type from GPIF (GP6/GP7)
    pub ornament: Option<String>,
}

impl Default for NoteEffect {
    fn default() -> Self {
        NoteEffect {
            accentuated_note: false,
            bend: None,
            ghost_note: false,
            grace: None,
            hammer: false,
            harmonic: None,
            heavy_accentuated_note: false,
            left_hand_finger: Fingering::Open,
            let_ring: false,
            palm_mute: false,
            right_hand_finger: Fingering::Open,
            slides: Vec::new(),
            staccato: false,
            tremolo_picking: None,
            trill: None,
            vibrato: false,
            ornament: None,
        }
    }
}

impl NoteEffect {
    pub(crate) fn is_bend(&self) -> bool {
        self.bend.is_some()
    }

    pub(crate) fn is_harmonic(&self) -> bool {
        self.harmonic.is_some()
    }

    pub(crate) fn is_grace(&self) -> bool {
        self.grace.is_some()
    }

    pub(crate) fn is_trill(&self) -> bool {
        self.trill.is_some()
    }

    pub(crate) fn is_tremollo_picking(&self) -> bool {
        self.tremolo_picking.is_some()
    }

    pub(crate) fn is_default(&self) -> bool {
        let d = NoteEffect::default();
        self.left_hand_finger == d.left_hand_finger
            && self.right_hand_finger == d.right_hand_finger
            && self.bend == d.bend
            && self.harmonic == d.harmonic
            && self.grace == d.grace
            && self.trill == d.trill
            && self.tremolo_picking == d.tremolo_picking
            && self.vibrato == d.vibrato
            && self.slides == d.slides
            && self.hammer == d.hammer
            && self.palm_mute == d.palm_mute
            && self.staccato == d.staccato
            && self.let_ring == d.let_ring
    }

    pub(crate) fn is_fingering(&self) -> bool {
        self.left_hand_finger != Fingering::Open || self.right_hand_finger != Fingering::Open
    }
}
