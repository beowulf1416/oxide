use crate::app::{
    Workspace,
    RootNode
};

#[derive(Debug)]
pub enum ApplicationMessage {
    Close,
    CloseRequest,
    WorkspaceOpen(Workspace),
    WorkspaceSave,
    WorkspaceFolderAdd(RootNode)
}