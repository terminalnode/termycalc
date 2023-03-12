mod state;

use std::cell::RefCell;
use std::num::ParseIntError;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Grid};
use crate::state::State;

const APP_ID: &str = "xyz.terminalnode.TermyCalc";
const BUTTON_MARGIN: i32 = 3;
const GRID_MARGIN: i32 = 3;

fn main() -> glib::ExitCode {
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
    let grid = Grid::builder()
        .margin_start(GRID_MARGIN)
        .margin_end(GRID_MARGIN)
        .margin_top(GRID_MARGIN)
        .margin_bottom(GRID_MARGIN)
        .row_spacing(GRID_MARGIN)
        .column_spacing(GRID_MARGIN)
        .build();
    grid.attach(&number_button(1, state.clone()), 0, 0, 1, 1);
    grid.attach(&number_button(2, state.clone()), 1, 0, 1, 1);
    grid.attach(&number_button(3, state.clone()), 2, 0, 1, 1);

    grid.attach(&number_button(4, state.clone()), 0, 1, 1, 1);
    grid.attach(&number_button(5, state.clone()), 1, 1, 1, 1);
    grid.attach(&number_button(6, state.clone()), 2, 1, 1, 1);

    grid.attach(&number_button(7, state.clone()), 0, 2, 1, 1);
    grid.attach(&number_button(8, state.clone()), 1, 2, 1, 1);
    grid.attach(&number_button(9, state.clone()), 2, 2, 1, 1);

    grid.attach(&number_button(0, state.clone()), 0, 3, 2, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("TermyCalc")
        .child(&grid)
        .build();

    window.set_default_size(400, 400);
    window.present();
}

fn number_button(
    number: i32,
    state: Rc<RefCell<State>>,
) -> Button {
    let button = Button::builder()
        .label(format!("{}", number))
        .margin_top(BUTTON_MARGIN)
        .margin_bottom(BUTTON_MARGIN)
        .margin_start(BUTTON_MARGIN)
        .margin_end(BUTTON_MARGIN)
        .build();

    button.connect_clicked(move |_| {
        let mut x = &mut *state.borrow_mut();
        x.append(number)
    });

    return button
}
