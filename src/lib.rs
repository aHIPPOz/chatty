#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
pub fn main() -> Result<(), JsValue> {
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Setup launcher toggle
    {
        let ui_handle = ui.as_weak();
        ui.on_toggle_launcher(move || {
            let ui = ui_handle.unwrap();
            ui.set_show_launcher(!ui.get_show_launcher());
        });
    }

    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}



