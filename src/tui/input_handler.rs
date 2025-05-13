use color_eyre::eyre;

use crate::config::Action;

pub trait InputHandler {
	type State;

	fn handle_input(&mut self, action: Action, state: &mut Self::State) -> eyre::Result<bool>;
}
