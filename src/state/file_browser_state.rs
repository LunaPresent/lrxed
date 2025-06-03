use crate::song::Song;
use std::{cmp::Ordering, collections::HashMap, ffi::OsStr, fs, path::PathBuf};

pub enum FileBrowserItem {
	Directory(PathBuf),
	Song(Song),
}

#[derive(Default)]
pub struct FileBrowserState {
	cache: HashMap<PathBuf, Vec<FileBrowserItem>>,
	pub directory: PathBuf,
	pub selected_line: i16,
}

impl FileBrowserState {
	pub fn get_directory_contents(&mut self) -> &[FileBrowserItem] {
		self.cache
			.entry(self.directory.clone())
			.or_insert_with(|| {
				if let Ok(directory) = fs::read_dir(&self.directory) {
					let mut result = directory
						.filter(Result::is_ok)
						.map(|dir_item| dir_item.unwrap().path())
						.filter(|path| path.is_dir() || path.extension() == Some(OsStr::new("mp3")))
						.map(|path| {
							if path.is_file() {
								FileBrowserItem::Song(Song::from_file(path).unwrap())
							} else {
								FileBrowserItem::Directory(path)
							}
						})
						.collect::<Vec<_>>();

					result.sort_by(|f, _| {
						if let FileBrowserItem::Directory(_) = f {
							Ordering::Less
						} else {
							Ordering::Greater
						}
					});

					result
				} else {
					vec![]
				}
			})
			.as_slice()
	}
}
