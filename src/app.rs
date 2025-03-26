use std::{cmp::max, f32::consts::PI};

use egui::{
    self, include_image, menu, CentralPanel, Color32, Image, Pos2, Rect, Stroke, TopBottomPanel,
    Vec2, Visuals, Widget,
};
use egui_extras::install_image_loaders;
use winit::application;

use crate::commands::{execute_command, Turtle};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RuggedTurtleApp {
    #[serde(skip)]
    input: String,
    text_editor: String,
    opened_editor: bool,
    #[serde(skip)]
    turtle: Turtle,
}

impl Default for RuggedTurtleApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            text_editor: "".to_string(),
            opened_editor: false,
            turtle: Turtle::default(),
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
        install_image_loaders(&cc.egui_ctx);
        let mut application: RuggedTurtleApp = Default::default();
        if let Some(storage) = cc.storage {
            application = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        application
    }
}

impl eframe::App for RuggedTurtleApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Executes at the start of the program to initialize the turtle
        if self.turtle == Turtle::default() {
            let height = ctx.screen_rect().width().max(ctx.screen_rect().height()) * 0.030;
            self.turtle.set_size(0.75 * height, height);
            //application
            //    .turtle
            //    .set_icon("/home/gyorgy/Desktop/Rust projects/RuggedTurtle/assets/rugged_turtle.svg");
            self.turtle
                .set_position(ctx.screen_rect().center().x, ctx.screen_rect().center().y);
            self.turtle.angle = 0.0;
        }
        CentralPanel::default().show(&ctx, |ui| {
            // Painting the lines drawn by the turtle
            ui.painter().line(self.turtle.path.clone(), Stroke::new(1.0, Color32::BLACK));
            // TODO: Implementing customizable turtle images
            egui::widgets::Image::new(include_image!(
                "/home/gyorgy/Desktop/Rust projects/RuggedTurtle/assets/rugged_turtle.svg"
            ))
            .rotate((2_f32 * PI) - self.turtle.angle, Vec2::splat(0.5))
            .paint_at(
                ui,
                Rect::from_center_size(
                    Pos2::new(self.turtle.pos_x, self.turtle.pos_y),
                    Vec2::new(self.turtle.width, self.turtle.height),
                ),
            );
        });
        TopBottomPanel::bottom("Console").show(ctx, |ui| {
            egui::widgets::TextEdit::singleline(&mut self.input)
                .desired_width(f32::INFINITY)
                .background_color(Color32::KHAKI)
                .ui(ui);
            if ui.button("Futtatás").clicked() {
                execute_command(self.input.clone(), &mut self.turtle);
            }
        });
        TopBottomPanel::top("Menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Fájl", |ui| {
                    if ui.button("Új").clicked() {
                        self.turtle = Turtle::default();
                    }
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
