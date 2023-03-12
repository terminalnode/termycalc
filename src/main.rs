mod state;

use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{
    glib::ExitCode,
    Application,
    ApplicationWindow,
    Button,
    Grid,
    Label,
};
use crate::state::{Operation, State};

const APP_ID: &str = "xyz.terminalnode.TermyCalc";
const BUTTON_MARGIN: i32 = 3;
const GRID_MARGIN: i32 = 3;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    let state = Rc::new(RefCell::new(State::default()));

    app.connect_activate(move |app| build_ui(app, state.clone()));
    app.run()
}

fn build_ui(
    app: &Application,
    state: Rc<RefCell<State>>,
) {
    let display = Label::builder()
        .label(state.borrow().get_display())
        .build();

    let grid = Grid::builder()
        .margin_start(GRID_MARGIN)
        .margin_end(GRID_MARGIN)
        .margin_top(GRID_MARGIN)
        .margin_bottom(GRID_MARGIN)
        .row_spacing(GRID_MARGIN)
        .column_spacing(GRID_MARGIN)
        .build();

    grid.attach(&display, 0, 0, 4, 1);

    // Operation buttons
    grid.attach(&operation_button(Operation::Addition, state.clone()), 0, 1, 1, 1);
    grid.attach(&operation_button(Operation::Subtraction, state.clone()), 1, 1, 1, 1);
    grid.attach(&operation_button(Operation::Multiplication, state.clone()), 2, 1, 1, 1);
    grid.attach(&operation_button(Operation::Division, state.clone()), 3, 1, 1, 1);
    grid.attach(&button(format!("C\nL\nR")), 3, 4, 1, 2);
    grid.attach(&button(format!("=")), 3, 2, 1, 2);

    // Number buttons
    grid.attach(&number_button('1', state.clone()), 0, 2, 1, 1);
    grid.attach(&number_button('2', state.clone()), 1, 2, 1, 1);
    grid.attach(&number_button('3', state.clone()), 2, 2, 1, 1);
    grid.attach(&number_button('4', state.clone()), 0, 3, 1, 1);
    grid.attach(&number_button('5', state.clone()), 1, 3, 1, 1);
    grid.attach(&number_button('6', state.clone()), 2, 3, 1, 1);
    grid.attach(&number_button('7', state.clone()), 0, 4, 1, 1);
    grid.attach(&number_button('8', state.clone()), 1, 4, 1, 1);
    grid.attach(&number_button('9', state.clone()), 2, 4, 1, 1);
    grid.attach(&number_button('0', state.clone()), 0, 5, 2, 1);
    grid.attach(&number_button('.', state.clone()), 2, 5, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TermyCalc")
        .child(&grid)
        .build();

    window.set_default_size(400, 400);
    window.present();
}

fn button(
    label: String,
) -> Button {
    Button::builder()
        .label(label)
        .margin_top(BUTTON_MARGIN)
        .margin_bottom(BUTTON_MARGIN)
        .margin_start(BUTTON_MARGIN)
        .margin_end(BUTTON_MARGIN)
        .build()
}

fn number_button(
    digit: char,
    state: Rc<RefCell<State>>,
) -> Button {
    let button = button(format!("{}", digit));
    button.connect_clicked(move |_| {
        let x = &mut *state.borrow_mut();
        x.append(digit)
    });

    return button
}

fn operation_button(
    operation: Operation,
    state: Rc<RefCell<State>>,
) -> Button {
    let button = button(format!("{}", operation.to_char()));

    button.connect_clicked(move |_| {
        let x = &mut *state.borrow_mut();
        x.set_operation(operation.clone())
    });

    return button
}
