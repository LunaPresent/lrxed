use std::{
	fs::File,
	io::{BufRead, BufReader},
};

use color_eyre::eyre;

use crate::state::Coord;

use super::{Timestamp, lyric_line::LyricLine, metadata::Metadata};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Lyrics {
	metadata: Vec<Metadata>,
	lines: Vec<LyricLine>,
}

impl Lyrics {
	pub fn from_file(file: File) -> eyre::Result<Self> {
		let reader = BufReader::new(file);
		let mut lyrics = Lyrics::default();
		for line in reader.lines() {
			lyrics.parse_append(&line?);
		}
		Ok(lyrics)
	}

	pub fn parse_append(&mut self, s: &str) {
		let lines = s.lines();
		for line in lines {
			let mut it = line.trim_start().chars();
			let first = it.next();

			let (tag_delim, tag_end, _) = match first {
				Some('[') => it.enumerate().fold(
					(None, None, false),
					|(colon_i, close_i, escape), (i, c)| {
						if close_i.is_some() || escape {
							(colon_i, close_i, false)
						} else if colon_i.is_some() {
							match c {
								'\\' => (colon_i, None, true),
								']' => (colon_i, Some(i), false),
								_ => (colon_i, None, false),
							}
						} else {
							match c {
								'\\' => (None, None, true),
								':' => (Some(i), None, false),
								_ => (None, None, false),
							}
						}
					},
				),
				_ => (None, None, false),
			};

			if let (Some(tag_delim), Some(tag_end)) = (tag_delim, tag_end) {
				let line = line.trim_start();
				self.parse_append_line_with_tag(&line[1..tag_end], tag_delim, &line[tag_end + 2..]);
			} else {
				self.parse_append_line(line);
			}
		}
	}

	fn parse_append_line_with_tag(&mut self, tag: &str, tag_delim: usize, mut text: &str) {
		if let Ok(timestamp) = tag.parse() {
			if text.starts_with(' ') {
				text = &text[1..];
			}
			self.lines
				.push(LyricLine::new(Some(timestamp), text.to_owned()));
		} else {
			self.metadata.push(Metadata::parse_separate(
				&tag[..tag_delim],
				&tag[tag_delim + 1..],
			));
			text = text.trim();
			if !text.is_empty() {
				self.parse_append_line(text);
			}
		}
	}

	fn parse_append_line(&mut self, line: &str) {
		self.lines.push(LyricLine::new(None, line.to_owned()));
	}

	pub fn metadata(&self) -> impl Iterator<Item = &Metadata> {
		self.metadata.iter()
	}

	pub fn lines(&self) -> impl Iterator<Item = &LyricLine> {
		self.lines.iter()
	}

	pub fn line_count(&self) -> usize {
		self.lines.len()
	}

	pub fn time_at_line(&self, y: usize) -> Option<&Timestamp> {
		self.lines.get(y).and_then(|line| line.timestamp())
	}

	pub fn time_at_cursor(&self, cursor: Coord) -> Option<&Timestamp> {
		// TODO: karaoke: use x position to get word if possible, fallback to line
		self.lines.get(cursor.y).and_then(|line| line.timestamp())
	}
}
