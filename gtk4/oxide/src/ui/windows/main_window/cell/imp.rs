use gtk::{
    glib,
    subclass::prelude::*,
};



#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/devphilplus/oxide/cell.ui")]
pub struct Cell {
    #[template_child]
    pub value: TemplateChild<gtk::Inscription>,
}


#[glib::object_subclass]
impl ObjectSubclass for Cell {
    const NAME: &'static str = "Cell";
    type Type = super::Cell;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


impl ObjectImpl for Cell {
    fn dispose(&self) {
        // self.parent_dispose();
        self.dispose_template();
    }
}

impl WidgetImpl for Cell {}