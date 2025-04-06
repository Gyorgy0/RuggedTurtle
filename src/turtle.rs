use egui::{Color32, Pos2};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Turtle {
    pub position: Point,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub icon_path: String,
    pub path: Vec<Vec<Point>>,
    pub pencolor: Color32u8,
    pub path_color: Vec<Color32u8>,
    pub penwidth: f32,
    pub path_width: Vec<f32>,
    pub pen_up: bool,
}

impl Turtle {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = Point::new(x, y);
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
#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

impl Into<egui::Pos2> for Point {
    fn into(self) -> Pos2 {
        Pos2::new(self.x, self.y)
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

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Color32u8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color32u8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl Into<egui::Color32> for Color32u8 {
    fn into(self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }
}
