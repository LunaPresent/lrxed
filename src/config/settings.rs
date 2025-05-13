#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
	pub scrolloff: usize,
}

impl Default for Settings {
	fn default() -> Self {
		Self { scrolloff: 8 }
	}
}
