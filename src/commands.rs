use std::{f32::consts::PI, ops::Not, vec};

use egui::{Color32, Pos2};
use serde::{Deserialize, Serialize};

use crate::turtle::{Color32u8, Point, Turtle};

//use crate::documentation::Documentation;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum Commands {
    #[default]
    none,
    forward,
    rotate_right,
    rotate_left,
    pencolor,
    penwidth,
    penup,
    pendown,
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

const PENCOLOR: Command = Command {
    aliases: "tsz tollszin szin pc pencolor color",
    command: Commands::pencolor,
    //documentation:todo!(),
};

const PENWIDTH: Command = Command {
    aliases: "tv tollvastagsag vastagsag pw penwidth width",
    command: Commands::penwidth,
    //documentation:todo!(),
};

const PENUP: Command = Command {
    aliases: "tf tollfel pu penup",
    command: Commands::penup,
    //documentation:todo!(),
};

const PENDOWN: Command = Command {
    aliases: "tl tollle pd pendown",
    command: Commands::pendown,
    //documentation:todo!(),
};

pub fn execute_command(commandstring: String, turtle: &mut Turtle) {
    let command_tokens: Vec<&str> = commandstring.split(";").collect();
    command_tokens.iter().for_each(|command| {
        let trimmed_command = trim_whitespace(command);
        let structure: Vec<&str> = trimmed_command.split("(").collect();
        let mut arg: Vec<&str> = vec![];
        let mut args: Vec<&str> = vec![];
        let forward_commands: Vec<&str> = FORWARD.aliases.split(" ").collect();
        let rotate_right_commands: Vec<&str> = ROTATE_RIGHT.aliases.split(" ").collect();
        let rotate_left_commands: Vec<&str> = ROTATE_LEFT.aliases.split(" ").collect();
        let pencolor_commands: Vec<&str> = PENCOLOR.aliases.split(" ").collect();
        let penwidth_commands: Vec<&str> = PENWIDTH.aliases.split(" ").collect();
        let penup_commands: Vec<&str> = PENUP.aliases.split(" ").collect();
        let pendown_commands: Vec<&str> = PENDOWN.aliases.split(" ").collect();
        if structure.last().unwrap().contains(")") {
            arg = structure.last().unwrap().split(")").collect();
            args = arg.first().unwrap().split(",").collect();
        }
        if forward_commands.contains(structure.first().unwrap()) {
            let dist: i64 = arg.first().unwrap().parse().unwrap();
            let x_offset = dist as f32 * turtle.angle.sin();
            let y_offset = dist as f32 * turtle.angle.cos();
            if !turtle.pen_up {
                turtle.path[turtle.path_color.len() - 1].push(turtle.position);
                turtle.set_position(turtle.position.x - x_offset, turtle.position.y - y_offset);
                turtle.path[turtle.path_color.len() - 1].push(turtle.position);
            } else {
                turtle.set_position(turtle.position.x - x_offset, turtle.position.y - y_offset);
            }
        } else if rotate_right_commands.contains(structure.first().unwrap()) {
            let angle: f32 = arg.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32 * PI) / 360_f32);
            turtle.angle -= corrected_angle;
        } else if rotate_left_commands.contains(structure.first().unwrap()) {
            let angle: f32 = arg.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32 * PI) / 360_f32);
            turtle.angle -= (2_f32 * PI) - corrected_angle;
        } else if rotate_left_commands.contains(structure.first().unwrap()) {
            let angle: f32 = arg.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32 * PI) / 360_f32);
            turtle.angle -= (2_f32 * PI) - corrected_angle;
        } else if pencolor_commands.contains(structure.first().unwrap()) {
            let r: u8 = args.get(0).unwrap().parse().unwrap();
            let g: u8 = args.get(1).unwrap().parse().unwrap();
            let b: u8 = args.get(2).unwrap().parse().unwrap();
            let a: u8 = args.get(3).unwrap().parse().unwrap();
            turtle.pencolor = Color32u8::new(r, g, b, a);
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);
        } else if penwidth_commands.contains(structure.first().unwrap()) {
            let width: f32 = arg.first().unwrap().parse().unwrap();
            turtle.penwidth = width;
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);
        } else if penup_commands.contains(structure.first().unwrap()) {
            turtle.pen_up = true;
        } else if pendown_commands.contains(structure.first().unwrap()) {
            turtle.pen_up = false;
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);
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
