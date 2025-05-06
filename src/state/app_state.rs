use super::{AudioState, LyricsState};

#[derive(Default)]
pub struct AppState {
	pub audio_state: AudioState,
	pub lyrics_state: LyricsState,
}
