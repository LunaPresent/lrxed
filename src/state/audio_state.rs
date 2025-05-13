use color_eyre::eyre::{self};

use crate::audio::{AudioDevice, AudioPlayer};

#[derive(Default)]
pub struct AudioState {
	pub audio_device: AudioDevice,
	pub audio_player: Option<AudioPlayer>,
}

impl AudioState {
	pub fn seek_relative_or_ignore(&mut self, relative_position: f32) -> eyre::Result<()> {
		if let Some(player) = self.audio_player.as_mut() {
			let target_pos = player
				.duration()
				.unwrap()
				.mul_f32(relative_position.min(1.).max(0.));
			player.restart_or_ignore()?;
			player.seek(target_pos);
		}

		Ok(())
	}
}
