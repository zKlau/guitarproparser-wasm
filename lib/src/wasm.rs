use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use serde::Serialize;
use crate::model::song::Song;

#[wasm_bindgen]
pub fn parse_guitar_pro(data: &[u8], filename: &str) -> Result<JsValue, JsValue> {
    let mut song = Song::default();
    
    let ext = filename
        .split('.')
        .last()
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "UNKNOWN".to_string());
    
    let res = match ext.as_str() {
        "GP3" => song.read_gp3(data),
        "GP4" => song.read_gp4(data),
        "GP5" => song.read_gp5(data),
        "GP" => song.read_gp(data),
        "GPX" => song.read_gpx(data),
        _ => return Err(JsValue::from_str(&format!("Unsupported format '{}'. Supported: GP3, GP4, GP5, GP, GPX", ext))),
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