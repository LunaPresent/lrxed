use super::Timestamp;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricLine {
	timestamp: Option<Timestamp>,
	text: String,
}

impl LyricLine {
	pub fn new(timestamp: Option<Timestamp>, text: String) -> Self {
		Self { timestamp, text }
	}

	pub fn set_timestamp<T>(&mut self, timestamp: Option<T>)
	where
		T: Into<Timestamp>,
	{
		self.timestamp = timestamp.map(|x| x.into());
	}

	pub fn timestamp(&self) -> Option<&Timestamp> {
		self.timestamp.as_ref()
	}

	pub fn timestamp_text(&self) -> &str {
		match &self.timestamp {
			Some(timestamp) => timestamp.text(),
			None => "",
		}
	}

	pub fn text(&self) -> &str {
		&self.text
	}
}
