pub mod close_request;

pub mod workspace_open;
pub mod workspace_save;

pub mod workspace_folder_add;


relm4::new_action_group!(pub WindowActionGroup, "win");