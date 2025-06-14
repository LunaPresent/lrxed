use std::cmp::{max, min};

use ratatui::layout::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
	target_pos: Position,
	editor_pos: Position,
	render_origin: Option<Position>,
	scroll: Position,
	screen_size: Position,
}

impl Cursor {
	pub fn set_x(&mut self, x: u16) -> &mut Self {
		self.target_pos.x = x;
		self
	}

	pub fn set_y(&mut self, y: u16) -> &mut Self {
		self.target_pos.y = y;
		self
	}

	pub fn update_pos(&mut self, line_lengths: impl Iterator<Item = u16>) -> &mut Self {
		let (len_y, len_x) =
			line_lengths
				.enumerate()
				.fold((0, 0), |(count, len_sel), (i, len_cur)| {
					(
						count + 1,
						if i as u16 <= self.target_pos.y {
							len_cur
						} else {
							len_sel
						},
					)
				});
		self.editor_pos.y = min(len_y - 1, self.target_pos.y);
		self.editor_pos.x = min(self.target_pos.x, max(len_x, 1) - 1);
		self
	}

	pub fn update_scroll(&mut self, text_size: Position, scrolloff: u16) -> &mut Self {
		self.update_vertical_scroll(
			text_size.y,
			self.screen_size.y,
			min(scrolloff, text_size.y / 2),
		);
		self.update_horizontal_scroll(text_size.x, self.screen_size.x, 0);
		self
	}

	fn update_vertical_scroll(&mut self, text_height: u16, screen_height: u16, scrolloff: u16) {
		self.scroll.y = min(
			self.editor_pos.y - min(self.editor_pos.y, scrolloff),
			self.scroll.y,
		);
		let scroll_bottom = max(
			min(self.editor_pos.y + scrolloff + 1, text_height),
			self.scroll.y + screen_height,
		);
		self.scroll.y = scroll_bottom - screen_height;
	}

	fn update_horizontal_scroll(&mut self, text_width: u16, screen_width: u16, scrolloff: u16) {
		self.scroll.x = min(
			self.editor_pos.x - min(self.editor_pos.x, scrolloff),
			self.scroll.x,
		);
		let scroll_end = max(
			min(self.editor_pos.x + scrolloff + 1, text_width),
			self.scroll.x + screen_width,
		);
		self.scroll.x = scroll_end - screen_width;
	}

	pub fn set_render_origin(&mut self, origin: Option<Position>) -> &mut Self {
		self.render_origin = origin;
		self
	}

	pub fn set_screen_size(&mut self, size: Position) -> &mut Self {
		self.screen_size = size;
		self
	}

	pub fn pos(&self) -> Position {
		self.editor_pos
	}

	pub fn render_pos(&self) -> Option<Position> {
		self.render_origin.map(|render_origin| {
			Position::new(
				self.editor_pos.x + render_origin.x - self.scroll.x,
				self.editor_pos.y + render_origin.y - self.scroll.y,
			)
		})
	}

	pub fn scroll(&self) -> Position {
		self.scroll
	}
}
