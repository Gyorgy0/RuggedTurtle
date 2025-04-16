use egui::ahash::HashMap;

use crate::{commands::Variable, parsing::trim_whitespace};

pub fn parse_number_value(input: String, variables: &mut HashMap<String, Variable>) -> f64 {
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
    //println!("\nTokens{:?}\nOperators: {:?}", tokens, operators);
    let mut result: f64 = parse_to_number(tokens.first().unwrap().clone(), variables);
    for i in 0..operators.len() {
        // Multiplication
        if operators[i] == '*' {
            let nth_num = parse_to_number(tokens.get(i).unwrap().clone(), variables);
            let nplusth_num = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result = nth_num * nplusth_num;
            tokens[i] = result.to_string();
            tokens.remove(i + 1);
            operators.remove(i);
        }
    }
    for i in 0..operators.len() {
        // Float division
        if operators[i] == '/' {
            let new_number = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            let nth_num = parse_to_number(tokens.get(i).unwrap().clone(), variables);
            let nplusth_num = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result = nth_num / nplusth_num;
        }
    }
    for i in 0..operators.len() {
        // Integer division
        if operators[i] == ':' {
            let nth_num = parse_to_number(tokens.get(i).unwrap().clone(), variables);
            let nplusth_num = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result = (nth_num / nplusth_num).floor();
        }
    }
    for i in 0..operators.len() {
        // Remainder
        if operators[i] == '%' {
            let nth_num = parse_to_number(tokens.get(i).unwrap().clone(), variables);
            let nplusth_num = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result = nth_num % nplusth_num;
        }
    }
    for i in 0..operators.len() {
        if operators[i] == '+' {
            let new_number = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result += new_number;
        } else if operators[i] == '-' {
            let new_number = parse_to_number(tokens.get(i + 1).unwrap().clone(), variables);
            result -= new_number;
        }
    }
    result
}

fn parse_to_number(raw_number: String, variables: &mut HashMap<String, Variable>) -> f64 {
    let mut value = 0_f64;
    if raw_number.contains(&['(', ')'][..]) {
        // Searching for the beginning of the parenthesis
        let parenthesis_begin_index: Vec<_> = raw_number.match_indices("(").collect();
        // We split it off the string after the "("
        let new_raw_number = raw_number
            .split_at(parenthesis_begin_index.first().unwrap().0 + 1)
            .1
            .to_string();
        // We search for the last ")"" parenthesis and we split it off the string
        let parenthesis_end_index: Vec<_> = raw_number.match_indices(")").collect();
        let final_raw_number = new_raw_number
            .split_at(parenthesis_end_index.last().unwrap().0 - 1)
            .0
            .to_string();
        value = parse_number_value(final_raw_number, variables);
    } else if variables.contains_key(&raw_number) {
        value = variables
            .get(&raw_number)
            .unwrap()
            .variable_type
            .get_value();
    } else {
        value = raw_number.parse().unwrap();
    }
    value
}
