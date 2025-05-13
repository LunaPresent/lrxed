use crate::lyrics::Lyrics;

use super::Coord;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub cursor: Coord,
	pub scroll_y: usize,
}
