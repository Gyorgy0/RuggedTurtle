use crate::documentation::Documentation;
struct Condition {
    statement: Vec<()>,
    value: bool,
    block: Vec<()>,
    documentation: Documentation,
}

struct Cycle {
    rangefrom: i64,
    rangeto: i64,
    block: Vec<()>,
    documentation: Documentation,
}

struct Loop {
    condition: Condition,
    block: Vec<()>,
    documentation: Documentation,
}
