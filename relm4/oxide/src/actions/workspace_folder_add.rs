use log::{
    debug
};

use std::rc::Rc;
use std::fs;
use std::path::Path;
use std::error::Error;

use relm4::prelude::*;
use relm4::actions::*;
// use relm4_components::open_dialog::{
//     OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
// };

use rfd::FileDialog;

use walkdir::WalkDir;

use crate::app::{
    Workspace,
    RootNode,
    Node
};
use crate::actions::WindowActionGroup;
use crate::components::main_window::MainWindow;
use crate::application_message::ApplicationMessage;




relm4::new_stateless_action!(pub WorkspaceFolderAddAction, WindowActionGroup, "workspace-folder-add");

pub fn workspace_folder_add_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<WorkspaceFolderAddAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("WorkspaceFolderAddAction action triggered");

        match FileDialog::new()
            // .set_parent(&handle)
            .set_title("Select a folder")
            .pick_folder() {
                Some(folder) => {
                    debug!("Selected folder: {:?}", folder);

                    let path = folder.as_path();

                    let mut rn = RootNode::new(
                        &path.file_name().unwrap().to_str().unwrap(), 
                        &path.parent().unwrap().to_str().unwrap()
                    );

                    for entry in walkdir::WalkDir::new(path).min_depth(1).max_depth(3) {
                        debug!("entry: {:?}", entry);
                        // debug!("depth: {}", entry.depth());
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            let node = Node::new(
                                &path.file_name().unwrap().to_str().unwrap(),
                                &path.parent().unwrap().to_str().unwrap()
                            );
                            rn.children_push(&node);
                        }
                    }

                    sender.input(ApplicationMessage::WorkspaceFolderAdd(rn));
                },
                None => {
                    debug!("No folder selected");
                }
        }
    });
}