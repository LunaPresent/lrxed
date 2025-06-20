mod confirm_back_modal;
mod confirm_modal;
mod confirm_quit_modal;
mod editor_view;
mod file_tree_view;
mod keys_modal;
mod toasts_overlay;

pub use confirm_back_modal::ConfirmBackModal;
pub use confirm_quit_modal::ConfirmQuitModal;
pub use editor_view::EditorView;
pub use file_tree_view::FileTreeView;
pub use keys_modal::KeysModal;
pub use toasts_overlay::ToastsOverlay;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
	FileTree,
	Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
	ConfirmQuit,
	GoBack,
	Keys,
}
