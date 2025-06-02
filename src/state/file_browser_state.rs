use std::path::PathBuf;

#[derive(Default)]
pub struct FileBrowserState {
	pub directory: PathBuf,
	pub subdirectories: Vec<PathBuf>,
}
