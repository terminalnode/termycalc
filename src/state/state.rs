use crate::state::{Mode, Operation};

pub struct State {
    display: String,
    first_input: bool,
    first_operand: f64,
    second_operand: f64,
    operation: Operation,
    mode: Mode,
}

impl State {
    pub fn default() -> Self {
        State {
            display: String::from("Start pressing buttons :-)"),
            first_input: true,
            first_operand: 0.0,
            second_operand: 0.0,
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
        number: i32,
    ) {
        // TODO make operands into string and actually append, lol
        if self.first_input { self.first_operand += number as f64 }
        else { self.second_operand += number as f64 }
        println!("first_operand: {}", self.first_operand);
        println!("second_operand: {}", self.second_operand);
    }

    pub fn calculate(
        &mut self,
    ) {
        self.first_operand = match self.operation {
            Operation::Addition => { self.first_operand + self.second_operand }
            Operation::Multiplication => { self.first_operand * self.second_operand }
            Operation::Subtraction => { self.first_operand - self.second_operand }
            Operation::Division => {
                if self.second_operand == 0.0 { 0.0 }
                else { self.first_operand / self.second_operand }
            }
        };
        self.second_operand = 0.0;
        self.operation = Operation::Addition;
        println!("first_input: {}", self.first_input);
        println!("first_operand: {}", self.first_operand);
        println!("second_operand: {}", self.second_operand);
        println!("operation: {}", self.operation);
        println!("mode: {}", self.mode);
    }
}
