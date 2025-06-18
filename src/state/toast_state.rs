use std::{
	collections::VecDeque,
	time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toast {
	pub text: String,
	spawn_time: Instant,
}

impl Toast {
	fn new(text: String) -> Self {
		Self {
			text,
			spawn_time: Instant::now(),
		}
	}

	fn has_timed_out(&self, timeout: Duration) -> bool {
		self.spawn_time.elapsed() >= timeout
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ToastState {
	toast_queue: VecDeque<Toast>,
}

impl ToastState {
	pub fn push(&mut self, toast_text: String) {
		self.toast_queue.push_front(Toast::new(toast_text));
	}

	pub fn cull(&mut self, timeout: Duration) {
		while self
			.toast_queue
			.back()
			.is_some_and(|toast| toast.has_timed_out(timeout))
		{
			self.toast_queue.pop_back();
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = &Toast> {
		self.toast_queue.iter()
	}
}
