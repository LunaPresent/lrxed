use std::{ffi::OsStr, path::PathBuf};

use lofty::{
	file::TaggedFileExt,
	tag::{ItemKey, TagType},
};

#[derive(Debug)]
pub enum LoadSongError {
	PathWasDirectory,
	FileDoesNotExist,
	NoMp3FileFound,
	InvalidFileType,
}

#[derive(Clone, PartialEq)]
pub struct SongMeta {
	pub title: String,
	pub artist: String,
}

#[derive(Clone, PartialEq)]
pub struct Song {
	pub mp3_file: PathBuf,
	pub lrc_file: Option<PathBuf>,
	pub meta: Option<SongMeta>,
}

impl Song {
	pub fn from_file(path: PathBuf) -> Result<Song, LoadSongError> {
		if !path.is_file() {
			return Err(LoadSongError::PathWasDirectory);
		}

		if !path.exists() {
			return Err(LoadSongError::FileDoesNotExist);
		}

		match path.extension() {
			Some(ext) if ext == OsStr::new("mp3") => Ok(Self::from_mp3(path)),
			Some(ext) if ext == OsStr::new("lrc") => Self::from_lrc(path),
			_ => Err(LoadSongError::InvalidFileType),
		}
	}

	fn new(mp3_file: PathBuf, lrc_file: Option<PathBuf>) -> Self {
		let mut meta = None;

		if let Ok(Some(tags)) =
			lofty::read_from_path(mp3_file.as_path()).map(|tags| tags.tag(TagType::Id3v2).cloned())
		{
			let title = tags.get_string(&ItemKey::TrackTitle).unwrap_or_default();
			let artist = tags.get_string(&ItemKey::TrackArtist).unwrap_or_default();

			meta = Some(SongMeta {
				title: title.to_string(),
				artist: artist.to_string(),
			})
		}

		Self {
			mp3_file,
			lrc_file,
			meta,
		}
	}

	fn from_mp3(path: PathBuf) -> Song {
		let lrc_path = path.with_extension("lrc");

		Self::new(
			path.clone(),
			if lrc_path.exists() {
				Some(lrc_path)
			} else {
				None
			},
		)
	}

	fn from_lrc(path: PathBuf) -> Result<Song, LoadSongError> {
		let mp3_path = path.with_extension("mp3");

		if !mp3_path.exists() {
			return Err(LoadSongError::NoMp3FileFound);
		}

		Ok(Self::new(mp3_path, Some(path.clone())))
	}
}
