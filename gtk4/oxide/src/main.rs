mod ui;
mod app;


use gtk::{
    prelude::*,
    gio
};

use app::Application;


fn main() {
    env_logger::init();

    gio::resources_register_include!("org.devphilplus.oxide.gresource")
        .expect("failed to register resources");

    let app = Application::default();
    app.run();
}
