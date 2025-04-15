use std::{f32::consts::PI, num::ParseIntError, vec};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::{parsing::trim_whitespace, turtle::Turtle};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum VariableTypes {
    boolean { value: bool },
    number { value: f64 },
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Variable {
    name: String,
    raw_value: String,
    variable_type: VariableTypes,
    writable: bool,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Command {
    aliases: &'static str,
    //documentation: todo!(),
}

const FORWARD: Command = Command {
    aliases: "e elore f fd forward",
    //documentation:todo!(),
};

const ROTATE_RIGHT: Command = Command {
    aliases: "j jobb jobbra r rt right",
    //documentation:todo!(),
};

const ROTATE_LEFT: Command = Command {
    aliases: "b bal balra l lt left",
    //documentation:todo!(),
};

const PENCOLOR: Command = Command {
    aliases: "tsz tollszin szin pc pencolor color",
    //documentation:todo!(),
};

const PENWIDTH: Command = Command {
    aliases: "tv tollvastagsag vastagsag pw penwidth width",
    //documentation:todo!(),
};

const PENUP: Command = Command {
    aliases: "tf tollfel pu penup up",
    //documentation:todo!(),
};

const PENDOWN: Command = Command {
    aliases: "tl tollle pd pendown down",
    //documentation:todo!(),
};

const REPEAT: Command = Command {
    aliases: "i ism ismetles r rep repeat for",
    //documentation:todo!(),
};
pub fn execute_command(commandstring: String, turtle: &mut Turtle) {
    // Removing whitespaces
    let trimmed_commandstring: String = trim_whitespace(&commandstring);
    // This splits off the input
    // e.g. initial input:      "forward(100);right(90);forward(10)"
    //      processed output:   ["forward(100)", "right(90)", "forward(10)"]
    let mut splitted_command_tokens: Vec<&str> = trimmed_commandstring.split(";").collect();
    let mut command_tokens: Vec<String> = splitted_command_tokens
        .iter_mut()
        .map(|f| f.to_string())
        .collect();
    let mut command_block_counter: i32 = 0;
    let mut chained_commands: Vec<String> = vec![];
    //
    //  Printing out the command tokens before further processing
    //
    //println!("Raw cmd tokens: {:?}", command_tokens);

    // We chain the command together by specifying how deep the nested function goes
    // command_block_counter specifies the depth of the nested function
    (0..command_tokens.clone().len()).for_each(|i| {
        if command_tokens[i].contains("{")
            || command_tokens[i].contains("}")
            || command_block_counter > 0
        {
            command_block_counter += command_tokens[i].matches("{").count() as i32
                - command_tokens[i].matches("}").count() as i32;
            chained_commands.push(command_tokens[i].to_string());
            command_tokens[i] = "".to_string();
        }
        if command_block_counter == 0 {
            chained_commands.push(command_tokens[i].to_string());
            chained_commands.retain(|x| !x.is_empty());
            command_tokens[i] = chained_commands.join(";");
            chained_commands = vec![];
        }
    });
    command_tokens.retain(|x| !x.is_empty());
    //
    //  Printing out the chopped up input (command block for the execution controls are not chopped up)
    //
    //println!("Cmd tokens: {:?}", command_tokens);
    command_tokens.iter().enumerate().for_each(|command| {
        let mut args: Vec<&str> = vec![];
        let mut command_blocks: &str = "";
        // Structure:
        // e.g  input:  "forward(100)"
        //      output: ["forward", "100)"]
        // Structure of the <structure> variable
        // structure = ["<command>", "(<arguments>)", "{<command_block>}"]
        let mut structure: Vec<&str> = vec![];
        let mut bracket_begin: (&str, &str) = ("", "");
        let mut bracket_begin_remainder: String = "".into();
        let mut bracket_end: (&str, &str) = ("", "");
        let mut bracket_end_remainder: String = "".into();
        if command.1.contains("{") || command.1.contains("}") {
            let bracket_index_begin: Vec<_> = command.1.match_indices('{').collect();
            bracket_begin = command.1.split_at(bracket_index_begin.first().unwrap().0);
            bracket_begin_remainder = bracket_begin.1.replacen('{', "", 1);
            structure.push(bracket_begin.0);
            let bracket_index_end: Vec<_> = bracket_begin_remainder.match_indices('}').collect();
            bracket_end = bracket_begin_remainder.split_at(bracket_index_end.last().unwrap().0);
            bracket_end_remainder = bracket_end.1.replacen('}', "", 1);
            structure.push(bracket_end.0);
            structure.push(&bracket_end_remainder);
            structure.retain(|&x| !x.is_empty());
            command_blocks = structure.last().unwrap();
        }
        structure = command.1.split(&['(', ')'][..]).collect();
        structure.retain(|&x| !x.is_empty());
        args = structure.get(1).unwrap().split(",").collect();
        // Splitting the commands and their aliases for easier matching
        let forward_commands: Vec<&str> = FORWARD.aliases.split(" ").collect();
        let rotate_right_commands: Vec<&str> = ROTATE_RIGHT.aliases.split(" ").collect();
        let rotate_left_commands: Vec<&str> = ROTATE_LEFT.aliases.split(" ").collect();
        let pencolor_commands: Vec<&str> = PENCOLOR.aliases.split(" ").collect();
        let penwidth_commands: Vec<&str> = PENWIDTH.aliases.split(" ").collect();
        let penup_commands: Vec<&str> = PENUP.aliases.split(" ").collect();
        let pendown_commands: Vec<&str> = PENDOWN.aliases.split(" ").collect();
        let repeat_commands: Vec<&str> = REPEAT.aliases.split(" ").collect();
        //
        //  Printing out the chopped up input (command block for the execution controls are not chopped up)
        //
        //println!("Structure: {:?}", structure);
        if forward_commands.contains(structure.first().unwrap()) {
            let dist_result: Result<i64, ParseIntError> = args.first().unwrap().parse();
            let dist_inner: &&str = args.first().unwrap();
            let dist = match dist_result {
                Ok(distance) => distance,
                Err(_e) => {
                    println!(
                        "Sajnos a megadott távolság érvénytelen (\"{}\")!",
                        dist_inner
                    );
                    return;
                }
            };
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
            let angle: f32 = args.first().unwrap().parse().unwrap();
            let corrected_angle = angle * ((2_f32 * PI) / 360_f32);
            turtle.angle -= corrected_angle;
        } else if rotate_left_commands.contains(structure.first().unwrap()) {
            let angle: f32 = args.first().unwrap().parse().unwrap();
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
            let width: f32 = args.first().unwrap().parse().unwrap();
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
        } else if repeat_commands.contains(structure.first().unwrap()) {
            let from: usize = args.get(1).unwrap().parse().unwrap();
            let to: usize = args.get(2).unwrap().parse().unwrap();
            //
            //  Printing the command block that is passed to the next iteration
            //
            //println!("Command block: {}", command_blocks);
            for i in from..to {
                execute_command(command_blocks.to_string(), turtle);
            }
        }
    });
}
