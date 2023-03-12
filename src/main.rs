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
use crate::state::State;

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
    let label = Label::builder()
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

    grid.attach(&label, 0, 0, 3, 1);

    grid.attach(&number_button('1', state.clone()), 0, 1, 1, 1);
    grid.attach(&number_button('2', state.clone()), 1, 1, 1, 1);
    grid.attach(&number_button('3', state.clone()), 2, 1, 1, 1);

    grid.attach(&number_button('4', state.clone()), 0, 2, 1, 1);
    grid.attach(&number_button('5', state.clone()), 1, 2, 1, 1);
    grid.attach(&number_button('6', state.clone()), 2, 2, 1, 1);

    grid.attach(&number_button('7', state.clone()), 0, 3, 1, 1);
    grid.attach(&number_button('8', state.clone()), 1, 3, 1, 1);
    grid.attach(&number_button('9', state.clone()), 2, 3, 1, 1);

    grid.attach(&number_button('0', state.clone()), 0, 4, 2, 1);
    grid.attach(&number_button('.', state.clone()), 2, 4, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TermyCalc")
        .child(&grid)
        .build();

    window.set_default_size(400, 400);
    window.present();
}

fn number_button(
    digit: char,
    state: Rc<RefCell<State>>,
) -> Button {
    let button = Button::builder()
        .label(format!("{}", digit))
        .margin_top(BUTTON_MARGIN)
        .margin_bottom(BUTTON_MARGIN)
        .margin_start(BUTTON_MARGIN)
        .margin_end(BUTTON_MARGIN)
        .build();

    button.connect_clicked(move |_| {
        let x = &mut *state.borrow_mut();
        x.append(digit)
    });

    return button
}
