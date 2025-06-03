use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub enum LoadSongError {
	PathWasDirectory,
	FileDoesNotExist,
	NoMp3FileFound,
	InvalidFileType,
}

#[derive(Debug)]
pub struct Song {
	pub mp3_file: PathBuf,
	pub lrc_file: Option<PathBuf>,
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

	fn from_mp3(path: PathBuf) -> Song {
		let lrc_path = path.with_extension("lrc");

		Song {
			mp3_file: path,
			lrc_file: if lrc_path.exists() {
				Some(lrc_path)
			} else {
				None
			},
		}
	}

	fn from_lrc(path: PathBuf) -> Result<Song, LoadSongError> {
		let mp3_path = path.with_extension("mp3");

		if !mp3_path.exists() {
			return Err(LoadSongError::NoMp3FileFound);
		}

		Ok(Song {
			mp3_file: mp3_path,
			lrc_file: Some(path),
		})
	}
}
