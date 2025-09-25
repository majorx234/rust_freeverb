use eframe::{egui, glow::Context};

pub struct PluginHostEguiApp {
    loaded_plugin: String,
}

impl PluginHostEguiApp {
    pub fn new() -> Self {
        PluginHostEguiApp {
            loaded_plugin: String::new(),
        }
    }
}

impl eframe::App for PluginHostEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PluginHost");
        });
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {
        println!("PluginHost closed");
    }
}
