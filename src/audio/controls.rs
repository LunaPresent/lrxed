use std::{
	sync::{Mutex, atomic::AtomicBool},
	time::Duration,
};

#[derive(Debug)]
pub struct Controls {
	pub pause: AtomicBool,
	pub stopped: AtomicBool,
	pub volume: Mutex<f32>,
	pub speed: Mutex<f32>,
	pub seek: Mutex<Option<Duration>>,
	pub position: Mutex<Duration>,
	pub duration: Mutex<Option<Duration>>,
}
