use std::fmt::{Display, Formatter};

pub enum Mode {
    Clear,
    Append,
}

impl Display for Mode {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match *self {
            Mode::Clear => f.write_str("Clear"),
            Mode::Append => f.write_str("Append"),
        }
    }
}
