mod binary_panel;
mod dbg_app;
mod serializer_panel;
mod types;

use crate::dbg_app::DebuggerApp;
use eframe::egui::{Vec2, ViewportBuilder};
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions {
        viewport: ViewportBuilder {
            inner_size: Some(Vec2::new(1280.0, 720.0)),
            title: Some("SerBytes Debugger".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ = eframe::run_native(
        "SerBytes Debugger",
        options,
        Box::new(|_| Ok(Box::new(DebuggerApp::default()))),
    );
}
