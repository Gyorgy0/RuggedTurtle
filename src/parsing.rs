// Removes whitespace characters from the string
pub fn trim_whitespace(s: &str) -> String {
    let mut new_str: Vec<&str> = s.split_whitespace().collect();
    new_str.retain(|&x| !x.is_empty());

    new_str.join("").to_string()
}
