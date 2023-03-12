use std::fmt::{Display, Formatter};

pub enum Operation {
    Addition,
    Multiplication,
    Subtraction,
    Division,
}

impl Display for Operation {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match *self {
            Operation::Addition => f.write_str("Addition"),
            Operation::Multiplication => f.write_str("Multiplication"),
            Operation::Subtraction => f.write_str("Subtraction"),
            Operation::Division => f.write_str("Division"),
        }
    }
}
