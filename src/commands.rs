use crate::documentation::Documentation;

pub struct Command {
    command: String,
    arguments: Vec<()>,
    flags: Vec<String>,
    documentation: Documentation,
}


