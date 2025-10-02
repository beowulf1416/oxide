use gtk::{
    prelude::*,
    gio,
    glib::{
        self,
        clone,
        BoxedAnyObject
    }
};

pub struct TreeColumnView {
    view: gtk::ColumnView,
    selection_model: gtk::SingleSelection,
    store: gio::ListStore,
    base_list_model: gtk::TreeListModel
}


impl TreeColumnView {

    pub fn new(view: &gtk::ColumnView) -> Self {
        let store = gio::ListStore::new::<BoxedAnyObject>();
        let base_list_model = gtk::TreeListModel::new(
            store.clone(),
            false,
            false,
            |item| {
                let node = item.downcast_ref::<BoxedAnyObject>().unwrap();
                
                let child_store = gio::ListStore::new::<BoxedAnyObject>();
                return Some(child_store.upcast());
            }
        );
        let selection_model = gtk::SingleSelection::new(Some(base_list_model.clone()));

        return Self {
            view: view.clone(),
            selection_model,
            store,
            base_list_model
        };
    }
}