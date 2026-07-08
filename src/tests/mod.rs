#[cfg(test)]
mod tests {
    use std::any::type_name_of_val;

    use crate::{
        arithmetic::parse_number_value,
        commands::{Variable, VariableTypes},
        locale::Locale,
        turtle::Turtle,
    };

    #[test]
    fn pemdas_positive() {
        // Setting up enviroment
        let locale = vec![Locale::default()];
        let mut turtle = Turtle::default();

        let expression = String::from("6-10/5");
        assert_eq!(
            4_f64,
            parse_number_value(expression, &mut turtle, &locale, 0_usize)
        );
        let expression = String::from("25-8*2+(3*3)");
        assert_eq!(
            18_f64,
            parse_number_value(expression, &mut turtle, &locale, 0_usize)
        );
        let expression = String::from("(10+5*5)*((5*(0-2))+9-3*3*3)/2");
        assert_eq!(
            -490_f64,
            parse_number_value(expression, &mut turtle, &locale, 0_usize)
        );
    }
    #[test]
    fn pemdas_variables() {
        // Setting up enviroment
        let locale = vec![Locale::default()];
        let mut turtle = Turtle::default();

        let a = 7_f64;
        let b = 4_f64;
        turtle.variables.insert(
            String::from("a"),
            Variable {
                raw_value: String::from(a.to_string()),
                variable_type: VariableTypes::Number { value: a },
                writable: false,
            },
        );
        turtle.variables.insert(
            String::from("b"),
            Variable {
                raw_value: b.to_string(),
                variable_type: VariableTypes::Number { value: b },
                writable: false,
            },
        );
        let expression = String::from("3*a-(2*2*2)/b");
        assert_eq!(
            19_f64,
            parse_number_value(expression, &mut turtle, &locale, 0_usize)
        );

        let c = 2_f64;
        let d = 5_f64;
        turtle.variables.insert(
            String::from("c"),
            Variable {
                raw_value: c.to_string(),
                variable_type: VariableTypes::Number { value: c },
                writable: false,
            },
        );
        turtle.variables.insert(
            String::from("d"),
            Variable {
                raw_value: String::from(d.to_string()),
                variable_type: VariableTypes::Number { value: d },
                writable: false,
            },
        );
        let expression = String::from("4*c-2*d/c");
        assert_eq!(
            3_f64,
            parse_number_value(expression, &mut turtle, &locale, 0_usize)
        );
    }

    /// Negative numbers NEED TO BE IMPLEMENTED!!!
    #[test]
    fn pemdas_negative() {
        let expression = String::from("(10+5*5)*((5*(-2))+9-3*3*3)/2");
        assert_eq!(
            -490_f64,
            parse_number_value(
                expression,
                &mut Turtle::default(),
                &vec![Locale::default()],
                0_usize
            )
        );
    }
}
