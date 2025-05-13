use color_eyre::eyre;
use crossterm::event::KeyEvent;

pub trait InputHandler {
	type State;

	fn handle_input(&mut self, key: &KeyEvent, state: &mut Self::State) -> eyre::Result<bool>;
}
