use crate::{song::Song, tui::Cursor};
use std::{cmp::Ordering, collections::HashMap, ffi::OsStr, fs, path::PathBuf, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum FileBrowserItem {
	Directory(PathBuf),
	Song(Song),
}

impl From<PathBuf> for FileBrowserItem {
	fn from(value: PathBuf) -> Self {
		if value.is_dir() {
			Self::Directory(value)
		} else {
			Self::Song(Song::from_file(value).unwrap())
		}
	}
}

#[derive(Default)]
pub struct FileBrowserState {
	cache: HashMap<PathBuf, Rc<Vec<FileBrowserItem>>>,
	pub directory: PathBuf,
	pub cursor: Cursor,
	pub items: Rc<Vec<FileBrowserItem>>,
}

impl FileBrowserState {
	pub fn parent(&self) -> Option<PathBuf> {
		self.directory.parent().map(PathBuf::from)
	}

	pub fn open_directory(&mut self, path: &PathBuf) {
		self.directory = path.clone();
		self.items = Rc::clone(self.get_directory_contents());
	}

	fn get_directory_contents(&mut self) -> &Rc<Vec<FileBrowserItem>> {
		self.cache.entry(self.directory.clone()).or_insert_with(|| {
			let directory = if let Ok(result) = fs::read_dir(&self.directory) {
				result
			} else {
				return vec![].into();
			};

			let mut result = directory
				.filter(Result::is_ok)
				.map(|dir_item| dir_item.unwrap().path())
				.filter(|path| path.is_dir() || path.extension() == Some(OsStr::new("mp3")))
				.map(FileBrowserItem::from)
				.collect::<Vec<_>>();

			result.sort_by(|f, _| {
				if let FileBrowserItem::Directory(_) = f {
					Ordering::Less
				} else {
					Ordering::Greater
				}
			});

			result.into()
		})
	}
}
