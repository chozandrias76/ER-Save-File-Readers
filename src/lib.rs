pub mod traits;
pub mod models;

use models::save_slot::checksum::Checksum;
use traits::binary_readable::BinaryReadable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn read_checksum_from_bytes(bytes: &[u8], offset: usize) -> Result<JsValue, JsValue> {
    let mut cursor = std::io::Cursor::new(&bytes[offset..]);
    let checksum = Checksum::read(&mut cursor).map_err(|e| e.to_string())?;
    JsValue::from_serde(&checksum).map_err(|e| e.to_string().into())
}