use crate::app::{
    // Workspace,
    RootNode
};
use crate::structs::workspace::Workspace;

#[derive(Debug)]
pub enum ApplicationMessage {
    Close,
    CloseRequest,
    WorkspaceOpen(Workspace),
    WorkspaceSave,
    WorkspaceFolderAdd(RootNode)
}