use std::{f32::consts::PI, vec};

use egui::Pos2;
use log::debug;
use serde::{Deserialize, Serialize};

//use crate::documentation::Documentation;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Turtle {
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    icon_path: String,
    pub path: Vec<Pos2>,
}

impl Turtle {
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.pos_x = x;
        self.pos_y = y;
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

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}



#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum Commands {
    #[default]
    none,
    forward,
    rotate_right,
    rotate_left,
}
#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Command {
    aliases: &'static str,
    command: Commands,
    //documentation: todo!(),
}

const FORWARD: Command = Command {
    aliases: "e elore f forward",
    command: Commands::forward,
    //documentation:todo!(),
};

const ROTATE_RIGHT: Command = Command {
    aliases: "j jobb jobbra r right",
    command: Commands::rotate_right,
    //documentation:todo!(),
};

const ROTATE_LEFT: Command = Command {
    aliases: "b bal balra l left",
    command: Commands::rotate_left,
    //documentation:todo!(),
};

pub fn execute_command(commandstring: String, turtle: &mut Turtle) {
    let command_tokens:Vec<&str> = commandstring.split(";").collect();
    command_tokens.iter().for_each(|command| {
        let trimmed_command = trim_whitespace(command);
        let structure:Vec<&str> = trimmed_command.split("(").collect();
        let mut arg:Vec<&str> = vec![];
        let forward_commands:Vec<&str> = FORWARD.aliases.split(" ").collect();
        let rotate_right_commands:Vec<&str> = ROTATE_RIGHT.aliases.split(" ").collect();
        let rotate_left_commands:Vec<&str> = ROTATE_LEFT.aliases.split(" ").collect();
        if structure.last().unwrap().contains(")") {
            arg = structure.last().unwrap().split(")").collect();
        }
        if forward_commands.contains(structure.first().unwrap()) {
            let dist:i64 = arg.first().unwrap().parse().unwrap();
            let x_offset = dist as f32 * turtle.angle.sin();
            let y_offset = dist as f32 * turtle.angle.cos();
            turtle.path.push(Pos2::new(turtle.pos_x, turtle.pos_y));
            turtle.pos_x -= x_offset;
            turtle.pos_y -= y_offset;
            turtle.path.push(Pos2::new(turtle.pos_x, turtle.pos_y));
        }
        else if rotate_right_commands.contains(structure.first().unwrap()) {
            let angle:f32 = arg.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32*PI)/360_f32);
            turtle.angle -= corrected_angle; 
        }
        else if rotate_left_commands.contains(structure.first().unwrap()) {
            let angle:f32 = arg.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32*PI)/360_f32);
            turtle.angle -= (2_f32 * PI) - corrected_angle;
            
        }
    });
}

// Removes whitespace characters from the string
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str = s.trim().to_owned();
    let mut prev = ' ';
    new_str.retain(|ch| {
        let result = ch != ' ' || prev != ' ';
        prev = ch;
        result
    });
    new_str
}
