use color_eyre::eyre;

use crate::config::KeyChord;

pub trait InputHandler {
	type State;

	fn handle_input(&mut self, key_chord: KeyChord, state: &mut Self::State) -> eyre::Result<bool>;
}
