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

pub fn convert_vecs<T, U>(vector: Vec<T>) -> Vec<U>
where
    T: TryInto<U>,
    <T as std::convert::TryInto<U>>::Error: std::fmt::Display,
{
    vector
        .into_iter()
        .map(|value_t| match TryInto::try_into(value_t) {
            Ok(value_u) => value_u,
            Err(why) => {
                let t = std::any::type_name::<T>();
                let u = std::any::type_name::<U>();
                panic!("Error converting from {t} to {u}: {why}")
            }
        })
        .collect()
}
