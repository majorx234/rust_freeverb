use eframe::egui;
use freeverb_lib::FreeverbEguiApp;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "FreeverbEgui App",
        options,
        Box::new(|_cc| Ok(Box::new(FreeverbEguiApp::default()))),
    );
}
