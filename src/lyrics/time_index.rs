use std::{cmp, time::Duration};

use super::lyric_line::LyricLine;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TimeIndexEntry {
	pub time: Duration,
	pub line_num: Option<u16>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TimeIndexHint {
	idx: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeIndex {
	entries: Vec<TimeIndexEntry>,
}

impl Default for TimeIndex {
	fn default() -> Self {
		Self {
			entries: vec![TimeIndexEntry {
				time: Duration::ZERO,
				line_num: None,
			}],
		}
	}
}

impl TimeIndex {
	pub fn new<'a>(lyrics: impl Iterator<Item = &'a LyricLine>) -> Self {
		let mut x = Self {
			entries: Vec::default(),
		};
		x.rebuild(lyrics);
		x
	}

	pub fn rebuild<'a>(&mut self, lyrics: impl Iterator<Item = &'a LyricLine>) {
		self.entries.clear();
		self.entries.extend(
			std::iter::once(TimeIndexEntry {
				time: Duration::ZERO,
				line_num: None,
			})
			.chain(
				lyrics
					.enumerate()
					.filter_map(|(i, line)| match line.timestamp() {
						Some(timestamp) => Some(TimeIndexEntry {
							time: timestamp.time(),
							line_num: Some(i as u16),
						}),
						None => None,
					}),
			),
		);
		self.entries
			.sort_unstable_by(|a, b| a.time.cmp(&b.time).then(a.line_num.cmp(&b.line_num)));
	}

	pub fn find_random(&self, time: Duration) -> (TimeIndexEntry, TimeIndexHint) {
		let mut from = 0;
		let mut to = self.entries.len();
		while from != to {
			let idx = (to + from) / 2;
			if time < self.entries[idx].time {
				to = idx;
			} else if idx + 1 < self.entries.len() && time > self.entries[idx + 1].time {
				from = idx + 1;
			} else {
				return (self.entries[idx], TimeIndexHint { idx });
			}
		}
		(
			TimeIndexEntry {
				time: Duration::ZERO,
				line_num: None,
			},
			TimeIndexHint { idx: 0 },
		)
	}

	pub fn find_seq(&self, time: Duration, hint: TimeIndexHint) -> (TimeIndexEntry, TimeIndexHint) {
		let mut idx = cmp::min(hint.idx, self.entries.len() - 1);
		while time < self.entries[idx].time {
			if idx == 0 {
				return (
					TimeIndexEntry {
						time: Duration::ZERO,
						line_num: None,
					},
					TimeIndexHint { idx: 0 },
				);
			}
			idx -= 1;
		}
		while idx + 1 < self.entries.len() && time > self.entries[idx + 1].time {
			idx += 1;
		}
		(self.entries[idx], TimeIndexHint { idx })
	}
}
