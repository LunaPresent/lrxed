#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
	pub scrolloff: usize,
	pub jump_seconds: f32,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			scrolloff: 8,
			jump_seconds: 5.,
		}
	}
}
