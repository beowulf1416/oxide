use crate::app::Workspace;

#[derive(Debug)]
pub enum ApplicationMessage {
    Close,
    CloseRequest,
    WorkspaceOpen(Workspace), /* path  */
    WorkspaceSave,
}