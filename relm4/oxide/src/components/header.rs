use gtk::{glib, prelude::*};
use relm4::prelude::*;

use crate::application_message::ApplicationMessage;


struct Header {

}


#[relm4::component]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = ApplicationMessage;

    view! {
        gtk::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Button {
                set_label: "Oxide",
            }
        }
    }


    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Header {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}