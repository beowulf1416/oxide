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


pub struct WorkspaceView {

}


impl Default for WorkspaceView {
    fn default() -> Self {
        return Self {
        };
    }
}


// #[derive(Debug)]
// pub enum WorkspaceViewMessage {
//     FolderAdd
// }


#[relm4::component(pub)]
impl SimpleComponent for WorkspaceView {
    type Input = ();
    type Output = ApplicationMessage;
    type Init = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 0,

            gtk::Label {
                set_label: "Workspace",
                set_halign: gtk::Align::Start
            },

            gtk::ActionBar {
                set_hexpand: true,
                set_valign: gtk::Align::End,

                pack_start = &gtk::Button {
                    set_label: "Open Workspace",
                    set_icon_name: "document-open",
                    set_action_name: Some("win.workspace-open")

                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WorkspaceView{};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    //     // match msg {
    //     //     // WorkspaceViewMessage::FolderAdd => {
    //     //     //     debug!("//TODO: WorkspaceViewMessage::FolderAdd");
    //     //     // }
    //     // }
    // }
}