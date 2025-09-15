use audio_module::{parameters::Parameter, AudioModule, Command, Widget};
use bus::Bus;
use crossbeam_channel::Sender;
use eframe::{egui, glow::Context};

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
    params: Vec<(Box<dyn Parameter>, f32)>,
    jack_thread: Option<std::thread::JoinHandle<()>>,
    tx_close: Option<Bus<bool>>,
    tx_command: Option<Sender<Command>>,
}

impl FreeverbEguiApp {
    pub fn new<Module: AudioModule>(
        jack_thread: Option<std::thread::JoinHandle<()>>,
        tx_close: Option<Bus<bool>>,
        tx_command: Option<Sender<Command>>,
    ) -> Self {
        let num_params = Module::parameter_count();
        let mut params = Vec::new();
        for idx in 0..num_params {
            params.push((
                Module::parameter(idx),
                Module::parameter(idx).default_user_value(),
            ));
        }
        FreeverbEguiApp {
            num_params,
            params,
            tx_close,
            tx_command,
            jack_thread,
        }
    }
}

impl eframe::App for FreeverbEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Freeverb");
            ui.horizontal(|ui| {
                for id in 0..self.num_params {
                    let parameter = &self.params[id];
                    let mut param_value = parameter.1;
                    match parameter.0.widget() {
                        Widget::Slider => {
                            ui.add(egui::Slider::new(&mut param_value, 0.0..=100.0).vertical());
                        }
                        Widget::Button => {
                            let mut param_value_bool = parameter.1 != 0.0;
                            ui.add(toggle(&mut param_value_bool));
                            param_value = if param_value_bool { 1.0 } else { 0.0 };
                        }
                        _ => {}
                    };
                    if param_value != parameter.1 {
                        self.params[id].1 = param_value;
                        //if value changed
                        if let Some(ref tx_command) = self.tx_command {
                            tx_command
                                .try_send(Command::SetParameter(id, param_value))
                                .unwrap();
                        }
                    }
                }
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {
        if let Some(ref mut tx_close) = self.tx_close {
            if let Err(e) = tx_close.try_broadcast(false) {
                println!("could not send close e: {}", e);
            };

            if let Some(jack_thread) = self.jack_thread.take() {
                jack_thread.join().unwrap();
            }
            println!("jack_thread closed");
        }
    }
}
