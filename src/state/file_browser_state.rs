use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Default)]
pub struct FileBrowserState {
	cache: HashMap<PathBuf, Vec<PathBuf>>,
	pub directory: PathBuf,
	pub selected_line: i16,
}

impl FileBrowserState {
	pub fn get_directory_contents(&mut self) -> &[PathBuf] {
		self.cache
			.entry(self.directory.clone())
			.or_insert_with(|| {
				if let Ok(directory) = fs::read_dir(&self.directory) {
					directory
						.filter(Result::is_ok)
						.map(|r| r.unwrap().path())
						.collect()
				} else {
					vec![]
				}
			})
			.as_slice()
	}
}
