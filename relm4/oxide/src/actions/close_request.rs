use log::{
    debug
};

use std::rc::Rc;

use relm4::prelude::*;
use relm4::actions::*;

use crate::actions::WindowActionGroup;
use crate::components::main_window::MainWindow;
use crate::application_message::ApplicationMessage;



relm4::new_stateless_action!(pub CloseRequestAction, WindowActionGroup, "close-request");

pub fn close_request_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<CloseRequestAction> {
    debug!("Creating close request action");
    return RelmAction::new_stateless(move |_| {
        debug!("Close request action triggered");
        sender.input(ApplicationMessage::CloseRequest);
    });
}