use std::time::Duration;

use color_eyre::eyre;

use crate::audio::{AudioDevice, AudioPlayer};

#[derive(Default)]
pub struct AudioState {
	pub audio_device: AudioDevice,
	pub audio_player: Option<AudioPlayer>,
}

impl AudioState {
	pub fn seek_relative(&self, relative_position: f32) -> eyre::Result<Option<Duration>> {
		if let Some(player) = self.audio_player.as_ref() {
			let target_pos = player.duration().mul_f32(relative_position.min(1.).max(0.));
			player.seek(target_pos)?;
			Ok(Some(target_pos))
		} else {
			Ok(None)
		}
	}
}
