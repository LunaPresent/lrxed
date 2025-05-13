use super::{AudioState, Config, Coord, LyricsState};

#[derive(Default)]
pub struct AppState {
	pub audio: AudioState,
	pub lyrics: LyricsState,
	pub cursor_pos: Option<Coord>,
	pub config: Config,
}
