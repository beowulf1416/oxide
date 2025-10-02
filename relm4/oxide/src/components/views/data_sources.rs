use log::{
    debug,
    error
};
use std::rc::Rc;
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

use crate::application_message::ApplicationMessage;


pub struct DataSourcesView {

}



#[relm4::component(pub)]
impl SimpleComponent for DataSourcesView {
    type Input = ();
    type Output = ApplicationMessage;
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::Label {
                set_label: "Data Sources",
                set_halign: gtk::Align::Start
            },

            gtk::ActionBar {
                set_hexpand: true,
                set_valign: gtk::Align::End,

                pack_start = &gtk::Button {
                    set_label: "Add Datasource",
                    set_icon_name: "file-cabinet",
                    set_action_name: Some("win.data-source-add")
                },

            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DataSourcesView{};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
