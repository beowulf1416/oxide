use log::{
    debug,
    error
};

use std::rc::Rc;
use std::fs;
use std::error::Error;

use relm4::prelude::*;
use relm4::actions::*;


use crate::app::Workspace;
use crate::actions::WindowActionGroup;
use crate::components::main_window::MainWindow;
use crate::application_message::ApplicationMessage;


relm4::new_stateless_action!(pub WorkspaceSaveAction, WindowActionGroup, "workspace-save");


pub fn workspace_save_action(sender: Rc<ComponentSender<MainWindow>>, workspace: Rc<Workspace>) -> RelmAction<WorkspaceSaveAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("WorkspaceSaveAction action triggered");
        if let Err(e) = workspace.save() {
            error!("workspace save error: {:?}", e);
        }
    });
}