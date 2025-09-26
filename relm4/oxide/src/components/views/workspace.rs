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


#[derive(Debug)]
pub enum WorkspaceActions {
    Open,
    Save,
    FolderAdd
}


pub struct WorkspaceView {
    path: String
}


impl Default for WorkspaceView {
    fn default() -> Self {
        return Self {
            path: String::from("")
        };
    }
}


#[relm4::component(pub)]
impl SimpleComponent for WorkspaceView {
    type Input = WorkspaceActions;
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
            },

            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                gtk::ColumnView {
                    set_hexpand: true,
                    set_vexpand: true,
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = WorkspaceView {
            path: String::from("")
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            WorkspaceActions::Open => {
                debug!("WorkspaceActions::Open update");
            }
            WorkspaceActions::Save => {
                debug!("WorkspaceActions::Save update");
            }
            WorkspaceActions::FolderAdd => {
                debug!("WorkspaceActions::FolderAdd update");
            }
        }
    }
}