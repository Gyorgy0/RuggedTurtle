use std::{f32::consts::PI, ops::RangeInclusive};

use egui::{
    self, color_picker::Alpha, include_image, menu, Align2, CentralPanel, Color32, Rect,
    ScrollArea, Shadow, Stroke, TextStyle, TopBottomPanel, Vec2, Visuals, Widget,
};
use egui_dialogs::{dialog_window, Dialog, DialogContext, DialogDetails, Dialogs};
use egui_extras::install_image_loaders;

use crate::{commands::execute_command, turtle::Turtle};

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
    #[serde(skip)]
    dialogopen: bool,
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
            dialogopen: false,
        }
    }
}

impl RuggedTurtleApp<'_> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(Visuals::light());
        cc.egui_ctx
            .style_mut(|style| style.visuals.window_shadow = Shadow::NONE);
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
        const COLOR_PICKER_DIALOG_ID: &str = "color_picker_dialog";
        const WIDTH_INPUT_DIALOG_ID: &str = "width_input_dialog";
        // Logic for showing the dialogs and handling the reply is there is one
        if let Some(res) = self.dialogs.show(ctx) {
            if res.is_reply_of(COLOR_PICKER_DIALOG_ID) {
                if let Ok(picked_color) = res.reply() {
                    self.turtle.pencolor = picked_color;
                    self.turtle.path.push(vec![]);
                    self.turtle.path_color.push(self.turtle.pencolor);
                    self.turtle.path_width.push(self.turtle.penwidth);
                    self.dialogopen = false;
                }
            } else if res.is_reply_of(WIDTH_INPUT_DIALOG_ID) {
                if let Ok(new_width) = res.reply() {
                    self.turtle.penwidth = new_width;
                    self.turtle.path.push(vec![]);
                    self.turtle.path_color.push(self.turtle.pencolor);
                    self.turtle.path_width.push(self.turtle.penwidth);
                    self.dialogopen = false;
                }
            }
        }

        // Executes at the start of the program to initialize the turtle
        let mut bottom_size = 0_f32;
        let turtle_icon: egui::ImageSource = include_image!("assets/rugged_turtle.svg");
        if self.turtle == Turtle::default() {
            self.turtle.command_history.clear();
            self.turtle
                .command_history
                .push("A parancsok listájáért írd be a \"segitseg\" parancsot!".to_string());
            let height = ctx.screen_rect().width().max(ctx.screen_rect().height()) * 0.030;
            self.turtle.set_size(0.75 * height, height);
            self.turtle
                .set_position(ctx.screen_rect().center().x, ctx.screen_rect().center().y);
            self.turtle.path.push(vec![]);
            self.turtle.angle = 0.0;
            self.turtle.pencolor = Color32::from_rgba_unmultiplied(0, 0, 0, 255);
            if self.dark_mode {
                self.turtle.pencolor = Color32::from_rgba_unmultiplied(255, 255, 255, 255);
            }
            self.turtle.path_color.push(self.turtle.pencolor);
            self.turtle.penwidth = 1.0;
            self.turtle.pen_up = false;
            self.turtle.path_width.push(self.turtle.penwidth);
            ctx.forget_image(turtle_icon.uri().unwrap());
        }
        if !self.dark_mode {
            TopBottomPanel::bottom("Console").show(ctx, |ui| {
                bottom_size = ui.available_size_before_wrap().y;
                egui::widgets::TextEdit::singleline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .background_color(Color32::KHAKI)
                    .text_color(Color32::BLACK)
                    .ui(ui);
                ui.horizontal(|ui| {
                    if ui.button("Futtatás").clicked() {
                        self.turtle.variables.clear();
                        execute_command(self.input.clone(), &mut self.turtle);
                    }
                    if ui.button("Tollszín módosítása...").clicked() {
                        DialogDetails::new(ColorPickerDialog::new(self.turtle.pencolor))
                            .with_id(COLOR_PICKER_DIALOG_ID)
                            .show(&mut self.dialogs);
                        self.dialogopen = true;
                    }
                    if ui.button("Toll vastagsága...").clicked() {
                        DialogDetails::new(WidthInputDialog::new(self.turtle.penwidth))
                            .with_id(WIDTH_INPUT_DIALOG_ID)
                            .show(&mut self.dialogs);
                        self.dialogopen = true;
                    }
                });
            });
        } else if self.dark_mode {
            TopBottomPanel::bottom("Console").show(ctx, |ui| {
                bottom_size = ui.available_size_before_wrap().y;
                egui::widgets::TextEdit::singleline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .ui(ui);
                ui.horizontal(|ui| {
                    if ui.button("Futtatás").clicked() {
                        self.turtle.variables.clear();
                        execute_command(self.input.clone(), &mut self.turtle);
                    }
                    if ui.button("Tollszín módosítása...").clicked() {
                        DialogDetails::new(ColorPickerDialog::new(self.turtle.pencolor))
                            .with_id(COLOR_PICKER_DIALOG_ID)
                            .show(&mut self.dialogs);
                        self.dialogopen = true;
                    }
                    if ui.button("Toll vastagsága...").clicked() {
                        DialogDetails::new(WidthInputDialog::new(self.turtle.penwidth))
                            .with_id(WIDTH_INPUT_DIALOG_ID)
                            .show(&mut self.dialogs);
                        self.dialogopen = true;
                    }
                });
            });
        }
        //self.dialogs.show(ctx);
        TopBottomPanel::top("Menubar").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Fájl", |ui| {
                    if ui.button("Alaphelyzet").clicked() {
                        self.turtle = Turtle::default();
                    }
                    /*if ui.button("Új vászon").clicked() {
                        self.turtle = Turtle::default();
                    }
                    if ui.button("Mentés").clicked() {
                        self.input = "Fájl mentve...".to_string();
                    }
                    if ui.button("Szöveg szerkesztése").clicked() && !self.opened_editor {
                        self.opened_editor = true;
                    }*/
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
        CentralPanel::default().show(ctx, |ui| {
            if !self.dialogopen {
                ctx.style_mut(|style| style.visuals.window_shadow = Shadow::NONE);
                egui::containers::Window::new(" - Parancsok üzenetei - ")
                    .anchor(
                        Align2::CENTER_BOTTOM,
                        Vec2::new(0.0, -(bottom_size + 2_f32)),
                    )
                    .movable(false)
                    .constrain(true)
                    .resizable([false, true])
                    .max_height(ui.max_rect().x_range().max / 3.0)
                    .min_width(ctx.screen_rect().x_range().max)
                    .show(ctx, |ui| {
                        let text_style = TextStyle::Monospace;
                        let row_height = ui.text_style_height(&text_style);
                        ScrollArea::vertical()
                            .auto_shrink(false)
                            .scroll_bar_visibility(
                                egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                            )
                            .stick_to_bottom(true)
                            .show_rows(
                                ui,
                                row_height,
                                self.turtle.command_history.len(),
                                |ui, row_range| {
                                    for row in row_range {
                                        ui.label(
                                            " ".to_string() + &self.turtle.command_history[row],
                                        );
                                    }
                                },
                            );
                    });
            }
            ScrollArea::new([true, true]).show(ui, |ui| {
                // Painting the lines drawn by the turtle
                for i in 0..self.turtle.path_color.len() {
                    ui.painter().line(
                        self.turtle
                            .path
                            .get(i)
                            .unwrap_or(&vec![self.turtle.position])
                            .to_vec(),
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
                            self.turtle.position,
                            Vec2::new(self.turtle.width, self.turtle.height),
                        ),
                    );
            });
        });
    }
}

pub struct ColorPickerDialog {
    pub picked_color: Color32,
    pub original_color: Color32,
}

impl ColorPickerDialog {
    pub fn new(color: Color32) -> Self {
        Self {
            picked_color: color,
            original_color: color,
        }
    }
}
impl Dialog<Color32> for ColorPickerDialog {
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<Color32> {
        // Return None if the user hasn't selected something
        let mut res = None;

        // Draw the dialog ui
        dialog_window(ctx, dctx, "Szín kiválasztása").show(ctx, |ui| {
            ui.label("Kérlek, válassz egy színt: ");
            egui::widgets::color_picker::color_picker_color32(
                ui,
                &mut self.picked_color,
                Alpha::OnlyBlend,
            );
            ui.horizontal(|ui| {
                if ui.button("Kész").clicked() {
                    res = Some(self.picked_color);
                }
                if ui.button("Mégse").clicked() {
                    res = Some(self.original_color);
                }
                if ui.button("Szín kimásolása...").clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = format!(
                            "{}, {}, {}, {}",
                            self.picked_color.r(),
                            self.picked_color.g(),
                            self.picked_color.b(),
                            self.picked_color.a()
                        )
                    });
                    res = Some(self.original_color);
                }
            });
        });

        res
    }
}

pub struct WidthInputDialog {
    pub new_width: f32,
    pub original_width: f32,
}

impl WidthInputDialog {
    pub fn new(width: f32) -> Self {
        Self {
            new_width: width,
            original_width: width,
        }
    }
}
impl Dialog<f32> for WidthInputDialog {
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<f32> {
        // Return None if the user hasn't selected something
        let mut res = None;

        // Draw the dialog ui
        dialog_window(ctx, dctx, "Vonalvastagság kiválasztása").show(ctx, |ui| {
            ui.label("Kérlek, add meg, milyen vastag legyen a vonal: ");
            egui::Slider::new(&mut self.new_width, RangeInclusive::new(0_f32, 100_f32)).ui(ui);
            ui.horizontal(|ui| {
                if ui.button("Kész").clicked() {
                    res = Some(self.new_width);
                }
                if ui.button("Mégse").clicked() {
                    res = Some(self.original_width);
                }
            });
        });

        res
    }
}

pub struct NewCanvasDialog {
    pub size: Vec2,
    pub original_size: Vec2,
}

impl NewCanvasDialog {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            original_size: size,
        }
    }
}
impl Dialog<Vec2> for NewCanvasDialog {
    fn show(&mut self, ctx: &egui::Context, dctx: &DialogContext) -> Option<Vec2> {
        // Return None if the user hasn't selected something
        let mut res = None;

        // Draw the dialog ui
        dialog_window(ctx, dctx, "Új vászon létrehozása...").show(ctx, |ui| {
            ui.label("Kérlek, add meg az új vászon méretét: ");
            ui.horizontal(|ui| {
                if ui.button("Kész").clicked() {
                    res = Some(self.size);
                }
                if ui.button("Mégse").clicked() {
                    res = Some(self.original_size);
                }
            });
        });

        res
    }
}
