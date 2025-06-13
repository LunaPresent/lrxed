use std::{
	borrow::Cow,
	cmp::Ordering,
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
	rc::Rc,
};

use color_eyre::eyre;

use crate::{
	song::{LoadSongError, Song},
	tui::Cursor,
};

#[derive(Clone, PartialEq, Eq)]
pub enum FileBrowserItem {
	Directory(PathBuf),
	Song(Song),
}

impl FileBrowserItem {
	pub fn name(&self) -> Cow<str> {
		match self {
			Self::Directory(path) => Cow::Borrowed(
				path.file_name()
					.unwrap_or_default()
					.to_str()
					.unwrap_or_default(),
			),
			Self::Song(song) => {
				if let Some(ref meta) = song.meta {
					Cow::Owned(format!("{} - {}", meta.artist, meta.title))
				} else {
					Cow::Borrowed(
						song.mp3_file
							.file_name()
							.unwrap_or_default()
							.to_str()
							.unwrap_or_default(),
					)
				}
			}
		}
	}
}

impl TryFrom<&Path> for FileBrowserItem {
	type Error = LoadSongError;

	fn try_from(value: &Path) -> Result<Self, Self::Error> {
		if value.is_dir() {
			Ok(Self::Directory(value.to_path_buf()))
		} else {
			Ok(Self::Song(Song::from_file(value)?))
		}
	}
}

impl PartialOrd for FileBrowserItem {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for FileBrowserItem {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(FileBrowserItem::Directory(_), FileBrowserItem::Song(_)) => Ordering::Less,
			(FileBrowserItem::Song(_), FileBrowserItem::Directory(_)) => Ordering::Greater,
			(a, b) => a.name().cmp(&b.name()),
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

	pub fn open_directory(&mut self, path: &Path) -> eyre::Result<()> {
		if !path.exists() {
			return Err(eyre::eyre!("Specified directory does not exist"));
		}

		self.directory = path.to_path_buf();
		self.items = Rc::clone(self.get_directory_contents());

		Ok(())
	}

	fn get_directory_contents(&mut self) -> &Rc<Vec<FileBrowserItem>> {
		self.cache.entry(self.directory.clone()).or_insert_with(|| {
			let Ok(directory) = fs::read_dir(&self.directory) else {
				return Default::default();
			};

			let mut result = directory
				.filter_map(|item| item.map_or(None, |r| Some(r.path())))
				.map(|path| FileBrowserItem::try_from(path.as_path()))
				.filter_map(|result| result.map_or(None, Some))
				.collect::<Vec<_>>();

			result.sort();
			result.into()
		})
	}
}
