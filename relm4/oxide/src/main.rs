

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


// use app::App;



struct App {
    workspace: Workspace
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    path: String
}


impl Workspace {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn set_path(&self, path: String) -> Self {
        return Self::new(path);
    }
}



relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(ExampleAction, WindowActionGroup, "example");



#[relm4::component]
impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    // type Widgets = AppWidgets;

    menu! {
        main_menu: {
            "File" {
                "New" => ExampleAction,
                "Open" => ExampleAction,
                section! {
                    "Open Workspace" => ExampleAction,
                    "Save Workspace" => ExampleAction,
                },
                section! {
                    "Quit" => ExampleAction,
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
                        set_icon_name: "document-open"
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
        let model = App {
            workspace: Workspace::new(String::from("")),
        };
        let widgets = view_output!();

        ComponentParts { 
            widgets, 
            model
        }
    }
}


fn main() {
    env_logger::init();

    gio::resources_register_include!("org.devphilplus.oxide.gresource")
        .expect("failed to register resources");


    let app = RelmApp::new("org.devphilplus.oxide");

    app.run::<App>(());
}
