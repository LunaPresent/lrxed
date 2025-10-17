use color_eyre::eyre;

use std::{
	borrow::Cow,
	cmp::Ordering,
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

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
			Self::Song(song) => match song.meta {
				Some(ref meta) if !meta.artist.is_empty() && !meta.title.is_empty() => {
					Cow::Owned(format!("{} - {}", meta.artist, meta.title))
				}
				_ => Cow::Borrowed(
					song.mp3_file
						.file_name()
						.unwrap_or_default()
						.to_str()
						.unwrap_or_default(),
				),
			},
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
	cache: HashMap<PathBuf, Vec<FileBrowserItem>>,
	directory: PathBuf,
	pub cursor: Cursor,
}

impl FileBrowserState {
	pub fn items(&self) -> &[FileBrowserItem] {
		self.cache.get(&self.directory).map_or(&[], |result| result)
	}

	pub fn items_mut(&mut self) -> &mut [FileBrowserItem] {
		self.cache
			.get_mut(&self.directory)
			.map_or(&mut [], |result| result)
	}

	pub fn directory(&self) -> &Path {
		self.directory.as_path()
	}

	pub fn parent(&self) -> Option<PathBuf> {
		self.directory.parent().map(PathBuf::from)
	}

	pub fn open_directory(&mut self, path: &Path) -> eyre::Result<()> {
		eyre::ensure!(path.exists(), "Specified directory does not exist");

		self.directory = path.to_path_buf();

		if !self.cache.contains_key(&self.directory) {
			self.cache.insert(path.to_path_buf(), {
				match fs::read_dir(&self.directory) {
					Ok(directory) => {
						let mut result = directory
							.filter_map(|item| item.map_or(None, |r| Some(r.path())))
							.map(|path| FileBrowserItem::try_from(path.as_path()))
							.filter_map(|result| result.map_or(None, Some))
							.collect::<Vec<_>>();

						result.sort();
						result.into()
					}
					Err(_) => vec![],
				}
			});
		}

		Ok(())
	}

	pub fn update_selected_song(&mut self, new_song: Song) {
		let index = self.cursor.pos().y as usize;

		if let Some(FileBrowserItem::Song(song)) = self.items_mut().get_mut(index) {
			*song = new_song;
		}
	}
}
