use log::{
    debug
};

use gtk::{
    prelude::*,
    glib,
    glib::clone,
    gio,
};


use crate::ui::windows::main_window::MainWindow;
use crate::ui::windows::preferences::PreferencesWindow;


pub fn preferences_action(
    window: &MainWindow
) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("preferences", None);
    action.connect_activate(clone!(
        #[weak]
        window,
        move |_, _| {
            let app = window.application().unwrap();

            let pref_window = PreferencesWindow::new(&app);
            pref_window.set_transient_for(Some(&window));
            pref_window.present();
        }
    ));
    return action;
}