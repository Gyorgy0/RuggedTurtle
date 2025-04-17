use crate::{parsing::trim_whitespace, turtle::Turtle};

pub fn parse_number_value(input: String, turtle: &mut Turtle) -> f64 {
    // Removes whitespaces
    let mut trimmed_input = trim_whitespace(input.as_str());
    trimmed_input.push('#');
    // We split up the input to tokens
    // e.g. input:  15+9+8+(6+5)+x
    //      output: ["15","9", "(6", "5)", x]
    let mut tokens: Vec<String> = vec![];
    let mut operators: Vec<char> = vec![];
    let mut parenthesis_counter = 0;
    let mut new_token = String::new().to_owned();
    let allowed_operators = ['+', '-', '*', '/', ':', '%'];
    trimmed_input.chars().for_each(|char| {
        if allowed_operators.iter().any(|op| op == &char) && parenthesis_counter == 0 {
            operators.push(char);
            tokens.push(new_token.clone());
            new_token = String::new();
            return;
        } else if char == '(' {
            new_token.push(char);
            parenthesis_counter += 1;
            return;
        } else if char == ')' {
            new_token.push(char);
            parenthesis_counter -= 1;
            return;
        }
        // Terminating the input
        else if char == '#' {
            tokens.push(new_token.clone());
            new_token = String::new();
            return;
        }
        new_token.push(char);
    });
    //
    // Printing out tokens and operators
    //
    // println!("\nTokens{:?}\nOperators: {:?}", tokens, operators);
    for op in 0..operators.len() {
        // Multiplication
        let mut result = 0_f64;
        if operators[op] == '*' {
            let nth_num = parse_to_number(tokens.get(op).unwrap().clone(), turtle);
            let nplusth_num = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result = nth_num * nplusth_num;
            tokens[op + 1] = result.to_string();
            tokens[op] = '#'.into();
            operators[op] = '#';
        }
    }
    tokens.retain(|t| t != "#");
    operators.retain(|&o| o != '#');
    for op in 0..operators.len() {
        // Float division
        let mut result = 0_f64;
        if operators[op] == '/' {
            let nth_num = parse_to_number(tokens.get(op).unwrap().clone(), turtle);
            let nplusth_num = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result = nth_num / nplusth_num;
            tokens[op + 1] = result.to_string();
            tokens[op] = '#'.into();
            operators[op] = '#';
        }
    }
    tokens.retain(|t| t != "#");
    operators.retain(|&o| o != '#');
    for op in 0..operators.len() {
        // Integer division
        let mut result = 0_f64;
        if operators[op] == ':' {
            let nth_num = parse_to_number(tokens.get(op).unwrap().clone(), turtle);
            let nplusth_num = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result = (nth_num / nplusth_num).floor();
            tokens[op + 1] = result.to_string();
            tokens[op] = '#'.into();
            operators[op] = '#';
        }
    }
    tokens.retain(|t| t != "#");
    operators.retain(|&o| o != '#');
    for op in 0..operators.len() {
        // Remainder
        let mut result = 0_f64;
        if operators[op] == '%' {
            let nth_num = parse_to_number(tokens.get(op).unwrap().clone(), turtle);
            let nplusth_num = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result = nth_num % nplusth_num;
            tokens[op + 1] = result.to_string();
            tokens[op] = '#'.into();
            operators[op] = '#';
        }
    }
    tokens.retain(|t| t != "#");
    operators.retain(|&o| o != '#');

    // Additions and subtraction
    let mut result: f64 = parse_to_number(tokens.first().unwrap().clone(), turtle);
    for op in 0..operators.len() {
        if operators[op] == '+' {
            let new_number = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result += new_number;
        } else if operators[op] == '-' {
            let new_number = parse_to_number(tokens.get(op + 1).unwrap().clone(), turtle);
            result -= new_number;
        }
    }
    //
    // Printing out tokens and operators
    //
    println!("\nTokens{:?}\nOperators: {:?}", tokens, operators);
    result
}

fn parse_to_number(raw_number: String, turtle: &mut Turtle) -> f64 {
    let mut value = 0_f64;
    if raw_number.contains(&['(', ')'][..]) {
        /*// Searching for the beginning of the parenthesis
        let parenthesis_begin_index: Vec<_> = raw_number.match_indices("(").collect();
        // We split it off the string after the "("
        let new_raw_number = raw_number.split_at(parenthesis_begin_index.first().unwrap().0 + 1);
        // We search for the last ")"" parenthesis and we split it off the string
        let parenthesis_end_index: Vec<_> = new_raw_number.1.match_indices(")").collect();
        let final_raw_number = new_raw_number
            .1
            .split_at(parenthesis_end_index.last().unwrap().0);*/
        let parenthesis_begin_index: Vec<_> = raw_number.match_indices("(").collect();
        let parenthesis_begin = raw_number.split_at(parenthesis_begin_index.first().unwrap().0 + 1);
        let parenthesis_begin_remainder = parenthesis_begin.1;
        let parenthesis_end_index: Vec<_> =
            parenthesis_begin_remainder.match_indices(")").collect();
        let parenthesis_end =
            parenthesis_begin_remainder.split_at(parenthesis_end_index.last().unwrap().0);
        let final_raw_number = parenthesis_end.0;
        value = parse_number_value(final_raw_number.to_string(), turtle);
    } else if turtle.variables.contains_key(&raw_number) {
        value = turtle
            .variables
            .get(&raw_number)
            .unwrap()
            .variable_type
            .get_value();
    } else {
        let value_result: Result<f64, _> = raw_number.parse();
        value = match value_result {
            Ok(val) => val,
            Err(_e) => {
                turtle.command_history.push(format!(
                    "A megadott bemenetet ({}) nem lehet kiszámolni!",
                    raw_number
                ));
                return f64::NAN;
            }
        }
    }
    value
}
