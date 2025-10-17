use std::{fs::File, io::BufReader, path::PathBuf};

use color_eyre::eyre;
use lofty::file::AudioFile;
use rodio::{Decoder, OutputStream, OutputStreamHandle};

use super::AudioPlayer;

pub struct AudioDevice {
	_stream: OutputStream,
	handle: OutputStreamHandle,
}

impl Default for AudioDevice {
	fn default() -> Self {
		let (stream, handle) =
			OutputStream::try_default().expect("could not create audio output stream");
		Self {
			_stream: stream,
			handle,
		}
	}
}

impl AudioDevice {
	pub fn try_play(&self, audio_file_path: PathBuf) -> eyre::Result<AudioPlayer> {
		AudioPlayer::try_new(&self.handle, move || {
			let source = Decoder::new(BufReader::new(File::open(&audio_file_path)?))?;
			let tagged_file = lofty::read_from_path(audio_file_path.clone())?;
			Ok((source, tagged_file.properties().duration()))
		})
	}
}
