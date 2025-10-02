use log::{
    debug
};

use std::rc::Rc;
use std::fs;
use std::error::Error;

use relm4::prelude::*;
use relm4::actions::*;
// use relm4_components::open_dialog::{
//     OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
// };

use rfd::FileDialog;

use walkdir::WalkDir;

// use crate::app::Workspace;
use crate::structs::{
    workspace::Workspace,
    node::Node
};
use crate::actions::WindowActionGroup;
use crate::components::main_window::MainWindow;
use crate::application_message::ApplicationMessage;



relm4::new_stateless_action!(pub WorkspaceOpenAction, WindowActionGroup, "workspace-open");

pub fn workspace_open_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<WorkspaceOpenAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("WorkspaceOpenAction action triggered");

        match FileDialog::new()
            // .set_parent(&handle)
            .set_title("Select a folder")
            .pick_folder() {
                Some(folder) => {
                    // debug!("Selected folder: {:?}", folder);        

                    let root_path = folder.as_path();
                    // let root_path_buf = path.into_path();

                    let name = root_path.file_name().unwrap().to_str().unwrap();
                    let parent = root_path.parent().unwrap().to_str().unwrap();

                    let mut ws = Workspace::new(
                        name,
                        parent
                    );

                    for result in walkdir::WalkDir::new(root_path).min_depth(1).max_depth(1) {
                        // debug!("entry: {:?}", result);

                        let entry = result.unwrap();
                        // let entry_path = entry.path();
                        
                        let name = entry.file_name().to_str().unwrap();
                        let node = Node::new(
                            name,
                            root_path.to_str().unwrap()
                        );

                        // root.child_add(node);
                        ws.child_add(&node);
                    }

                    sender.input(ApplicationMessage::WorkspaceOpen(ws));
                },
                None => {
                    debug!("No folder selected");
                }
        }
    });
}