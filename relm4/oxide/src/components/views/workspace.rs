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
    SimpleComponent,
    typed_view::column::{
        LabelColumn,
        TypedColumnView
    }
};

use crate::application_message::ApplicationMessage;
use crate::structs::{
    workspace::Workspace,
    node::Node
};


#[derive(Debug)]
pub enum WorkspaceActions {
    Open(Workspace),
    Save,
    FolderAdd
}


#[derive(Debug, PartialEq, Eq)]
struct TreeItem {
    name: String,
    value: String
}


impl TreeItem {
    pub fn new(
        name: &str,
        value: &str
    ) -> Self {
        return Self {
            name: String::from(name),
            value: String::from(value)
        };
    }
}



struct NameColumn {

}

impl LabelColumn for NameColumn {
    type Item = TreeItem;
    type Value = String;

    const COLUMN_NAME: &'static str = "Name";

    const ENABLE_SORT: bool = true;
    const ENABLE_RESIZE: bool = true;

    fn get_cell_value(item: &Self::Item) -> Self::Value {
        debug!("get_cell_value: {:?}", item);
        return item.name.clone();
    }

    fn format_cell_value(value: &Self::Value) -> String {
        debug!("format_cell_value: {:?}", value);
        return value.clone();
    }
}


pub struct WorkspaceView {
    workspace: Workspace,

    view_wrapper: TypedColumnView<TreeItem, gtk::SingleSelection>
}


impl WorkspaceView {
    pub fn update(&mut self, ws: Workspace) {
        self.workspace = ws;
    }
}



// impl Default for WorkspaceView {
//     fn default() -> Self {
//         return Self {
//             workspace: Workspace::default(),
//             view_wrapper
//         };
//     }
// }


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

                #[local_ref]
                tree_view -> gtk::ColumnView {
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut view_wrapper = TypedColumnView::<TreeItem, gtk::SingleSelection>::new();
        view_wrapper.append_column::<NameColumn>();

        let model = WorkspaceView {
            workspace: Workspace::default(),
            view_wrapper: view_wrapper
        };

        // debug!("{:?}", &model.view_wrapper.view);
        let tree_view = &model.view_wrapper.view;

        let widgets = view_output!();

        // debug!("cv: {:?}", widgets.tree_view);


        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            WorkspaceActions::Open(ws)=> {
                debug!("WorkspaceActions::Open update: {:?}", ws);
                // self.workspace = ws;

                self.view_wrapper.clear();
                for n in ws.children() {
                    debug!("node: {:?}", n);
                    self.view_wrapper.append(TreeItem::new(
                        n.name().as_str(),
                        n.path().as_str()
                    ));
                }
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