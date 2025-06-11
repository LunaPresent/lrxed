use mime_guess::mime;

use std::{
	convert::identity,
	path::{Path, PathBuf},
};

use lofty::{
	file::TaggedFileExt,
	tag::{ItemKey, Tag, TagType},
};

#[derive(Debug)]
pub enum LoadSongError {
	PathWasDirectory,
	FileDoesNotExist,
	NoMp3FileFound,
	InvalidFileType,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SongMeta {
	pub title: String,
	pub artist: String,
}

impl From<Tag> for SongMeta {
	fn from(value: Tag) -> Self {
		let title = value.get_string(&ItemKey::TrackTitle).unwrap_or_default();
		let artist = value.get_string(&ItemKey::TrackArtist).unwrap_or_default();

		Self {
			title: title.to_string(),
			artist: artist.to_string(),
		}
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct Song {
	pub mp3_file: PathBuf,
	pub lrc_file: Option<PathBuf>,
	pub meta: Option<SongMeta>,
}

impl Song {
	pub fn is_valid_file_type(path: &Path) -> bool {
		mime_guess::from_path(path)
			.first()
			.map_or(false, |mime_type| mime_type.type_() == mime::AUDIO)
	}

	pub fn from_file(path: &Path) -> Result<Song, LoadSongError> {
		if !path.exists() {
			return Err(LoadSongError::FileDoesNotExist);
		}

		if !path.is_file() {
			return Err(LoadSongError::PathWasDirectory);
		}

		if Self::is_valid_file_type(&path) {
			Ok(Self::from_mp3(path.to_path_buf()))
		} else if path.extension().is_some_and(|it| it == "lrc") {
			Self::from_lrc(path.to_path_buf())
		} else {
			Err(LoadSongError::InvalidFileType)
		}
	}

	fn new(mp3_file: PathBuf, lrc_file: Option<&Path>) -> Self {
		let meta = lofty::read_from_path(&mp3_file)
			.map(|tags| tags.tag(TagType::Id3v2).cloned())
			.map_or(None, identity)
			.map(SongMeta::from);

		Self {
			meta,
			mp3_file,
			lrc_file: lrc_file.map(Path::to_path_buf),
		}
	}

	fn from_mp3(path: PathBuf) -> Song {
		let lrc_path = path.with_extension("lrc");
		let lrc_path = lrc_path.exists().then_some(lrc_path.as_path());

		Self::new(path.clone(), lrc_path)
	}

	fn from_lrc(path: PathBuf) -> Result<Song, LoadSongError> {
		let mp3_path = path.with_extension("mp3");

		if !mp3_path.exists() {
			return Err(LoadSongError::NoMp3FileFound);
		}

		Ok(Self::new(mp3_path, Some(&path)))
	}
}
