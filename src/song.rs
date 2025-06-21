use crate::lyrics::Lyrics;
use thiserror::Error;

use std::{
	cell::RefCell,
	convert::identity,
	fs::File,
	io::BufReader,
	path::{Path, PathBuf},
	rc::Rc,
};

use lofty::{
	file::TaggedFileExt,
	tag::{ItemKey, Tag, TagType},
};

#[derive(Debug, Error)]
pub enum LoadSongError {
	#[error("Path is not a file")]
	PathWasDirectory,
	#[error("File does not exist")]
	FileDoesNotExist,
	#[error("Invalid file type")]
	InvalidFileType,
	#[error("Failed to read lyrics file")]
	FailedToReadLyrics,
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
	pub meta: Option<SongMeta>,
	pub lrc_file: PathBuf,
	pub lyrics: Option<Rc<RefCell<Lyrics>>>,
}

impl Song {
	pub fn is_valid_file_type(path: &Path) -> bool {
		matches!(
			path.extension().unwrap_or_default().to_str(),
			Some("mp3" | "wav" | "flac" | "ogg")
		)
	}

	pub fn from_file(path: &Path) -> Result<Song, LoadSongError> {
		if !path.exists() {
			return Err(LoadSongError::FileDoesNotExist);
		}

		if !path.is_file() {
			return Err(LoadSongError::PathWasDirectory);
		}

		if Self::is_valid_file_type(&path) {
			Self::from_mp3(path)
		} else {
			Err(LoadSongError::InvalidFileType)
		}
	}

	fn new(mp3_file: PathBuf, lrc_file: PathBuf) -> Result<Song, LoadSongError> {
		let meta = lofty::read_from_path(&mp3_file)
			.map(|tags| tags.tag(TagType::Id3v2).cloned())
			.map_or(None, identity)
			.map(SongMeta::from);

		let lyrics = if let Ok(file) = File::open(&lrc_file) {
			let reader = BufReader::new(file);
			let mut result = Lyrics::default();

			if result.read_overwrite(reader).is_err() {
				return Err(LoadSongError::FailedToReadLyrics);
			} else {
				Some(result)
			}
		} else {
			None
		};

		Ok(Self {
			meta,
			mp3_file,
			lrc_file,
			lyrics: lyrics.map(|lyrics| Rc::new(RefCell::new(lyrics))),
		})
	}

	fn from_mp3(path: &Path) -> Result<Song, LoadSongError> {
		let lrc_path = if path.with_extension("lrc").exists() {
			path.with_extension("lrc")
		} else if path.with_extension("txt").exists() {
			path.with_extension("txt")
		} else {
			path.with_extension("lrc")
		};

		Self::new(path.into(), lrc_path)
	}
}
