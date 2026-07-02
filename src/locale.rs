use serde::{Deserialize, Serialize};

pub fn import_locales(locales: &mut Vec<Locale>) -> Vec<Locale> {
    locales.clear();
    #[cfg(not(any(
        target_os = "android",
        target_arch = "wasm32",
        target_os = "ios",
        target_os = "macos"
    )))]
    {
        // Materials - PC version (loads them from the src/materials folder)

        use std::fs;
        let paths = fs::read_dir("src/locale").unwrap();
        for path in paths {
            if path
                .as_ref()
                .is_ok_and(|path| path.file_name() != "default_locale.yml")
            {
                let locale: Result<Vec<u8>, std::io::Error> =
                    fs::read(path.as_ref().unwrap().path().display().to_string().as_str());
                let serialized_locale: Locale = yaml_serde::from_reader(locale.unwrap().as_slice())
                    .unwrap_or(Locale::default());
                locales.push(serialized_locale);
            }
        }
    }
    #[cfg(any(
        target_os = "android",
        target_arch = "wasm32",
        target_os = "ios",
        target_os = "macos"
    ))]
    {
        // Materials - Portable version (includes the files in src/materials in the executable file)

        use yaml_serde::from_str;

        use crate::included_files::FILES;
        // Locale
        locales.push(from_str(&FILES.locales.locale_en).unwrap());
        locales.push(from_str(&FILES.locales.locale_hu).unwrap());
    }
    locales.clone()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Locale {
    pub language_id: String,
    pub language_name: String,
    pub terminal_help_message: String,
    pub run_button: String,
    pub pencolor_button: String,
    pub pen_width_button: String,
    pub reset_menu: String,
    pub file_menu: String,
    pub settings_menu: String,
    pub languages_menu: String,
    pub dark_theme_menu: String,
    pub light_theme_menu: String,
    pub terminal_title: String,
    pub colorpicker_dialog_title: String,
    pub colorpicker_dialog_text: String,
    pub done_button: String,
    pub cancel_button: String,
    pub copy_color_button: String,
    pub pen_width_dialog_title: String,
    pub pen_width_dialog_text: String,
    pub new_canvas_dialog_title: String,
    pub new_canvas_dialog_text: String,
    pub help_menu: String,
    pub invalid_var_name_error: String,
    pub var_immutable_error: String,
    pub invalid_distance_error: String,
    pub invalid_angle_error: String,
    pub invalid_color_value_error: String,
    pub invalid_color_interval_error: String,
    pub invalid_pen_size_error: String,
    pub invalid_variable_error: String,
    pub invalid_loop_start_error: String,
    pub invalid_loop_end_error: String,
    pub invalid_loop_interval_error: String,
    pub invalid_expression: String,
}

impl Locale {
    pub fn get_language_name(&self) -> String {
        String::from(&self.language_name)
    }
}

pub fn get_text(locale: &Vec<Locale>, selected_locale: usize) -> &Locale {
    &locale[selected_locale]
}

impl Default for Locale {
    fn default() -> Self {
        Self {
            language_id: String::from("EN"),
            language_name: String::from("English"),
            terminal_help_message: String::from("Type \"help\" to display the commands! If the turtle is not visible, type \"reset\" command."),
            run_button: String::from("Run"),
            pencolor_button: String::from("Change pen color..."),
            pen_width_button: String::from("Change pen width..."),
            reset_menu: String::from("Reset"),
            file_menu: String::from("File"),
            settings_menu: String::from("Settings"),
            languages_menu: String::from("Languages"),
            dark_theme_menu: String::from("Dark theme"),
            light_theme_menu: String::from("Light theme"),
            terminal_title: String::from(" - Command history - "),
            colorpicker_dialog_title: String::from("Color selection"),
            colorpicker_dialog_text: String::from("Please, select a color: "),
            done_button: String::from("Done"),
            cancel_button: String::from("Cancel"),
            copy_color_button: String::from("Copy color..."),
            pen_width_dialog_title: String::from("Line width selection"),
            pen_width_dialog_text: String::from("Please, adjust the line width: "),
            new_canvas_dialog_title: String::from("New canvas creation..."),
            new_canvas_dialog_text: String::from("Please, specify the size of the new canvas:"),
            help_menu: String::from(r#"#############
 # Variables
 #############
 -   <variable_name> = variable value (also it support arithmetic operations (+, -, *, /, :, %))
 
 
 #########################
 # Arithmetic operations
 #########################
 - '+' - addition (it adds two numbers or variables together)
 - '-' - subtraction (it subtracts a number or variable from another number or variable)
 - '*' - multiplication (multiplies a number or variable with another number or variable)
 - '/' - full division (divides a number or variable with another number or variable -> it can yield a decimal number)
 - ':' - integer division (divides a number or variable with another number or variable -> it yields an integer)
 - '%' - remainder (it gives the remainder from a division)
 
 
 #############
 # Commands
 #############
 - forward(number of pixels, that the turtle needs to move forward)
 Aliases: e(), elore(), f(), forward()
 
 - right(the angle that the turtle needs to turn to the right in degrees - 0-360)
 Aliases: j(), jobb(), jobbra(), r(), right()
 
 - left(the angle that the turtle needs to turn to the left in degrees - 0-360)
 Aliases: b(), bal(), balra(), l(), left()
 
 - pencolor(red color component 0-255, green color component 0-255, blue color component 0-255, alpha channel value 0-255) - it changes the color of the line using the RGBA color model. Examples: (0,0,0,255) - black, (255,255,255,255) - white. (255,255,255,0) - transparent
 Aliases: tsz(), tollszin(), szin(), pc(), pencolor(), color()
 
 - penwidth(width of the pen in pixels -> large value = thicker line)
 Aliases: tv(), tollvastagsag(), vastagsag(), pw(), penwidth(), width()
 
 - penup - it picks up the pen, so the turtle doesn't leave a trail
 Aliases: tf, tollfel, pu, penup
 
 - pendown - it puts down the pen, so the turtle leaves a trail
 Aliases: tl, tollle, pd, pendown
 
 - evaluate(<variable>) - it evaluates an expression or variable and displays it's simplified form
 Aliases: kier(), kiertekeles(), kiszamolas(), eval(), calc(), calculate(), evaluate()
 
 - print(<variable>) - it prints the variables' expression without simplification
 Aliases: ki(), kiir(), kiiratas(), print()
 
 - clear - clears the terminal's command history
 Aliases: trl, torol, clr, clear

 - reset - resets the application
 Aliases: alaphelyzet, reset, default
 
 - help - diplays the help menu for the commands
 Aliases: ?, segitseg, help
 
 - for(<variable>, from, to (exclusive boundary - it goes until it hits the number before the specified boundary)) {commands}
 Aliases: i() {}, ism() {}, ismetles() {}, r() {}, rep() {}, repeat() {}, for() {}"#),
            invalid_var_name_error: String::from("You can't name a variable as a valid numerical value/expression."),
            var_immutable_error: String::from("You can't update the \"{}\" variable!"),
            invalid_distance_error: String::from("The turtle can't travel the specified distance ({})!"),
            invalid_angle_error: String::from("The turtle can't rotate the specified angle ({})!"),
            invalid_color_value_error: String::from("The specified color value ({}) can't be specified! The color value intervals need to be between 0 and 255."),
            invalid_color_interval_error: String::from("The color values can be between 0 and 255. The color value ({}) is invalid!"),
            invalid_pen_size_error: String::from("The turtle can't set it's pen's size to {}!"),
            invalid_variable_error: String::from("The variable, named \"{}\" is not found!"),
            invalid_loop_start_error: String::from("The specified start of the loop ({}) is not a valid number!"),
            invalid_loop_end_error: String::from("The specified end of the loop ({}) is not a valid number!"),
            invalid_loop_interval_error: String::from("The specified start of the loop ({}) needs to be less or equal to it's end ({})!"),
            invalid_expression: String::from("The specified input ({}) can't be evaluated!"),
        }
    }
}
