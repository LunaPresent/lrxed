use std::{
	convert::identity,
	path::{Path, PathBuf},
};

use lofty::{
	file::TaggedFileExt,
	tag::{ItemKey, Tag, TagType},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadSongError {
	#[error("Path is not a file")]
	PathWasDirectory,
	#[error("File does not exist")]
	FileDoesNotExist,
	#[error("Invalid file type")]
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
	pub lrc_file: PathBuf,
	pub meta: Option<SongMeta>,
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
			Ok(Self::from_mp3(path.to_path_buf()))
		} else {
			Err(LoadSongError::InvalidFileType)
		}
	}

	fn new(mp3_file: PathBuf, lrc_file: PathBuf) -> Self {
		let meta = lofty::read_from_path(&mp3_file)
			.map(|tags| tags.tag(TagType::Id3v2).cloned())
			.map_or(None, identity)
			.map(SongMeta::from);

		Self {
			meta,
			mp3_file,
			lrc_file,
		}
	}

	fn from_mp3(path: PathBuf) -> Song {
		Self::new(path.clone(), path.with_extension("lrc"))
	}
}
