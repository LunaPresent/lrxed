use std::io::{BufRead, Read, Write};

use color_eyre::eyre;
use unicode_width::UnicodeWidthStr;

use super::{Timestamp, lyric_line::LyricLine, metadata::Metadata};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lyrics {
	metadata: Vec<Metadata>,
	lines: Vec<LyricLine>,
	sync_percentage: u8,
}

impl Default for Lyrics {
	fn default() -> Self {
		Self {
			metadata: Default::default(),
			lines: vec![LyricLine::default()],
			sync_percentage: 0,
		}
	}
}

impl Lyrics {
	fn calc_sync_percentage(&self) -> u8 {
		let (synced_line_count, line_count) = self
			.lines()
			.iter()
			.filter(|line| !line.text().is_empty())
			.fold((0, 0), |(slc, lc), line| {
				(slc + line.timestamp().is_some() as u32, lc + 1)
			});

		(synced_line_count * 100 / line_count.max(1)) as u8
	}

	pub fn sync_percentage(&self) -> u8 {
		self.sync_percentage
	}

	pub fn read_overwrite(&mut self, reader: impl Read + BufRead) -> eyre::Result<()> {
		self.metadata.clear();
		self.lines.clear();
		for line in reader.lines() {
			self.parse_append(&format!("{}\n", &line?));
		}
		if self.lines.is_empty() {
			self.lines.push(Default::default());
		}

		self.sync_percentage = self.calc_sync_percentage();

		Ok(())
	}

	pub fn write_to(&self, writer: &mut impl Write) -> eyre::Result<()> {
		for line in &self.lines {
			if let Some(timestamp) = line.timestamp() {
				writeln!(writer, "[{}] {}", timestamp.text(), line.text())?;
			} else {
				writeln!(writer, "{}", line.text())?;
			}
		}
		writer.flush()?;
		Ok(())
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
				self.parse_append_line_with_tag(
					&line[1..tag_end + 1],
					tag_delim,
					&line[tag_end + 2..],
				);
			} else {
				self.parse_append_line(line);
			}
		}

		self.sync_percentage = self.calc_sync_percentage();
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

	pub fn lines(&self) -> &[LyricLine] {
		self.lines.as_slice()
	}

	pub fn line_count(&self) -> u16 {
		self.lines.len() as u16
	}

	pub fn line_widths(&self) -> impl Iterator<Item = u16> {
		self.lines.iter().map(|x| x.text().width() as u16)
	}

	pub fn time_at_line(&self, y: u16) -> Option<&Timestamp> {
		self.lines.get(y as usize).and_then(|line| line.timestamp())
	}

	pub fn time_at_cursor(&self, _x: u16, y: u16) -> Option<&Timestamp> {
		// TODO: karaoke: use x position to get word if possible, fallback to line
		self.lines.get(y as usize).and_then(|line| line.timestamp())
	}

	pub fn set_timestamp_at_line(&mut self, index: usize, timestamp: Option<impl Into<Timestamp>>) {
		self.lines[index].set_timestamp(timestamp);
		self.sync_percentage = self.calc_sync_percentage();
	}
}
