use audio_module::{parameters::Parameter, AudioModule, AudioProcessor, Widget};
use eframe::egui;

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter().rect(
            rect,
            radius,
            visuals.bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }
    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

pub struct FreeverbEguiApp {
    num_params: usize,
    params: Vec<Box<dyn Parameter>>,
}

impl FreeverbEguiApp {
    pub fn new<Module: AudioModule>() -> Self {
        let num_params = Module::parameter_count();
        let mut params = Vec::new();
        for idx in 0..num_params {
            params.push(Module::parameter(idx));
        }
        FreeverbEguiApp { num_params, params }
    }
}

impl eframe::App for FreeverbEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Freeverb");
            ui.horizontal(|ui| {
                for id in 0..self.num_params {
                    let parameter = &self.params[id];
                    let widget = match parameter.widget() {
                        Widget::Slider => {
                            let mut param_value = parameter.default_user_value();
                            ui.add(egui::Slider::new(&mut param_value, 0.0..=100.0).vertical());
                            // TODO: send data to id, rx_command
                        }
                        Widget::Button => {
                            let mut param_value = if parameter.default_user_value() == 0.0 {
                                false
                            } else {
                                true
                            };
                            ui.add(toggle(&mut param_value));
                            // TODO: send data to id, rx_command
                        }
                        _ => {}
                    };
                }
            });
        });
    }
}
