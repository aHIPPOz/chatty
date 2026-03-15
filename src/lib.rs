#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
pub fn main() -> Result<(), JsValue> {
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Handle launcher toggle
    {
        let ui_handle = ui.as_weak();
        ui.on_show_launcher(move || {
            let ui = ui_handle.unwrap();
            ui.set_launcher_visible(true);
        });
    }

    {
        let ui_handle = ui.as_weak();
        ui.on_hide_launcher(move || {
            let ui = ui_handle.unwrap();
            ui.set_launcher_visible(false);
        });
    }

    // Handle workspace switching
    {
        let ui_handle = ui.as_weak();
        ui.on_request_increase_value(move || {
            let ui = ui_handle.unwrap();
            let ws = ui.get_active_workspace();
            ui.set_active_workspace(if ws < 4 { ws + 1 } else { 1 });
        });
    }

    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}



