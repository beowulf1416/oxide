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
// use relm4_components::open_dialog::{
//     OpenDialog, OpenDialogMsg, OpenDialogResponse, OpenDialogSettings,
// };
// use relm4_components::save_dialog::{
//     SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings,
// };


use crate::app::{
    App,
    Workspace
};
use crate::application_message::ApplicationMessage;
use crate::actions::*;
use crate::components::*;


pub struct MainWindow {
    app: App,

    header: Controller<header::Header>,
    workspace_view: Controller<views::workspace::WorkspaceView>,
    data_sources_view: Controller<views::data_sources::DataSourcesView>,
    editor_view: Controller<views::editor::EditorView>
}


// impl Default for MainWindow {
//     fn default() -> Self {
//         return Self {
//             app: App::new(),
//             workspace_view: workspace::WorkspaceView::default()
//         };
//     }
// }


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
                "New" {
                    "Text File" => new_text_file_action::NewTextFileAction,
                    "SQL File" => new_sql_file_action::NewSQLFileAction,
                    "Python File" => new_python_file_action::NewPythonFileAction
                },
                "Open" => ExampleAction,
                section! {
                    "Workspace" {
                        "Open Workspace" => workspace_open::WorkspaceOpenAction,
                        "Save Workspace" => workspace_save::WorkspaceSaveAction,
                        section! {
                            "Add Folder" => workspace_folder_add::WorkspaceFolderAddAction
                        }
                    }
                },
                section! {
                    "Quit" => close_request::CloseRequestAction,
                }
            },
            "Help" {
                "About" => about_action::AboutAction,
            }
        },
        new_files_menu: {
            "Text File" => new_text_file_action::NewTextFileAction,
            "SQL File" => new_sql_file_action::NewSQLFileAction,
            "Python File" => new_python_file_action::NewPythonFileAction
        }
    }
    

    view! {
        #[root]
        main_window = gtk::ApplicationWindow {
            set_title: Some("Oxide"),
            set_default_size: (800, 600),
            set_titlebar: Some(model.header.widget()),
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

                    pack_start = &gtk::MenuButton {
                        set_label: "New",
                        set_icon_name: "document-new",
                        set_menu_model: Some(&new_files_menu)
                    },
                    pack_start = &gtk::Button {
                        set_label: "Save Workspace",
                        set_icon_name: "save",
                        set_action_name: Some("win.workspace-save"),
                    },
                    pack_start = &gtk::Button {
                        set_label: "Add Folder to Workspace",
                        set_icon_name: "plus-square-outline",
                        set_action_name: Some("win.workspace-folder-add")
                    },
                },
                
                gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_hexpand: true,
                    set_vexpand: true,

                    #[wrap(Some)]
                    set_start_child = &gtk::Notebook {
                        set_tab_pos: gtk::PositionType::Left,

                        append_page[Some(&gtk::Button::builder()
                            .icon_name("folder-visiting")
                            .tooltip_text("Workspace")
                            .build()
                        )] = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 0,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 0,

                                model.workspace_view.widget(),
                            },
                        },
                        append_page[Some(&gtk::Button::builder()
                            .icon_name("file-manager")
                            .tooltip_text("Data Sources")
                            .build()
                        )] = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 0,

                            gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 0,

                                model.data_sources_view.widget(),
                            },
                        },
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 0,

                        append = model.editor_view.widget(),
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let header = header::Header::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let workspace = views::workspace::WorkspaceView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let data_sources = views::data_sources::DataSourcesView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let editor = views::editor::EditorView::builder()
            .launch(())
            .forward(sender.input_sender(), identity);

        let model = MainWindow {
            app: App::new(),
            header: header,
            workspace_view: workspace,
            data_sources_view: data_sources,
            editor_view: editor
        };
        let widgets = view_output!();


        let rc_sender = Rc::new(sender);

        /* add actions */
        let mut group = RelmActionGroup::<WindowActionGroup>::new();

        let close_request_action = crate::actions::close_request::close_request_action(rc_sender.clone());
        group.add_action(close_request_action);

        let workspace_open_action = crate::actions::workspace_open::workspace_open_action(rc_sender.clone());
        group.add_action(workspace_open_action);

        let workspace_save_action = crate::actions::workspace_save::workspace_save_action(
            rc_sender.clone(), 
            model.app.workspace().clone()
        );
        group.add_action(workspace_save_action);

        let workspace_folder_add_action = crate::actions::workspace_folder_add::workspace_folder_add_action(
            rc_sender.clone()
        );
        group.add_action(workspace_folder_add_action);

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
                if let Err(e) = self.app.workspace().save() {
                    error!("error saving workspace: {:?}", e);
                }
            }
            ApplicationMessage::WorkspaceOpen(workspace) => {
                debug!("application message workspace open: {:?}", workspace);
                let _ = self.workspace_view.sender().send(crate::components::views::workspace::WorkspaceActions::Open);
                // if let Ok(_) = workspace.save() {
                //     self.app.set_workspace(workspace);
                // }
            }
            ApplicationMessage::WorkspaceFolderAdd(node) => {
                debug!("application message workspace folder add: {:?}", node);
                debug!("workspace: {:?}", self.app.workspace());
                debug!("workspace: {:?}", Rc::into_inner(self.app.workspace()));
                // let mut workspace = Rc::into_inner(self.app.workspace()).unwrap();
                // workspace.node_push(&node);

                // self.app.workspace_mut().node_push(&node);
            }
        }
    }
}