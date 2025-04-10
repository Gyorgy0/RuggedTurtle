use std::{cmp::max, default, f32::consts::PI};

use egui::{
    self, color_picker::Alpha, include_image, load::SizedTexture, menu, CentralPanel, Color32,
    ImageSource, Rect, Stroke, TextureHandle, TextureOptions, TopBottomPanel, Vec2, Visuals,
    Widget,
};
use egui_dialogs::{
    dialog_window, Dialog, DialogContext, DialogDetails, Dialogs, StandardDialog, StandardReply,
};
use egui_extras::install_image_loaders;

use crate::{
    commands::execute_command,
    turtle::{convert_vecs, Color32u8, Point, Turtle},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RuggedTurtleApp<'a> {
    #[serde(skip)]
    input: String,
    text_editor: String,
    opened_editor: bool,
    #[serde(skip)]
    turtle: Turtle,
    canvas: (u16, u16),
    dark_mode: bool,
    #[serde(skip)]
    dialogs: Dialogs<'a>,
    colordialog: bool,
}

impl Default for RuggedTurtleApp<'_> {
    fn default() -> Self {
        Self {
            input: String::new(),
            text_editor: "".to_string(),
            opened_editor: false,
            turtle: Turtle::default(),
            canvas: (640, 480),
            dark_mode: false,
            dialogs: Dialogs::default(),
            colordialog: false,
        }
    }
}

impl<'a> RuggedTurtleApp<'_> {
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
        if application.dark_mode {
            cc.egui_ctx.set_visuals(Visuals::dark());
            cc.egui_ctx.set_pixels_per_point(1.25);
        } else if !application.dark_mode {
            cc.egui_ctx.set_visuals(Visuals::light());
            cc.egui_ctx.set_pixels_per_point(1.25);
        }
        application
    }
}

impl eframe::App for RuggedTurtleApp<'_> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Executes at the start of the program to initialize the turtle
        let turtle_icon: egui::ImageSource = include_image!("assets/rugged_turtle.svg");
        if self.turtle == Turtle::default() {
            let height = ctx.screen_rect().width().max(ctx.screen_rect().height()) * 0.030;
            self.turtle.set_size(0.75 * height, height);
            self.turtle
                .set_position(ctx.screen_rect().center().x, ctx.screen_rect().center().y);
            self.turtle.path.push(vec![]);
            self.turtle.angle = 0.0;
            self.turtle.pencolor = Color32u8::new(0, 0, 0, 255);
            if self.dark_mode {
                self.turtle.pencolor = Color32u8::new(255, 255, 255, 255);
            }
            self.turtle.path_color.push(self.turtle.pencolor);
            self.turtle.penwidth = 1.0;
            self.turtle.pen_up = false;
            self.turtle.path_width.push(self.turtle.penwidth);
            ctx.forget_image(turtle_icon.uri().unwrap());
        }
        CentralPanel::default().show(&ctx, |ui| {
            // Painting the lines drawn by the turtle
            for i in 0..self.turtle.path_color.len() {
                ui.painter().line(
                    convert_vecs(
                        self.turtle
                            .path
                            .get(i)
                            .clone()
                            .unwrap_or(&vec![self.turtle.position])
                            .to_vec(),
                    ),
                    Stroke::new(
                        *self.turtle.path_width.get(i).unwrap(),
                        *self.turtle.path_color.get(i).unwrap(),
                    ),
                );
            }
            // Plus function: Implementing customizable turtle images
            //self.turtle.set_icon(turtle_icon.uri().unwrap());
            egui::widgets::Image::new(turtle_icon.clone())
                .rotate((2_f32 * PI) - self.turtle.angle, Vec2::splat(0.5))
                .paint_at(
                    ui,
                    Rect::from_center_size(
                        self.turtle.position.into(),
                        Vec2::new(self.turtle.width, self.turtle.height),
                    ),
                );
        });
        if !self.dark_mode {
            TopBottomPanel::bottom("Console").show(ctx, |ui| {
                egui::widgets::TextEdit::singleline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .background_color(Color32::KHAKI)
                    .text_color(Color32::BLACK)
                    .ui(ui);
                if ui.button("Futtatás").clicked() {
                    execute_command(self.input.clone(), &mut self.turtle);
                }
            });
        } else if self.dark_mode {
            TopBottomPanel::bottom("Console").show(ctx, |ui| {
                egui::widgets::TextEdit::singleline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .ui(ui);

                if ui.button("Futtatás").clicked() {
                    DialogDetails::new(ColorPickerDialog {
                        picked_color: self.turtle.pencolor.into(),
                    })
                    .on_reply(move |res| {
                        res
                    })
                    .show(&mut self.dialogs);
                    execute_command(self.input.clone(), &mut self.turtle);
                }
            });
        }
        self.dialogs.show(ctx);
        if let Some(res) = self.dialogs.show(ctx) {
            // handle reply from close confirmation dialog
                match res.reply() {
                    Ok(Color32) => {
                        res.reply().unwrap();
                    },
                    _ => {},
                }
        }
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
                        self.dark_mode = true;
                    }
                    if ui.button("Világos mód").clicked() {
                        ctx.set_visuals(Visuals::light());
                        self.dark_mode = false;
                    }
                });
            });
        });
    }
}

pub struct ColorPickerDialog {
    pub picked_color: Color32,
}

impl Dialog<Color32> for ColorPickerDialog {
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<Color32> {
        // return None if the user hasn't replied
        let mut res = None;

        // draw the dialog
        dialog_window(ctx, dctx, "Confirm name").show(ctx, |ui| {
            ui.label("What's your name: ");
            egui::widgets::color_picker::color_picker_color32(
                ui,
                &mut self.picked_color,
                Alpha::OnlyBlend,
            );
            if ui.button("Done").clicked() {
                // set the reply and end the dialog
                res = Some(self.picked_color.clone());
            }
        });

        res
    }
}
