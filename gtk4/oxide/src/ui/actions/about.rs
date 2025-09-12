use log::{
    debug
};

use gtk::{
    prelude::*,
    subclass::prelude::*,
    glib,
    glib::clone,
    gio,
};


use crate::ui::windows::main_window::MainWindow;
use crate::ui::windows::about_window::AboutWindow;


pub fn about_action(
    window: &MainWindow
) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("about", None);
    action.connect_activate(clone!(
        #[weak]
        window,
        move |_, _| {
            debug!("//TODO: about action");

            let app = window.application().unwrap();
            // let obj = window.obj();

            let about_window = AboutWindow::new(&app);
            about_window.set_transient_for(Some(&window));
            about_window.present();
        }
    ));
    return action;
}