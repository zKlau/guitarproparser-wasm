// MixTableChange structure
use serde::{Serialize, Deserialize};
use crate::model::rse::RseInstrument;
use super::item::MixTableItem;
use super::wah::WahEffect;

/// A MixTableChange describes a change in mix parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MixTableChange {
    pub instrument: Option<MixTableItem>,
    pub rse: RseInstrument,
    pub volume: Option<MixTableItem>,
    pub balance: Option<MixTableItem>,
    pub chorus: Option<MixTableItem>,
    pub reverb: Option<MixTableItem>,
    pub phaser: Option<MixTableItem>,
    pub tremolo: Option<MixTableItem>,
    pub tempo_name: String,
    pub tempo: Option<MixTableItem>,
    pub hide_tempo: bool,
    pub wah: Option<WahEffect>,
    pub use_rse: bool,
}

impl Default for MixTableChange {
    fn default() -> Self {
        MixTableChange {
            instrument: None,
            rse: RseInstrument::default(),
            volume: None,
            balance: None,
            chorus: None,
            reverb: None,
            phaser: None,
            tremolo: None,
            tempo_name: String::new(),
            tempo: None,
            hide_tempo: true,
            wah: None,
            use_rse: false,
        }
    }
}

impl MixTableChange {
    pub(crate) fn is_just_wah(&self) -> bool {
        self.instrument.is_none()
            && self.volume.is_none()
            && self.balance.is_none()
            && self.chorus.is_none()
            && self.reverb.is_none()
            && self.phaser.is_none()
            && self.tremolo.is_none()
            && self.tempo.is_none()
            && self.wah.is_none()
    }
}
