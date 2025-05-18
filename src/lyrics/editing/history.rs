use color_eyre::eyre;

use crate::lyrics::{Lyrics, TimeIndex};

use super::Edit;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct History {
	undo_stack: Vec<Edit>,
	redo_stack: Vec<Edit>,
}

impl History {
	pub fn push(&mut self, edit: Edit) {
		self.undo_stack.push(edit);
		self.redo_stack.clear();
	}

	pub fn undo(&mut self, lyrics: &mut Lyrics, time_index: &mut TimeIndex) -> eyre::Result<()> {
		if let Some(edit) = self.undo_stack.pop() {
			edit.execute_backwards(lyrics, time_index)?;
			self.redo_stack.push(edit);
		}
		Ok(())
	}

	pub fn redo(&mut self, lyrics: &mut Lyrics, time_index: &mut TimeIndex) -> eyre::Result<()> {
		if let Some(edit) = self.redo_stack.pop() {
			edit.execute_forwards(lyrics, time_index)?;
			self.undo_stack.push(edit);
		}
		Ok(())
	}
}
