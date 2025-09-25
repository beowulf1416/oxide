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


relm4::new_stateless_action!(pub AboutAction, WindowActionGroup, "about");


pub fn new_text_file_action(sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<AboutAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("AboutAction action triggered");
    });
}