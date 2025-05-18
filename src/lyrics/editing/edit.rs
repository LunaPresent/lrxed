use color_eyre::eyre;

use crate::lyrics::{Lyrics, TimeIndex};

use super::edit_action::EditAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
	forwards_action: EditAction,
	backwards_action: EditAction,
}

impl Edit {
	pub fn new(forwards_action: EditAction, backwards_action: EditAction) -> Self {
		Self {
			forwards_action,
			backwards_action,
		}
	}

	pub fn execute_forwards(
		&self,
		lyrics: &mut Lyrics,
		time_index: &mut TimeIndex,
	) -> eyre::Result<()> {
		self.forwards_action.execute(lyrics, time_index)
	}

	pub fn execute_backwards(
		&self,
		lyrics: &mut Lyrics,
		time_index: &mut TimeIndex,
	) -> eyre::Result<()> {
		self.backwards_action.execute(lyrics, time_index)
	}
}
