mod components;
mod actions;
mod app;
mod application_message;

use log::{
    debug
};
use serde::{Deserialize, Serialize};

use gtk::{
    prelude::*,
    gio
};

use relm4::{
    prelude::*,
    ComponentParts,
    ComponentSender,
    SimpleComponent
};
use relm4::actions::*;


use crate::components::main_window::MainWindow;




fn main() {
    env_logger::init();

    gio::resources_register_include!("org.devphilplus.oxide.gresource")
        .expect("failed to register resources");


    let app = RelmApp::new("org.devphilplus.oxide");

    app.run::<MainWindow>(());
}
