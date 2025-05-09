use super::{AudioState, Config, LyricsState};

#[derive(Default)]
pub struct AppState {
	pub audio: AudioState,
	pub lyrics: LyricsState,
	pub config: Config,
}
