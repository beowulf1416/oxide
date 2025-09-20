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


use crate::ui::windows::main_window::directory_item::DirectoryItem;
use crate::ui::windows::main_window::cell::{
    Cell,
    Entry
};
use crate::ui::windows::main_window::node::{
    Node,
    NodeObject
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

        /* name */
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
        /* name */

        /* type */
        let type_factory = gtk::SignalListItemFactory::new();
        type_factory.connect_setup(move |_factory, list_item| {
            let item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

            let r = Cell::default();
            item.set_child(Some(&r));
        });

        type_factory.connect_bind(move |_factory, list_item| {
            let item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

            let child = item.child().and_downcast::<Cell>().unwrap();
            let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();

            let r: Ref<DirectoryItem> = entry.borrow();
            let e = Entry {
                value: r.item_type()
            };

            child.set_value(&e);
        });

        let type_column = gtk::ColumnViewColumn::new(Some("Name"), Some(type_factory));
        self.files.append_column(&type_column);
        /* type */
    }

    fn setup_treeview(&self) {
        // example tree data
        let data = vec![
            Node {
                name: "file_1".into(),
                children: vec![
                    Node {
                        name: "file_1_1".into(),
                        children: vec![]
                    },
                    Node {
                        name: "file_1_2".into(),
                        children: vec![]
                    }
                ]
            },
            Node {
                name: "file_2".into(),
                children: vec![
                    Node {
                        name: "file_2_1".into(),
                        children: vec![]
                    },
                    Node {
                        name: "file_2_2".into(),
                        children: vec![]
                    }
                ]
            }
        ];
        let store = gio::ListStore::new::<BoxedAnyObject>();
        for node in data {
            store.append(&BoxedAnyObject::new(node));
        }

        let tlm = gtk::TreeListModel::new(
            store,
            false, // passthrough = false -> produces TreeListRow
            false, // autoexpand = false
            |item| {
                let node = item.downcast_ref::<BoxedAnyObject>().unwrap();
                let node: Ref<Node> = node.borrow();

                let children_store = gio::ListStore::new::<BoxedAnyObject>();
                for child in &node.children {
                    children_store.append(&BoxedAnyObject::new(child.clone()));
                }
                Some(children_store.upcast())
            },
        );

        let selection = gtk::SingleSelection::new(Some(tlm));
        self.files.set_model(Some(&selection));

        /* name */
        let name_factory = gtk::SignalListItemFactory::new();
        name_factory.connect_setup(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let expander = gtk::TreeExpander::new();
            let label = gtk::Label::new(None);
            label.set_xalign(0.0);
            expander.set_child(Some(&label));          
            item.set_child(Some(&expander));
        });
        name_factory.connect_bind(|_, item| {
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            let row: gtk::TreeListRow = item.property("item");

            // debug!("row: {:?}", row);

            if let Some(inner) = row.item() {
                // debug!("inner: {:?}", inner.downcast_ref::<BoxedAnyObject>());

                let node: Ref<Node> = inner.downcast_ref::<BoxedAnyObject>().unwrap().borrow();
                // debug!("node: {:?}", node);

                // let node = inner.downcast_ref::<Node>().unwrap();
            //     // let name: String = node.property::<String>("name");
                let name = node.name();
                if let Some(expander) = item.child().and_downcast::<gtk::TreeExpander>() {
                    if let Some(child) = expander.child().and_downcast::<gtk::Label>() {
                        child.set_label(&name);
                    }
                    expander.set_list_row(Some(&row));
                }
            }
        });
        // name_factory.connect_unbind(|_, item| {
        //     let item = item.downcast_ref::<gtk::ListItem>().unwrap();
        //     if let Some(expander) = item.child().and_downcast::<gtk::TreeExpander>() {
        //         expander.set_list_row(None::<&gtk::TreeListRow>);
        //         if let Some(label) = expander.child().and_downcast::<gtk::Label>() {
        //             label.set_label("");
        //         }
        //     }
        // });

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
        // self.setup_columnview();
        self.setup_treeview();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}