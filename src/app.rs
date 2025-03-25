use egui::{self, menu, CentralPanel, Color32, Stroke, TopBottomPanel, Visuals, Widget};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RuggedTurtleApp {
    #[serde(skip)]
    input: String,
    text_editor: String,
    opened_editor: bool,
}

impl Default for RuggedTurtleApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            text_editor: "// A very simple example\n\
                            fn main() {\n\
                            \tprintln!(\"Hello world!\");\n\
                            }\n\
                        "
            .to_string(),
            opened_editor: false,
        }
    }
}

impl RuggedTurtleApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(Visuals::light());
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for RuggedTurtleApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(&ctx, |ui| {
            egui::Window::new("Szöveg szerkesztő")
                .collapsible(true)
                .open(&mut self.opened_editor)
                .show(ctx, |ctx| {
                    egui::widgets::TextEdit::multiline(&mut self.text_editor)
                        .code_editor()
                        .ui(ctx);
                });
            ui.painter().circle(
                ctx.screen_rect().center(),
                55.0,
                Color32::RED,
                Stroke::new(15.0, Color32::BLUE),
            );
        });
        TopBottomPanel::bottom("Console").show(ctx, |ui| {
            egui::widgets::TextEdit::singleline(&mut self.input)
                .desired_width(f32::INFINITY)
                .background_color(Color32::KHAKI)
                .ui(ui);
        });
        TopBottomPanel::top("Menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Fájl", |ui| {
                    if ui.button("Mentés").clicked() {
                        self.input = "Fájl mentve...".to_string();
                    }
                    if ui.button("Szöveg szerkesztése").clicked() && !self.opened_editor {
                        self.opened_editor = true;
                    }
                });
                ui.menu_button("Beállítások", |ui| {
                    if ui.button("Sötét mód").clicked() {
                        ctx.set_visuals(Visuals::dark());
                    }
                    if ui.button("Világos mód").clicked() {
                        ctx.set_visuals(Visuals::light());
                    }
                });
            });
        });
    }
}
