use std::{
	sync::{Arc, atomic::Ordering},
	time::Duration,
};

use color_eyre::eyre;
use rodio::{OutputStreamHandle, Sample, Source, cpal::FromSample};

use super::{controls::Controls, stop_on_end::StopOnEnd};

pub trait StartPlayback {
	fn start_playback(&self) -> eyre::Result<()>;
}

pub struct PlaybackStarter<F, S>
where
	F: Fn() -> eyre::Result<S> + 'static,
	S: Source + Send + 'static,
	f32: FromSample<S::Item>,
	S::Item: Sample + Send,
{
	handle: OutputStreamHandle,
	factory: F,
	controls: Arc<Controls>,
}

impl<F, S> PlaybackStarter<F, S>
where
	F: Fn() -> eyre::Result<S> + 'static,
	S: Source + Send + 'static,
	f32: FromSample<S::Item>,
	S::Item: Sample + Send,
{
	pub fn new(handle: OutputStreamHandle, factory: F, controls: Arc<Controls>) -> Self {
		Self {
			handle,
			factory,
			controls,
		}
	}
}

impl<F, S> StartPlayback for PlaybackStarter<F, S>
where
	F: Fn() -> eyre::Result<S> + 'static,
	S: Source + Send + 'static,
	f32: FromSample<S::Item>,
	S::Item: Sample + Send,
{
	fn start_playback(&self) -> eyre::Result<()> {
		let source = (self.factory)()?;
		*self.controls.duration.lock().unwrap() = source.total_duration();

		let access_controls = self.controls.clone();
		let source = source
			.track_position()
			.speed(1.0)
			.pausable(true)
			.amplify(1.0)
			.stoppable()
			.periodic_access(Duration::from_millis(5), move |src| {
				if access_controls.stopped.load(Ordering::SeqCst) {
					src.stop();
					*access_controls.position.lock().unwrap() = Duration::ZERO;
				}
				*access_controls.position.lock().unwrap() =
					src.inner().inner().inner().inner().get_pos();
				src.inner_mut()
					.set_factor(*access_controls.volume.lock().unwrap());
				src.inner_mut()
					.inner_mut()
					.inner_mut()
					.set_factor(*access_controls.speed.lock().unwrap());
				if let Some(seek) = access_controls.seek.lock().unwrap().take() {
					let speed = *access_controls.speed.lock().unwrap();
					_ = src.try_seek(seek.div_f32(speed));
				}
				src.inner_mut()
					.inner_mut()
					.set_paused(access_controls.pause.load(Ordering::SeqCst));
			})
			.convert_samples::<f32>();

		let source = StopOnEnd::new(source, self.controls.clone());
		self.handle.play_raw(source)?;
		Ok(())
	}
}
