use log::{
    debug
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
use relm4::actions::*;


use crate::app::App;
use crate::application_message::ApplicationMessage;


use crate::actions::*;


pub struct MainWindow {

}



// relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ExampleAction, WindowActionGroup, "example");
// relm4::new_stateless_action!(CloseRequestAction, WindowActionGroup, "close-request");


#[relm4::component(pub)]
impl SimpleComponent for MainWindow {
    type Input = ApplicationMessage;
    type Output = ();
    type Init = ();

    menu! {
        main_menu: {
            "File" {
                "New" => ExampleAction,
                "Open" => ExampleAction,
                section! {
                    "Open Workspace" => workspace_open::WorkspaceOpenAction,
                    "Save Workspace" => ExampleAction,
                },
                section! {
                    "Quit" => close_request::CloseRequestAction,
                }
            },
            "Help" {
                "About" => ExampleAction,
            }
        }
    }

    view! {
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Oxide"),
            set_default_size: (800, 600),
            // set_child: Some(&self.widgets.main_box),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 0,

                gtk::PopoverMenuBar::from_model(Some(&main_menu)),

                // #[name(main_box)]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 0
                },

                gtk::ActionBar {
                    set_hexpand: true,
                    set_valign: gtk::Align::End,

                    pack_start = &gtk::Button {
                        set_label: "New",
                        set_icon_name: "document-new"
                    },

                    pack_start = &gtk::Button {
                        set_label: "Open",
                        set_icon_name: "document-open",
                        set_action_name: Some("win.workspace-open")

                    },
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = MainWindow {
        };
        let widgets = view_output!();


        let rc_sender = Rc::new(sender);

        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        let close_request_action = crate::actions::close_request::close_request_action(rc_sender.clone());
        group.add_action(close_request_action);

        let workspace_open_action = crate::actions::workspace_open::workspace_open_action(rc_sender.clone());
        group.add_action(workspace_open_action);

        group.register_for_widget(&widgets.main_window);


        ComponentParts { 
            widgets, 
            model
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ApplicationMessage::CloseRequest => {
                debug!("application message close request");
            }
            ApplicationMessage::Close => {
                relm4::main_application().quit();
            }
            ApplicationMessage::WorkspaceSave => {
                debug!("application message workspace save");
            }
            ApplicationMessage::WorkspaceOpen => {
                debug!("application message workspace open");
            }
        }
    }
}