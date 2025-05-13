use crate::lyrics::Lyrics;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub cursor_y: usize,
	pub scroll_y: usize,
}
