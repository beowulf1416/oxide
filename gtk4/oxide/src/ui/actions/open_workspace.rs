use log::{
    debug,
    error
};

use std::io::prelude::*;
use std::fs::{
    self,
    File
};

use gtk::{
    prelude::*,
    glib,
    glib::clone,
    gio,
};

use rfd::FileDialog;


use crate::ui::windows::main_window::MainWindow;


pub fn open_workspace_action(
    window: &MainWindow
) -> gio::SimpleAction {
    let action = gio::SimpleAction::new("workspace.open", None);
    action.connect_activate(clone!(
        #[weak]
        window,
        move |_, _| {
            debug!("Workspace Open action activated");

            // let gtk_window = window.downcast_ref::<gtk::Window>().unwrap();
            // let handle = gtk_window.native().unwrap();

            match FileDialog::new()
                // .set_parent(handle)
                .set_title("Select a folder")
                .pick_folder() {
                    Some(folder) => {
                        debug!("Selected folder: {:?}", folder);
                        load_workspace(folder.to_str().unwrap());

                        // window.open_workspace(folder.to_str().unwrap());
                    },
                    None => {
                        debug!("No folder selected");
                    }
            }
        }
    ));
    return action;
}

fn load_workspace(path: &str) {
    debug!("Loading workspace from path: {}", path);

    match fs::read_to_string(format!("{}/workspace.json", path)) {
        Ok(content) => {
            debug!("Workspace content: {}", content);
        }
        Err(e) => {
            // error!("Failed to read workspace file: {}", e);
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    debug!("Workspace file not found, creating a new one.");

                    match File::create(format!("{}/workspace.json", path)) {
                        Ok(mut file) => {
                            let initial_content = r#"{}"#;
                            file.write_all(initial_content.as_bytes()).unwrap();
                        }
                        Err(e) => {
                            error!("Failed to create workspace file: {}", e);
                        }
                    }
                }
                _ => {
                    error!("Failed to read workspace file: {}", e);
                }
            }
        }
    }
}