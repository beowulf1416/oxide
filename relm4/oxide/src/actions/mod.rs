pub mod close_request;
pub mod about_action;

pub mod workspace_open;
pub mod workspace_save;

pub mod workspace_folder_add;


pub mod new_text_file_action;
pub mod new_sql_file_action;
pub mod new_python_file_action;




relm4::new_action_group!(pub WindowActionGroup, "win");