#[derive(Debug)]
pub enum ApplicationMessage {
    Close,
    CloseRequest,
    WorkspaceOpen,
    WorkspaceSave,
}