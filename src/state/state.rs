use crate::state::{Mode, Operation};

pub struct State {
    display: String,
    first_input: bool,
    first_operand: String,
    second_operand: String,
    operation: Operation,
    mode: Mode,
}

impl State {
    pub fn default() -> Self {
        State {
            display: String::from("Start pressing buttons :-)"),
            first_input: true,
            first_operand: String::from("0"),
            second_operand: String::from("0"),
            operation: Operation::Addition,
            mode: Mode::Clear,
        }
    }

    pub fn get_display(
        &self,
    ) -> &str {
        &self.display
    }

    pub fn append(
        &mut self,
        digit: char,
    ) {
        if self.first_input { self.first_operand = format!("{}{}", self.first_operand, digit) }
        else { self.second_operand = format!("{}{}", self.second_operand, digit) }
        println!("first_operand: {}", self.first_operand);
        println!("second_operand: {}", self.second_operand);
    }

    pub fn set_operation(
        &mut self,
        operation: Operation,
    ) {
        self.operation = operation;
        self.first_input = false;
    }

    // TODO safe unwrap
    fn first_operand_as_f64(&self) -> Result<f64, String> {
        self.first_operand.parse::<f64>()
            .or_else(|_| Err(format!("Could not parse first operand as f64")))
    }

    // TODO safe unwrap
    fn second_operand_as_f64(&self) -> Result<f64, String> {
        self.second_operand.parse::<f64>()
            .or_else(|_| Err(format!("Could not parse second operand as f64")))
    }

    pub fn calculate(
        &mut self,
    ) {
        let first = self.first_operand_as_f64();
        let second = self.second_operand_as_f64();
        match match self.operation {
            Operation::Addition => { first.and_then(|f| second.map(|s| f + s)) }
            Operation::Multiplication => { first.and_then(|f| second.map(|s| f * s)) }
            Operation::Subtraction => { first.and_then(|f| second.map(|s| f - s)) }
            Operation::Division => {
                Ok(0.0) // TODO can't be bothered with rust error handling atm lol
                // if self.second_operand == "0.0" { Ok(0.0) }
                // else { Ok(first.and_then(|f| second.map(|s| f / s))) }
            }
        } {
            Ok(result) => {
                let result = format!("{}", result);
                self.display = result.clone();
                self.first_operand = result;
                self.second_operand = String::from("0");
            },
            Err(error) => {
                self.display = format!("Error! {}", error);
                self.first_operand = String::from("0");
                self.second_operand = String::from("0");
            },
        };

        println!("first_input: {}", self.first_input);
        println!("first_operand: {}", self.first_operand);
        println!("second_operand: {}", self.second_operand);
        println!("operation: {}", self.operation);
        println!("mode: {}", self.mode);
    }
}
