mod editor_view;
mod file_tree_view;

pub use editor_view::EditorView;
pub use file_tree_view::FileTreeView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
	FileTree,
	Editor,
}
