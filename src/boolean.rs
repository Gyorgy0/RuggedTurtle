use crate::parsing::trim_whitespace;

pub fn parse_boolean_value(input: String) -> bool {
    // Removes whitespaces
    let trimmed_input = trim_whitespace(input.as_str());
    let parenthesis_blocks: Vec<&str> = vec![];
    false
}
