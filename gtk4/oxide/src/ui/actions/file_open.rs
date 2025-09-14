use log::{
    debug
};

use gtk::{
    prelude::*,
    glib,
    glib::clone,
    gio,
};

use rfd::FileDialog;


use crate::ui::windows::main_window::MainWindow;


pub fn file_open_action(
    window: &MainWindow
) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("file.open", None);
    action.connect_activate(clone!(
        #[weak]
        window,
        move |_, _| {
            debug!("File Open action activated");

            match FileDialog::new()
                // .set_parent(&window)
                .set_title("Select a folder")
                .pick_folder() {
                    Some(folder) => {
                        debug!("Selected folder: {:?}", folder);
                    },
                    None => {
                        debug!("No folder selected");
                    }
            }
        }
    ));
    return action;
}