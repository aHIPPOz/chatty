// main.rs - Native Linux Wayland Shell Entry Point
// Uses Smithay as Wayland compositor backend
// UI rendered via Slint
//
// WARNING: This is a template for future native development.
// Currently, the project focuses on WASM prototyping (see lib.rs).

use std::process;

fn main() {
    eprintln!("🐧 Shell Desktop - Native Wayland Version");
    eprintln!("");
    eprintln!("⚠️  Native backend is not yet implemented.");
    eprintln!("📍 Current focus: WASM web prototype (lib.rs)");
    eprintln!("");
    eprintln!("See SMITHAY_ROADMAP.md for the native development plan.");
    eprintln!("");
    eprintln!("To run the web version:");
    eprintln!("  wasm-pack build --release --target web");
    eprintln!("  python3 -m http.server 8000");
    eprintln!("  Open http://localhost:8000");
    eprintln!("");
    
    process::exit(1);
}
