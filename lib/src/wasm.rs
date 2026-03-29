use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use serde::Serialize;
use crate::model::song::Song;

#[wasm_bindgen]
pub fn parse_guitar_pro(data: &[u8]) -> Result<JsValue, JsValue> {
    let mut song = Song::default();
    
    // Attempt to detect format by extension or content?
    // For now, let's try GP5 as a default or implement a generic reader
    // The CLI uses extension. In WASM, we might just try them or ask the user to specify.
    
    // A simple way to detect GP3/4/5 is by looking at the first bytes for the version string.
    let res = if data.len() > 30 {
        let version_str = String::from_utf8_lossy(&data[1..31]);
        if version_str.contains("FICHIER GUITAR PRO v3") {
            song.read_gp3(data)
        } else if version_str.contains("FICHIER GUITAR PRO v4") {
            song.read_gp4(data)
        } else if version_str.contains("FICHIER GUITAR PRO v5") {
            song.read_gp5(data)
        } else {
            // Try GP (GP7) or GPX (GP6) which are zip files
            if data.len() > 4 && &data[0..4] == b"BCF\x04" {
                 song.read_gp(data)
            } else if data.len() > 4 && &data[0..4] == b"PK\x03\x04" {
                 song.read_gpx(data)
            } else {
                return Err(JsValue::from_str("Unsupported or unknown Guitar Pro format"));
            }
        }
    } else {
        return Err(JsValue::from_str("File too short"));
    };

    match res {
        Ok(_) => {
            let serializer = serde_wasm_bindgen::Serializer::json_compatible();
            song.serialize(&serializer)
                .map_err(|e: serde_wasm_bindgen::Error| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&format!("Error reading file: {}", e))),
    }
}
