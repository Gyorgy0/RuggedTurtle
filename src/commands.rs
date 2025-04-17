use std::{f32::consts::PI, vec};

use egui::{Color32, TextBuffer};
use serde::{Deserialize, Serialize};

use crate::{arithmetic::parse_number_value, parsing::trim_whitespace, turtle::Turtle};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum VariableTypes {
    Boolean { value: bool },
    Number { value: f64 },
}

impl VariableTypes {
    pub fn get_value(&self) -> f64 {
        let mut val = 0_f64;
        if let VariableTypes::Number { value } = self {
            val = *value;
        }
        val
    }
    pub fn _get_boolean(&self) -> bool {
        let mut val = false;
        if let VariableTypes::Boolean { value } = self {
            val = *value;
        }
        val
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Variable {
    pub raw_value: String,
    pub variable_type: VariableTypes,
    pub writable: bool,
}

const HELP_MENU_HUN:&str = "#############
 # Változók
 #############
 -   <változó_neve> = változó értéke (megadható más változó is, plusz aritmetikai műveletek (+, -, *, /, :, %))
 
 
 #########################
 # Aritmetikai műveletek
 #########################
 - '+' - összeadás (összead két számot, vagy változót)
 - '-' - kivonás (kivon egy számot egy másik számból, vagy változót)
 - '*' - szorzás (összeszoroz egy számot egy másik számmal, vagy változót)
 - '/' - teljes osztás (eloszt egy számot egy másik számmal, vagy változóval, nem feltétlen egész szám az eredmény)
 - ':' - egész osztás (eloszt egy számot egy másik számmal, vagy változóval, egész szám az eredmény)
 - '%' - maradékos osztás (eloszt egy számot egy másik számmal, vagy változóval, ennek az osztásnak a maradékát adja vissza)
 
 
 #############
 # Parancsok
 #############
 - elore(pixelek száma, amivel előre kell mennie a teknősnek)
 Rövidítések: e(), elore(), f(), forward()
 
 - jobbra(szög megadása fokban, hogy mennyit forduljon el a karakter jobb oldalára 0-360)
 Rövidítések: j(), jobb(), jobbra(), r(), right()
 
 - balra(szög megadása fokban, hogy mennyit forduljon el a karakter bal oldalára 0-360)
 Rövidítések: b(), bal(), balra(), l(), left()
 
 - tollszin(piros szín megadása 0-255, zöld szín megadása 0-255, kék szín megadása 0-255, alfa csatorna megadása 0-255) - megadja a vonal színét RGBA színként pl.(0,0,0,255) - fekete, (255,255,255,255) - fehér. (255,255,255,255) - átlátszó
 Rövidítések: tsz(), tollszin(), szin(), pc(), pencolor(), color()
 
 - tollvastagsag(toll vastagsága pixelekben, minnél nagyobb, annál vastagabb a vonal)
 Rövidítések: tv(), tollvastagsag(), vastagsag(), pw(), penwidth(), width()
 
 - tollfel - felveszi a tollat a vászonról, így a teknős nem hagy maga után nyomot
 Rövidítések: tf, tollfel, pu, penup
 
 - tollle - lerakja a tollat a vászonra, így a teknős újra nyomot hagy
 Rövidítések: tl, tollle, pd, pendown
 
 - kiertekeles(<változó>) - egyszerűsíti és kiírja a megadott változó értékét
 Rövidítések: kier(), kiertekeles(), kiszamolas(), eval(), calc(), calculate(), evaluate()
 
 - kiiratas(<változó>) - kiírja a megadott változót
 Rövidítések: ki(), kiir(), kiiratas(), print()
 
 - torol - kitörli a terminál kimenetét
 Rövidítések: trl, torol, clr, clear

 - alaphelyzet - alaphelyzetbe rakja az alkalmazást
 Rövidítések: alaphelyzet, reset, default
 
 - segitseg - kiírja a parancsokat és azok használatát
 Rövidítések: ?, segitseg, help
 
 - ismetles(változó, ettől, addig (exkluzív határ - megadott SZÁM ELŐTTI SZÁMIG megy)) {parancsok}
 Rövidítések: i() {}, ism() {}, ismetles() {}, r() {}, rep() {}, repeat() {}, for() {}";

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

const PRINTVAL: Command = Command {
    aliases: "kier kiertekeles kiszamolas eval calc calculate evaluate",
    //documentation:todo!(),
};

const PRINTRAW: Command = Command {
    aliases: "ki kiir kiiratas print",
    //documentation:todo!(),
};

const CLEAR: Command = Command {
    aliases: "trl torol clr clear",
    //documentation:todo!(),
};

const RESET: Command = Command {
    aliases: "alaphelyzet reset default",
    //documentation:todo!(),
};

const REPEAT: Command = Command {
    aliases: "i ism ismetles r rep repeat for",
    //documentation:todo!(),
};

const HELP: Command = Command {
    aliases: "? segitseg help",
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
            let bracket_begin_index: Vec<_> = command.1.match_indices("{").collect();
            bracket_begin = command.1.split_at(bracket_begin_index.first().unwrap().0);
            bracket_begin_remainder = bracket_begin.1.replacen('{', "", 1);
            structure.push(bracket_begin.0);
            let bracket_end_index: Vec<_> = bracket_begin_remainder.match_indices("}").collect();
            bracket_end = bracket_begin_remainder.split_at(bracket_end_index.last().unwrap().0);
            bracket_end_remainder = bracket_end.1.replacen('}', "", 1);
            structure.push(bracket_end.0);
            structure.push(&bracket_end_remainder);
            structure.retain(|&x| !x.is_empty());
            command_blocks = structure.last().unwrap();
        }
        if command.1.contains(&['(', ')'][..]) {
            structure = command.1.split(&['(', ')'][..]).collect();
            structure.retain(|&x| !x.is_empty());
            args = structure.get(1).unwrap().split(",").collect();
        } else if !command.1.contains(&['(', ')'][..]) {
            structure = command.1.split('=').collect();
            structure.retain(|&x| !x.is_empty());
            args = structure.first().unwrap().split(",").collect();
        }
        // This is where we declare the variable
        // <var>=<value> - value can be a boolean or a number
        if command.1.contains('=') && !command.1.contains(";") {
            let variable: Vec<&str> = command.1.split('=').collect();
            let variable_name_result: Result<f64, _> = variable.first().unwrap().parse();
            match variable_name_result {
                Ok(_val) => {
                    turtle
                        .command_history
                        .push("Érvényes számot nem lehet megadni változóként!".to_string());
                    return;
                }
                Err(_e) => {
                    let new_var = Variable {
                        raw_value: variable.get(1).unwrap().to_string(),
                        variable_type: VariableTypes::Number {
                            value: parse_number_value(variable.get(1).unwrap().to_string(), turtle),
                        },
                        writable: true,
                    };
                    if !turtle
                        .variables
                        .contains_key(variable.first().unwrap().as_str())
                    {
                        turtle
                            .variables
                            .insert(variable.first().unwrap().to_string(), new_var);
                    } else if turtle
                        .variables
                        .contains_key(variable.first().unwrap().as_str())
                    {
                        if !turtle
                            .variables
                            .get(variable.first().unwrap().as_str())
                            .unwrap()
                            .writable
                        {
                            turtle.command_history.push(format!(
                                "Nem lehet felülírni a \"{}\" változót!",
                                &variable.first().unwrap()
                            ));
                        } else {
                            turtle
                                .variables
                                .insert(variable.first().unwrap().to_string(), new_var);
                        }
                    }
                }
            }
            return;
        }
        // Splitting the commands and their aliases for easier matching
        let forward_commands: Vec<&str> = FORWARD.aliases.split(" ").collect();
        let rotate_right_commands: Vec<&str> = ROTATE_RIGHT.aliases.split(" ").collect();
        let rotate_left_commands: Vec<&str> = ROTATE_LEFT.aliases.split(" ").collect();
        let pencolor_commands: Vec<&str> = PENCOLOR.aliases.split(" ").collect();
        let penwidth_commands: Vec<&str> = PENWIDTH.aliases.split(" ").collect();
        let penup_commands: Vec<&str> = PENUP.aliases.split(" ").collect();
        let pendown_commands: Vec<&str> = PENDOWN.aliases.split(" ").collect();
        let printval_commands: Vec<&str> = PRINTVAL.aliases.split(" ").collect();
        let printraw_commands: Vec<&str> = PRINTRAW.aliases.split(" ").collect();
        let clear_commands: Vec<&str> = CLEAR.aliases.split(" ").collect();
        let reset_commands: Vec<&str> = RESET.aliases.split(" ").collect();
        let repeat_commands: Vec<&str> = REPEAT.aliases.split(" ").collect();
        let help_commands: Vec<&str> = HELP.aliases.split(" ").collect();
        //
        //  Printing out the chopped up input (command block for the execution controls are not chopped up)
        //
        //println!("Structure: {:?}", structure);
        if forward_commands.contains(structure.first().unwrap()) {
            let dist = parse_number_value(args.first().unwrap().to_string(), turtle);
            if dist.is_nan() || dist.is_infinite() {
                turtle
                    .command_history
                    .push(format!("A beírt távolságot ({}) nem tudja lemenni a teknős!", dist));
                return;
            }
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
            let angle: f64 = parse_number_value(args.first().unwrap().to_string(), turtle);
            if angle.is_nan() || angle.is_infinite() {
                turtle
                    .command_history
                    .push(format!("A beírt szögnyit ({}) nem tud fordulni a teknős!", angle));
                return;
            }
            else {
                let corrected_angle = angle as f32 * ((2_f32 * PI) / 360_f32);
            turtle.angle -= corrected_angle;
            }
        } else if rotate_left_commands.contains(structure.first().unwrap()) {
            let angle: f64 = parse_number_value(args.first().unwrap().to_string(), turtle);
            if angle.is_nan() || angle.is_infinite() {
                turtle
                    .command_history
                    .push(format!("A beírt szögnyit ({}) nem tud fordulni a teknős!", angle));
                return;
            }
            else {
                let corrected_angle = angle as f32 * ((2_f32 * PI) / 360_f32);
                turtle.angle -= (2_f32 * PI) - corrected_angle;
            }
        } else if pencolor_commands.contains(structure.first().unwrap()) {
            let r: f64 = parse_number_value(args.first().unwrap().to_string(), turtle);
            let g: f64 = parse_number_value(args.get(1).unwrap().to_string(), turtle);
            let b: f64 = parse_number_value(args.get(2).unwrap().to_string(), turtle);
            let a: f64 = parse_number_value(args.get(3).unwrap().to_string(), turtle);
            let mut colors = [r, g, b, a];
            (0..colors.len()).for_each(|color| {
                if colors[color].is_nan() {
                    turtle
                    .command_history
                    .push(format!("A beírt színérték ({}) nem érvényes! A színértékek 0 és 255 közötti egész számok lehetnek!",colors[color]));
                colors = [turtle.pencolor.r() as f64, turtle.pencolor.g() as f64, turtle.pencolor.b() as f64, turtle.pencolor.a() as f64];
                }
                else if colors[color] < 0_f64 || colors[color] > 255_f64 || (colors[color] %1_f64 != 0_f64){
                    turtle
                    .command_history
                    .push(format!("A színértékek csak 0 és 255 közötti egész számok lehetnek, az ({}) színérték érvénytelen!",colors[color]));
                    colors = [turtle.pencolor.r() as f64, turtle.pencolor.g() as f64, turtle.pencolor.b() as f64, turtle.pencolor.a() as f64];
                    return;
                }
            });
            turtle.pencolor = Color32::from_rgba_unmultiplied(colors[0] as u8,colors[1] as u8,colors[2] as u8,colors[3] as u8);
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);
        } else if penwidth_commands.contains(structure.first().unwrap()) {
            let width: f64 = parse_number_value(args.first().unwrap().to_string(), turtle);
            if width.is_nan() || width.is_infinite() {
                turtle
                    .command_history
                    .push(format!("A beírt nagyságú tollat ({}) nem tudja használni a teknős!", width));
                return;
            }
            else {
                turtle.penwidth = width as f32;
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);}
        } else if penup_commands.contains(structure.first().unwrap()) {
            turtle.pen_up = true;
        } else if pendown_commands.contains(structure.first().unwrap()) {
            turtle.pen_up = false;
            if turtle.path.last().is_some() {
            turtle.path.push(vec![]);
            turtle.path_color.push(turtle.pencolor);
            turtle.path_width.push(turtle.penwidth);
            }
        } else if printval_commands.contains(structure.first().unwrap()) {
            // Command for printing out the variables numerical or booleanic value
            let searched_var_result: Option<&Variable> =
                turtle.variables.get(*args.first().unwrap());
            let searched_var: &Variable = match searched_var_result {
                Some(result) => result,
                None => {
                    turtle.command_history.push(format!(
                        "Nem létezik a \"{}\" változó!",
                        args.first().unwrap()
                    ));
                    return;
                }
            };
            turtle.command_history.push(format!(
                "{} = {}",
                args.first().unwrap(),
                searched_var.variable_type.get_value()
            ));
            // Printing out all the variables
            //println!("{:?}", turtle.variables.iter());
        } else if printraw_commands.contains(structure.first().unwrap()) {
            // Command for printing out the variables raw value
            let searched_var_result: Option<&Variable> =
                turtle.variables.get(*args.first().unwrap());
            let searched_var: &Variable = match searched_var_result {
                Some(result) => result,
                None => {
                    turtle.command_history.push(format!(
                        "Nem létezik a \"{}\" változó!",
                        args.first().unwrap()
                    ));
                    return;
                }
            };
            turtle.command_history.push(format!(
                "{} = {}",
                args.first().unwrap(),
                searched_var.raw_value
            ));
        } else if clear_commands.contains(structure.first().unwrap()) {
            turtle.command_history.clear();
        } else if reset_commands.contains(structure.first().unwrap()) {
            *turtle = Turtle::default();
        } else if repeat_commands.contains(structure.first().unwrap()) {
            let from_result: Result<isize,_> = args.get(1).unwrap().parse();
            let to_result: Result<isize,_> = args.get(2).unwrap().parse();

            let from: isize = match from_result {
                Ok(val) => val,
                Err(_e) => {
                    turtle
                    .command_history
                    .push(format!("A beírt ciklus kezdet ({}) nem érvényes szám, csakis egész számok lehetnek megadva!", args.get(1).unwrap()));return},
            };
            let to: isize = match to_result {
                Ok(val) => val,
                Err(_e) => {
                    turtle
                    .command_history
                    .push(format!("A beírt ciklus vég ({}) nem érvényes szám, csakis egész számok lehetnek megadva!", args.get(1).unwrap()));return},
            };
            if from <= to {
            let var = Variable {
                raw_value: from.to_string(),
                variable_type: VariableTypes::Number { value: from as f64 },
                writable: false,
            };
            turtle
                .variables
                .insert(args.first().unwrap().to_string(), var);
            //
            //  Printing the command block that is passed to the next iteration
            //
            //println!("Command block: {}", command_blocks);
            for var in from..to {
                let variable = Variable {
                    raw_value: var.to_string(),
                    variable_type: VariableTypes::Number { value: var as f64 },
                    writable: false,
                };
                turtle
                    .variables
                    .insert(args.first().unwrap().to_string(), variable);
                execute_command(command_blocks.to_string(), turtle);
            }}
            else {
                turtle
                    .command_history
                    .push(format!("A beírt ciklus kezdetének ({}) kisebb, vagy egyenlőnek kell lennie, mint a végének ({})!", from, to));
            }
        } else if help_commands.contains(structure.first().unwrap()) {
            turtle.command_history.push(HELP_MENU_HUN.to_string());
        } else {
            turtle
                .command_history
                .push("A parancsok listájáért írd be a \"segitseg\" parancsot!".to_string());
        }
    });
}
