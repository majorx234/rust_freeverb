use eframe::egui;
use plugin_host::PluginHostEguiApp;

fn main() {
    let options = eframe::NativeOptions::default();

    let plugin_host_egui_app = PluginHostEguiApp::new();

    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(plugin_host_egui_app))),
    );
}
