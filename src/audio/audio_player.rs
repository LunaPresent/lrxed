use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use color_eyre::eyre;
use rodio::{OutputStreamHandle, Sample, Source, cpal::FromSample};

use super::controls::Controls;
use super::playback_starter::{PlaybackStarter, StartPlayback};

/// Handle to a device that outputs sounds.
pub struct AudioPlayer {
	controls: Arc<Controls>,
	playback_starter: Box<dyn StartPlayback>,
}

impl AudioPlayer {
	pub fn try_new<F, S>(
		stream: &OutputStreamHandle,
		source_factory: F,
	) -> eyre::Result<AudioPlayer>
	where
		F: Fn() -> eyre::Result<(S, Duration)> + 'static,
		S: Source + Send + 'static,
		f32: FromSample<S::Item>,
		S::Item: Sample + Send,
	{
		let controls = Arc::new(Controls {
			pause: AtomicBool::new(false),
			stopped: AtomicBool::new(false),
			volume: Mutex::new(0.5),
			speed: Mutex::new(1.0),
			seek: Mutex::new(None),
			position: Mutex::new(Duration::ZERO),
			duration: Mutex::new(Duration::ZERO),
		});

		let controls_clone = controls.clone();

		let player = AudioPlayer {
			controls,
			playback_starter: Box::new(PlaybackStarter::new(
				stream.clone(),
				source_factory,
				controls_clone,
			)),
		};

		player.playback_starter.start_playback()?;

		Ok(player)
	}

	/// Gets the volume of the sound.
	///
	/// The value `1.0` is the "normal" volume (unfiltered input). Any value other than 1.0 will
	/// multiply each sample by this value.
	pub fn volume(&self) -> f32 {
		*self.controls.volume.lock().unwrap()
	}

	/// Changes the volume of the sound.
	///
	/// The value `1.0` is the "normal" volume (unfiltered input). Any value other than `1.0` will
	/// multiply each sample by this value.
	pub fn set_volume(&self, value: f32) {
		*self.controls.volume.lock().unwrap() = value;
	}

	/// Gets the play speed of the sound
	///
	/// # Note:
	/// 1. **Increasing the speed will increase the pitch by the same factor**
	/// - If you set the speed to 0.5 this will halve the frequency of the sound
	///   lowering its pitch.
	/// - If you set the speed to 2 the frequency will double raising the
	///   pitch of the sound.
	/// 2. **Change in the speed affect the total duration inversely**
	/// - If you set the speed to 0.5, the total duration will be twice as long.
	/// - If you set the speed to 2 the total duration will be halve of what it
	///   was.
	pub fn speed(&self) -> f32 {
		*self.controls.speed.lock().unwrap()
	}

	/// Changes the play speed of the sound.
	///
	/// The value `1.0` is the "normal" speed (unfiltered input). Any value other than `1.0` will
	/// change the play speed of the sound.
	///
	/// #### Note:
	/// 1. **Increasing the speed would also increase the pitch by the same factor**
	/// - If you increased set the speed to 0.5, the frequency would be slower (0.5x the original frequency) .
	/// - Also if you set the speed to 1.5 the frequency would be faster ( 1.5x the original frequency).
	/// 2. **Change in the speed would affect your total duration inversely**
	/// - if you set the speed by 0.5, your total duration would be (2x the original total duration) longer.
	/// - Also if you set the speed to 2 the total duration would be (0.5 the original total_duration) shorter
	pub fn set_speed(&self, value: f32) {
		*self.controls.speed.lock().unwrap() = value;
	}

	/// Attempts to seek to a given position in the current source.
	///
	/// As long as the duration of the source is known, seek is guaranteed to saturate
	/// at the end of the source. For example given a source that reports a total duration
	/// of 42 seconds calling `try_seek()` with 60 seconds as argument will seek to
	/// 42 seconds.
	///
	/// # Errors
	/// This function will cause a panic if one of the underlying
	/// sources does not support seeking.
	///
	/// It will cause a panic if an implementation ran into one during the seek.
	///
	/// When seeking beyond the end of a source this
	/// function might cause a panic if the duration of the source is not known.
	pub fn seek(&self, pos: Duration) -> eyre::Result<()> {
		let duration = self.duration();
		if duration <= pos {
			self.stop();
			*self.controls.position.lock().unwrap() = duration;
		} else {
			if self.controls.stopped.fetch_and(false, Ordering::SeqCst) {
				self.playback_starter.start_playback()?;
			}
			*self.controls.seek.lock().unwrap() = Some(pos);
		}
		Ok(())
	}

	/// Pauses or resumes playback
	pub fn set_paused(&self, value: bool) {
		self.controls.pause.store(value, Ordering::SeqCst);
	}

	/// Gets whether the player is paused
	pub fn is_paused(&self) -> bool {
		self.controls.pause.load(Ordering::SeqCst)
	}

	/// Stops the player
	pub fn stop(&self) {
		self.controls.stopped.store(true, Ordering::SeqCst);
	}

	/// Returns the position of the sound that's being played.
	///
	/// This takes into account any speedup or delay applied.
	///
	/// Example: if you apply a speedup of *2* to an mp3 decoder source and
	/// [`position\(\)`](AudioPlayer::position) returns *5s* then the position in the mp3
	/// recording is *10s* from its start.
	pub fn position(&self) -> Duration {
		*self.controls.position.lock().unwrap()
	}

	pub fn duration(&self) -> Duration {
		*self.controls.duration.lock().unwrap()
	}
}

impl Drop for AudioPlayer {
	fn drop(&mut self) {
		self.controls.stopped.store(true, Ordering::Relaxed);
	}
}
