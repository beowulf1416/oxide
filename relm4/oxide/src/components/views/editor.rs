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


pub struct EditorView {

}



#[relm4::component(pub)]
impl SimpleComponent for EditorView {
    type Input = ();
    type Output = ApplicationMessage;
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::Label {
                set_label: "Editor",
                set_halign: gtk::Align::Start
            },

            gtk::Notebook {
                set_tab_pos: gtk::PositionType::Top,
                set_scrollable: true,

                append_page[Some(&gtk::Label::new(Some("page 1")))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 0,

                    gtk::Label {
                        set_label: "Page 1",
                        set_halign: gtk::Align::Start
                    },
                },

                append_page[Some(&gtk::Label::new(Some("page 2")))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 0,

                    gtk::Label {
                        set_label: "Page 2",
                        set_halign: gtk::Align::Start
                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = EditorView{};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}