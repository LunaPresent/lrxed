use std::time::Duration;

use color_eyre::eyre;

use crate::lyrics::{Lyrics, TimeIndex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditAction {
	SetTimestamp {
		idx: u16,
		timestamp: Option<Duration>,
	},
	RestoreState(Vec<u8>),
}

impl EditAction {
	pub fn execute(&self, lyrics: &mut Lyrics, time_index: &mut TimeIndex) -> eyre::Result<()> {
		match self {
			EditAction::SetTimestamp { idx, timestamp } => {
				if lyrics.line_count() <= *idx {
					return Err(eyre::eyre!("Line index out of range"));
				}
				lyrics.lines_mut()[*idx as usize].set_timestamp(*timestamp);
				time_index.rebuild(lyrics.lines().iter());
			}
			EditAction::RestoreState(buffer) => {
				lyrics.read_overwrite(&buffer[..])?;
				time_index.rebuild(lyrics.lines().iter());
			}
		};
		Ok(())
	}
}
