use eframe::{
    egui::{
        self, vec2, CursorIcon, Id, InnerResponse, Label, LayerId, Order, Rect, Sense, Shape, Ui,
        Vec2,
    },
    glow::Context,
};

pub fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    //let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());
    let is_being_dragged = ui.memory(|mem| mem.is_anything_being_dragged());

    let margin = Vec2::splat(4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout(), None);
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    InnerResponse::new(ret, response)
}

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
        egui::CentralPanel::default().show(ctx, |mut ui| {
            let can_accept_what_is_being_dragged = true;
            let response = drop_target(&mut ui, can_accept_what_is_being_dragged, |ui| {
                // if ui.memory(|mem| mem.data.borrow_mut {}
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&Context>) {
        println!("PluginHost closed");
    }
}
