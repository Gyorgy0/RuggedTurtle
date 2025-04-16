use egui::{ahash::HashMap, Color32, Pos2};
use serde::{Deserialize, Serialize};

use crate::commands::Variable;

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
    pub variables: HashMap<String, Variable>,
}

impl Turtle {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Pos2::new(x, y);
    }
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}
