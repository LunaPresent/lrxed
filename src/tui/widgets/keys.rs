use std::cmp;
use std::iter;

use ratatui::layout::Flex;
use ratatui::text::Span;
use ratatui::{
	layout::{Constraint, Layout},
	widgets::{StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

use crate::config::Action;
use crate::config::KeyChord;
use crate::{config::Context, state::AppState};

pub struct KeysWidget;

impl StatefulWidget for KeysWidget {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let line_count = state
			.config
			.keys
			.iter()
			.fold(0, |count, (_, it)| count + it.count() + 2);

		state.modal.popup_scroll =
			cmp::min(state.modal.popup_scroll, line_count as u16 - area.height);

		let line_areas = Layout::vertical(Constraint::from_lengths(iter::repeat_n(
			1,
			line_count.min(area.height as usize),
		)))
		.split(area);

		let key_str_max = state
			.config
			.keys
			.iter()
			.flat_map(|(_, key_bindings)| {
				key_bindings.map(|(key_chord, _)| key_chord.to_string().width())
			})
			.max()
			.unwrap_or_default() as u16;
		let key_binding_layout =
			Layout::horizontal([Constraint::Length(key_str_max), Constraint::Fill(1)]).spacing(1);

		let lines = state
			.config
			.keys
			.iter()
			.flat_map(|(context, key_bindings)| {
				iter::once(Line::ContextHeader(context))
					.chain(
						key_bindings.map(|(key_chord, action)| Line::KeyBinding(key_chord, action)),
					)
					.chain(iter::once(Line::Blank))
			})
			.skip(state.modal.popup_scroll as usize);

		for (line, &area) in lines.zip(line_areas.iter()) {
			match line {
				Line::Blank => (),
				Line::ContextHeader(context) => {
					let s: &str = context.into();
					Span::from(s)
						.style(state.config.theme.title)
						.render(area, buf);
				}
				Line::KeyBinding(key_chord, action) => {
					let [key_area, action_area] = key_binding_layout.areas(area);
					let key_span =
						Span::from(key_chord.to_string()).style(state.config.theme.accent);
					let [key_area] =
						Layout::horizontal([Constraint::Length(key_span.width() as u16)])
							.flex(Flex::End)
							.areas(key_area);
					key_span.render(key_area, buf);
					Span::from(action.to_string()).render(action_area, buf);
				}
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Line {
	Blank,
	ContextHeader(Context),
	KeyBinding(KeyChord, Action),
}
