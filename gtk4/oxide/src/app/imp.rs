use log::{
    debug
};

use gtk::{
    prelude::*,
    subclass::prelude::*,
    glib,
};



use crate::ui::windows::main_window::MainWindow;


#[derive(Debug, Default)]
pub struct Application {

}


impl Application {

}


#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Silo";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {}

impl ApplicationImpl for Application {
    fn activate(&self) {
        self.parent_activate();

        // let app = self.app.clone();
        // debug!("app: {:?}", app);

        let mw = MainWindow::new(&*self.obj());

        // self.setup_actions();


        mw.present();
    }
}

impl GtkApplicationImpl for Application {}