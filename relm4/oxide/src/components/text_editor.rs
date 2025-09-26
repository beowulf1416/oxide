use gtk::{glib, prelude::*};
use relm4::prelude::*;

use crate::application_message::ApplicationMessage;
use crate::actions::*;


#[relm4::widget_template(pub)]
impl WidgetTemplate for TextEditor {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                gtk::TextView {

                }
            }
        }
    }
}


