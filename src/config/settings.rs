#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
	pub scrolloff: u16,
}

impl Default for Settings {
	fn default() -> Self {
		Self { scrolloff: 8 }
	}
}
