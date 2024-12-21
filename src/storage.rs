use wasm_bindgen::prelude::*;
use web_sys::{window, Storage};

/// Store a string in localStorage
#[wasm_bindgen]
pub fn store_string(key: &str, value: &str) -> Result<(), JsValue> {
    // Get the `localStorage` object
    let storage = get_local_storage()?;

    // Store the string under the provided key
    storage.set_item(key, value)?;

    Ok(())
}

/// Retrieve a string from localStorage
#[wasm_bindgen]
pub fn retrieve_string(key: &str) -> Result<Option<String>, JsValue> {
    // Get the `localStorage` object
    let storage = get_local_storage()?;

    // Retrieve the string by its key
    let value = storage.get_item(key)?;

    Ok(value)
}

/// Helper to get the `localStorage` object
fn get_local_storage() -> Result<Storage, JsValue> {
    // Get the global `window` object
    let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;

    // Access `localStorage`
    window
        .local_storage()?
        .ok_or_else(|| JsValue::from_str("No `localStorage` available"))
}
