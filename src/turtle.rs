use egui::{Color32, Pos2};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Turtle {
    pub position: Pos2,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub icon_path: String,
    pub path: Vec<Vec<Pos2>>,
    pub pencolor: Color32,
    pub path_color: Vec<Color32>,
    pub penwidth: f32,
    pub path_width: Vec<f32>,
    pub pen_up: bool,
}

impl Turtle {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Pos2::new(x, y);
    }
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
    pub fn set_icon(&mut self, path_to_img: &str) {
        self.icon_path = path_to_img.to_string();
    }
    pub fn get_icon(&mut self) -> &str {
        &self.icon_path
    }
    pub fn clear_all(&mut self) {
        self.path = vec![];
    }
}
