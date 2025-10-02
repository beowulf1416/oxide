use log::{
    debug,
    error
};
use std::rc::Rc;
use std::convert::identity;
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


use crate::app::{
    App,
    Workspace
};
use crate::application_message::ApplicationMessage;
use crate::actions::*;
use crate::components::*;


pub struct AboutWindow {

}



#[relm4::component(pub)]
impl SimpleComponent for AboutWindow {
    type Input = ApplicationMessage;
    type Output = ();
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::Button {
                set_label: "Ok"
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AboutWindow {};
        let widgets = view_output!();

        ComponentParts { 
            widgets, 
            model
        }
    }
}