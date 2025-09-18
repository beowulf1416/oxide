use log::{
    debug
};

use std::cell::Ref;

use gtk::{
    prelude::*,
    glib::{
        self,
        clone,
        BoxedAnyObject
    },
    subclass::prelude::*,
    gio,
    gio::prelude::*
};


// use crate::ui::windows::main_window::views::file_item::FileItem;
// use crate::ui::windows::main_window::item::FileItem;
use crate::ui::windows::main_window::directory_item::DirectoryItem;
use crate::ui::windows::main_window::cell::{
    Cell,
    Entry
};



#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/devphilplus/oxide/main_window.ui")]
pub struct MainWindow { 
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub menubar: TemplateChild<gtk::PopoverMenuBar>,
    #[template_child]
    pub files: TemplateChild<gtk::ColumnView>,
}


impl MainWindow {

    fn add_actions(&self) {
        let obj = self.obj().clone();

        let preferences_action = crate::ui::actions::preferences::preferences_action(&obj);
        self.obj().add_action(&preferences_action);

        let about_action = crate::ui::actions::about::about_action(&obj);
        self.obj().add_action(&about_action);

        let file_open_action = crate::ui::actions::file_open::file_open_action(&obj);
        self.obj().add_action(&file_open_action);
    }

    fn setup_columnview(&self) {
        let store = gio::ListStore::new::<BoxedAnyObject>();

        (0..100).for_each(|i| {
            store.append(&BoxedAnyObject::new(DirectoryItem::new(
                format!("File {i}").as_str(), 
                format!("Type {i}").as_str()
            )));
        });
        
        let sm = gtk::SingleSelection::new(Some(store.upcast::<gio::ListModel>()));
        self.files.set_model(Some(&sm));

        let name_factory = gtk::SignalListItemFactory::new();
        name_factory.connect_setup(move |_factory, list_item| {
            let item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

            let r = Cell::default();
            item.set_child(Some(&r));
        });

        name_factory.connect_bind(move |_factory, list_item| {
            let item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

            let child = item.child().and_downcast::<Cell>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();

            let r: Ref<DirectoryItem> = entry.borrow();
            let e = Entry {
                value: r.name()
            };

            child.set_value(&e);
        });

        let name_column = gtk::ColumnViewColumn::new(Some("Name"), Some(name_factory));

        self.files.append_column(&name_column);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();

        self.add_actions();
        self.setup_columnview();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}