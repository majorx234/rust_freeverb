use eframe::{
    egui::{self, Align2, Color32, Id, LayerId, Order, TextStyle},
    glow::Context,
};
use std::fmt;

pub struct PluginHostEguiApp {
    _loaded_plugin: String,
    dropped_files: Vec<egui::DroppedFile>,
}

impl PluginHostEguiApp {
    pub fn new() -> Self {
        PluginHostEguiApp {
            _loaded_plugin: String::new(),
            dropped_files: Vec::new(),
        }
    }
}

impl eframe::App for PluginHostEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PluginHost");
        });
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                for file in &i.raw.hovered_files {
                    if let Some(path) = &file.path {
                        fmt::write(&mut text, format_args!("\n{}", path.display()))
                            .expect("Error occurred while trying to write in String");
                        //write!(text, "\n{}", path.display()).ok();
                    } else if !file.mime.is_empty() {
                        fmt::write(&mut text, format_args!("\n{}", file.mime))
                            .expect("Error occurred while trying to write in String");
                        // write!(text, "\n{}", file.mime).ok();
                    } else {
                        text += "\n???";
                    }
                }
                text
            });

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });

        // Show dropped files (if any):
        if !self.dropped_files.is_empty() {
            let mut open = true;
            egui::Window::new("Dropped files")
                .open(&mut open)
                .show(ctx, |ui| {
                    for file in &self.dropped_files {
                        let mut info = if let Some(path) = &file.path {
                            path.display().to_string()
                        } else if !file.name.is_empty() {
                            file.name.clone()
                        } else {
                            "???".to_owned()
                        };

                        let mut additional_info = vec![];
                        if !file.mime.is_empty() {
                            additional_info.push(format!("type: {}", file.mime));
                        }
                        if let Some(bytes) = &file.bytes {
                            additional_info.push(format!("{} bytes", bytes.len()));
                        }
                        if !additional_info.is_empty() {
                            info += &format!(" ({})", additional_info.join(", "));
                        }

                        ui.label(info);
                    }
                });
            if !open {
                self.dropped_files.clear();
            }
        }
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {
        println!("PluginHost closed");
    }
}
