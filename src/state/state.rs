use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::state::{Mode, Operation};

pub struct State {
    display: String,
    display_listeners: Vec<Rc<RefCell<dyn Fn(&str)>>>,
    result: Option<f64>,
    input: String,
    operation: Operation,
    mode: Mode,
}

impl Debug for State {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "State {{\n\tdisplay: {},\n\tresult: {:?},\n\tinput: {},\n\toperation: {},\n\tmode: {}\n}}",
            self.display,
            self.result,
            self.input,
            self.operation,
            self.mode,
        )
    }
}

impl State {
    pub fn default() -> Self {
        State {
            display: String::from("Start pressing buttons :-)"),
            display_listeners: Vec::with_capacity(1),
            result: None,
            input: String::from("0"),
            operation: Operation::Addition,
            mode: Mode::Clear,
        }
    }

    // region internal
    fn set_display(
        &mut self,
        display: String,
    ) {
        self.display = display;
        for listener in &self.display_listeners {
            listener.borrow()(&self.display.clone())
        }
    }
    // endregion internal

    pub fn get_display(&self) -> &str {
        &self.display
    }

    pub fn add_display_listener(
        &mut self,
        listener: Rc<RefCell<dyn Fn(&str)>>,
    ) {
        self.display_listeners.push(listener);
    }

    pub fn append(
        &mut self,
        digit: char,
    ) {
        if self.input == "0" && (digit == '.' || digit == ',') {
            self.input.push('.')
        } else if self.input == "0" {
            self.input = String::from(digit);
        } else {
            self.input.push(digit);
        }

        self.set_display(self.input.clone());
    }

    pub fn set_operation(
        &mut self,
        operation: Operation,
    ) {
        self.operation = operation;
    }

    fn result_as_f64(&self) -> Result<f64, String> {
        self.result.ok_or(String::from("Result is None"))
    }

    fn input_as_f64(&self) -> Result<f64, String> {
        self.input.parse::<f64>()
            .or_else(|_| Err(format!("Could not parse first operand as f64")))
    }

    pub fn calculate(&mut self) {
        // TODO safe unwrap lol
        let first = self.result_as_f64().unwrap();
        let second = self.input_as_f64().unwrap();

        let result = match self.operation {
            Operation::Addition => Ok(first + second),
            Operation::Multiplication => Ok(first * second),
            Operation::Subtraction => Ok(first - second),
            Operation::Division => match second {
                0.0 => { Err("Division by zero!".to_string()) }
                _ => { Ok(first / second) }
            }
        };

        match result {
            Ok(result) => {
                self.display = format!("{}", result);
                self.result = Some(result);
                self.input = String::from("0");
            },
            Err(error) => {
                self.display = format!("Error! {}", error);
                self.result = None;
                self.input = String::from("0");
                self.operation = Operation::Addition;
            },
        };

        println!("{:?}", self);
    }
}
