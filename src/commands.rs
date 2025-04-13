use std::{f32::consts::PI, vec};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::turtle::Turtle;

//use crate::documentation::Documentation;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum Commands {
    #[default]
    none,
    forward,
    rotate_right,
    rotate_left,
    pencolor,
    pencolorpicker,
    penwidth,
    penwidthpicker,
    penup,
    pendown,
    repeat,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum VariableTypes {
    boolean(bool),
    number(f64),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Variable {
    name: String,
    variable_type: VariableTypes,
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

const REPEAT: Command = Command {
    aliases: "i ism ismetles r rep repeat for",
    command: Commands::repeat,
    //documentation:todo!(),
};
pub fn execute_command(commandstring: String, turtle: &mut Turtle) {
    // Removing whitespaces
    let mut trimmed_commandstring: String = trim_whitespace(&commandstring);
    // This splits off the input
    // e.g. initial input:      "forward(100);right(90);forward(10)"
    //      processed output:   ["forward(100)", "right(90)", "forward(10)"]
    let mut splitted_command_tokens: Vec<&str> = trimmed_commandstring.split(";").collect();
    let mut command_tokens: Vec<String> = splitted_command_tokens
        .iter_mut()
        .map(|f| f.to_string())
        .collect();
    //
    //  Printing out the trimmed tokens
    //
    println!("{:?}", command_tokens);
    let mut is_command_block = false;
    let mut repeat_command: Vec<String> = vec![];
    (0..command_tokens.clone().len()).into_iter().for_each(|i| {
        if command_tokens[i].contains("{") {
            is_command_block = true;
            repeat_command.push(command_tokens[i].to_string());
            command_tokens[i] = "".to_string();
        } else if !command_tokens[i].contains("}")
            && !command_tokens[i].contains("}")
            && is_command_block
        {
            repeat_command.push(command_tokens[i].to_string());
            command_tokens[i] = "".to_string();
        } else if command_tokens[i].contains("}") {
            is_command_block = false;
            repeat_command.push(command_tokens[i].to_string());
            command_tokens[i] = repeat_command.join(";");
            //
            //  Printing out the trimmed tokens
            //
            println!("{:?}", repeat_command);
            repeat_command = vec![];
        }
    });
    command_tokens.retain(|x| *x != "".to_string());
    //
    //  Printing out the chopped up input (command block for the execution controls are not chopped up)
    //
    println!("{:?}", command_tokens);
    command_tokens.iter().enumerate().for_each(|command| {
        // Structure:
        // e.g  input:  "forward(100)"
        //      output: ["forward", "100)"]
        // Structure of the <structure> variable
        // structure = ["<command>", "<arguments>)", "<command_block>}"]
        let structure: Vec<&str> = command.1.split('(').collect();
        let mut arg: Vec<&str> = vec![];
        let mut args: Vec<&str> = vec![];
        let mut command_block: Vec<&str> = vec![];
        let mut command_blocks: &str = "";
        // Splitting the commands and their aliases for easier matching
        let forward_commands: Vec<&str> = FORWARD.aliases.split(" ").collect();
        let rotate_right_commands: Vec<&str> = ROTATE_RIGHT.aliases.split(" ").collect();
        let rotate_left_commands: Vec<&str> = ROTATE_LEFT.aliases.split(" ").collect();
        let pencolor_commands: Vec<&str> = PENCOLOR.aliases.split(" ").collect();
        let penwidth_commands: Vec<&str> = PENWIDTH.aliases.split(" ").collect();
        let penup_commands: Vec<&str> = PENUP.aliases.split(" ").collect();
        let pendown_commands: Vec<&str> = PENDOWN.aliases.split(" ").collect();
        let repeat_commands: Vec<&str> = REPEAT.aliases.split(" ").collect();
        if structure.get(1).unwrap().contains(")") {
            arg = structure.get(1).unwrap().split(")").collect();
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
        } else if pencolor_commands.contains(structure.first().unwrap()) {
            let r: u8 = args.first().unwrap().parse().unwrap();
            let g: u8 = args.get(1).unwrap().parse().unwrap();
            let b: u8 = args.get(2).unwrap().parse().unwrap();
            let a: u8 = args.get(3).unwrap().parse().unwrap();
            turtle.pencolor = Color32::from_rgba_unmultiplied(r, g, b, a);
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
        if structure.last().unwrap().contains("}") {
            command_block = structure.last().unwrap().split("}").collect();
            command_blocks = command_block.first().unwrap();
        } /*if repeat_commands.contains(structure.first().unwrap()) {
              let from: usize = args.get(1).unwrap().parse().unwrap();
              let to: usize = args.get(2).unwrap().parse().unwrap();
              for i in from..to {
                  execute_command(command_blocks.to_string(), turtle);
              }
          }*/
    });
}

// Removes whitespace characters from the string
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str: Vec<&str> = s.split_whitespace().collect();
    new_str.retain(|&x| x != "");
    let mut string = new_str.join("").to_string();

    string
}
