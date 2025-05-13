use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use rodio::source::SeekError;
use rodio::{Sample, Source};

use super::controls::Controls;

/// When the inner source is empty this sets stopped
#[derive(Debug, Clone)]
pub struct StopOnEnd<I> {
	input: I,
	controls: Arc<Controls>,
	stop_set: bool,
}

impl<I> StopOnEnd<I> {
	pub fn new(input: I, controls: Arc<Controls>) -> StopOnEnd<I> {
		StopOnEnd {
			input,
			controls,
			stop_set: false,
		}
	}
}

impl<I: Source> Iterator for StopOnEnd<I>
where
	I: Source,
	I::Item: Sample,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<I::Item> {
		let next = self.input.next();
		if !self.stop_set && next.is_none() {
			self.controls.stopped.store(true, Ordering::Relaxed);
			self.stop_set = true;
		}
		next
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.input.size_hint()
	}
}

impl<I> Source for StopOnEnd<I>
where
	I: Source,
	I::Item: Sample,
{
	fn current_frame_len(&self) -> Option<usize> {
		self.input.current_frame_len()
	}

	fn channels(&self) -> u16 {
		self.input.channels()
	}

	fn sample_rate(&self) -> u32 {
		self.input.sample_rate()
	}

	fn total_duration(&self) -> Option<Duration> {
		self.input.total_duration()
	}

	fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
		self.input.try_seek(pos)
	}
}
