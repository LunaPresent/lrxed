use std::time::Duration;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsLine {
	pub time: Option<Duration>,
	pub time_str: String,
	pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Vec<LyricsLine>,
	pub cursor_y: usize,
	pub scroll_y: usize,
}

impl Default for LyricsState {
	fn default() -> Self {
		Self {
			lyrics: vec![
				LyricsLine {
					time: None,
					time_str: "".to_owned(),
					text: "If I could".to_owned(),
				},
				LyricsLine {
					time: None,
					time_str: "".to_owned(),
					text: "Walk right beside you, show you just who I am".to_owned(),
				},
				LyricsLine {
					time: None,
					time_str: "".to_owned(),
					text: "You know I would".to_owned(),
				},
				LyricsLine {
					time: None,
					time_str: "00:12.87".to_owned(),
					text: "The memories we made fade".to_owned(),
				},
				LyricsLine {
					time: None,
					time_str: "00:13.36".to_owned(),
					text: "I realise we will never be the same".to_owned(),
				},
			],
			cursor_y: 0,
			scroll_y: 0,
		}
	}
}
