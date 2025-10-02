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


relm4::new_stateless_action!(pub NewSQLFileAction, WindowActionGroup, "new-sql-file");


pub fn new_sql_file_action(_sender: Rc<ComponentSender<MainWindow>>) -> RelmAction<NewSQLFileAction> {
    return RelmAction::new_stateless(move |_| {
        debug!("NewSQLFileAction action triggered");
    });
}