mod confirm_back_modal;
mod confirm_modal;
mod confirm_quit_modal;
mod editor_view;
mod file_tree_view;

pub use confirm_back_modal::ConfirmBackModal;
pub use confirm_quit_modal::ConfirmQuitModal;
pub use editor_view::EditorView;
pub use file_tree_view::FileTreeView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
	FileTree,
	Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
	ConfirmQuit,
	GoBack,
}
