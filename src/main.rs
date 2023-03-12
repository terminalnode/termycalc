mod state;

use std::cell::RefCell;
use std::num::ParseIntError;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use crate::state::State;

const APP_ID: &str = "xyz.terminalnode.TermyCalc";

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
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TermyCalc")
        .child(&number_button(1, state.clone()))
        .child(&number_button(2, state.clone()))
        .child(&number_button(3, state.clone()))
        .build();
    window.present();
}

fn number_button(
    number: i32,
    state: Rc<RefCell<State>>,
) -> Button {
    let button = Button::builder()
        .label(format!("{}", number))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |_| {
        let mut x = &mut *state.borrow_mut();
        x.append(number)
    });

    return button
}
