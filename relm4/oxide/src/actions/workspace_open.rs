use log::{
    debug
};

use std::rc::Rc;

use relm4::prelude::*;
use relm4::actions::*;

use crate::actions::WindowActionGroup;
use crate::components::main_window::MainWindow;
use crate::application_message::ApplicationMessage;



relm4::new_stateless_action!(pub WorkspaceOpenAction, WindowActionGroup, "workspace-open");

pub fn workspace_open_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<WorkspaceOpenAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("WorkspaceOpenAction action triggered");
        sender.input(ApplicationMessage::WorkspaceOpen);
    });
}