use gtk::{glib, prelude::*};
use relm4::prelude::*;

use crate::application_message::ApplicationMessage;
use crate::actions::*;


pub struct Header {

}



relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");


#[relm4::component(pub)]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = ApplicationMessage;

    menu! {
        settings: {
            "Preferences" => PreferencesAction
        }
    }

    view! {
        gtk::HeaderBar {
            pack_end = &gtk::MenuButton {
                set_icon_name: "settings",
                set_menu_model: Some(&settings)
            }
        }
    }


    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}