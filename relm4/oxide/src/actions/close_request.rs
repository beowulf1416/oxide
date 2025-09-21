use log::{
    debug
};

use relm4::prelude::*;
use relm4::actions::*;

use crate::actions::WindowActionGroup;
use crate::App;



relm4::new_stateless_action!(CloseRequestAction, WindowActionGroup, "close-request");

pub type CloseRequestRelmAction = RelmAction<CloseRequestAction>;

pub fn close_request_action(sender: &ComponentSender<App>) -> CloseRequestRelmAction {
    debug!("Creating close request action");
    return RelmAction::new_stateless(move |_| {
        debug!("Close request action triggered");
        // relm4::emit_global(ApplicationMessage::CloseRequest);
    });
}