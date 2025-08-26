use eframe::egui;

pub struct FreeverbEguiApp {
    counter: u32,
}

impl Default for FreeverbEguiApp {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for FreeverbEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
            ui.horizontal(|ui| {
                if ui.button("up").clicked() {
                    self.counter += 1;
                }
                if ui.button("down").clicked() {
                    if self.counter > 0 {
                        self.counter -= 1;
                    }
                }
                ui.label(format!("value: {}", self.counter));
            })
        });
    }
}
