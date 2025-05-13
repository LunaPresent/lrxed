use std::{fs::File, io::BufReader};

use color_eyre::eyre;
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
	pub fn try_play(&self, audio_file_path: &str) -> eyre::Result<AudioPlayer> {
		let path = audio_file_path.to_owned();
		AudioPlayer::try_new(&self.handle, move || {
			Ok(Decoder::new(BufReader::new(File::open(&path)?))?)
		})
	}
}
