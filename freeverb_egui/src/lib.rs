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
    dampening: f64,
    width: f64,
    room_size: f64,
    freeze: bool,
    dry: f64,
    wet: f64,
}

impl Default for FreeverbEguiApp {
    fn default() -> Self {
        Self {
            dampening: 50.0,
            width: 50.0,
            room_size: 50.0,
            freeze: false,
            dry: 50.0,
            wet: 50.0,
        }
    }
}

impl eframe::App for FreeverbEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Freeverb");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.dampening, 0.0..=100.0).vertical());
                ui.add(egui::Slider::new(&mut self.width, 0.0..=100.0).vertical());
                ui.add(egui::Slider::new(&mut self.room_size, 0.0..=100.0).vertical());
                ui.add(toggle(&mut self.freeze));
                ui.add(egui::Slider::new(&mut self.dry, 0.0..=100.0).vertical());
                ui.add(egui::Slider::new(&mut self.wet, 0.0..=100.0).vertical());
            })
        });
    }
}
