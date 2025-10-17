use std::cmp;
use std::iter;

use ratatui::layout::Flex;
use ratatui::text::Span;
use ratatui::{
	layout::{Constraint, Layout},
	widgets::{StatefulWidget, Widget},
};
use strum::IntoDiscriminant;
use unicode_width::UnicodeWidthStr;

use crate::config::Action;
use crate::config::Context;
use crate::config::KeyChord;
use crate::state::AppState;
use crate::state::ModalState;

pub struct KeysWidget;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Line<'a> {
	Blank,
	ContextHeader(&'a str),
	KeyBinding(&'a str, &'a str),
}

fn get_keys_sorted(
	modal_state: &mut ModalState,
	keys: impl Iterator<Item = (Context, impl Iterator<Item = (KeyChord, Action)>)>,
) -> impl Iterator<Item = (&str, impl Iterator<Item = (&str, &str)>)> {
	modal_state
		.keys_view_cache
		.get_or_insert_with(|| {
			keys.map(|(context, key_bindings)| {
				let mut kb: Vec<_> = key_bindings.collect();
				kb.sort_unstable_by(|(_, action_lhs), (_, action_rhs)| {
					action_lhs
						.discriminant()
						.cmp(&action_rhs.discriminant())
						.then_with(|| action_lhs.to_string().cmp(&action_rhs.to_string()))
				});
				let context_str: &str = context.into();
				let key_bindings_vec: Vec<(String, String)> = kb
					.iter()
					.map(|(key_chord, action)| (key_chord.to_string(), action.to_string()))
					.collect();
				(context_str.to_owned(), key_bindings_vec)
			})
			.filter(|(_, key_bindings)| !key_bindings.is_empty())
			.collect()
		})
		.iter()
		.map(|(context, key_bindings)| {
			(
				context.as_str(),
				key_bindings
					.iter()
					.map(|(key_chord, action)| (key_chord.as_str(), action.as_str())),
			)
		})
}

impl StatefulWidget for KeysWidget {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let line_count = get_keys_sorted(&mut state.modal, state.config.keys.iter())
			.fold(0, |count, (_, it)| count + it.count() + 2)
			- 1;

		state.modal.popup_scroll =
			cmp::min(state.modal.popup_scroll, line_count as u16 - area.height);

		let line_areas = Layout::vertical(Constraint::from_lengths(iter::repeat_n(
			1,
			line_count.min(area.height as usize),
		)))
		.split(area);

		let key_str_max = get_keys_sorted(&mut state.modal, state.config.keys.iter())
			.flat_map(|(_, key_bindings)| key_bindings.map(|(key_chord, _)| key_chord.width()))
			.max()
			.unwrap_or_default() as u16;
		let key_binding_layout =
			Layout::horizontal([Constraint::Length(key_str_max), Constraint::Fill(1)]).spacing(1);

		let scroll = state.modal.popup_scroll;
		let lines = get_keys_sorted(&mut state.modal, state.config.keys.iter())
			.flat_map(|(context, key_bindings)| {
				iter::once(Line::ContextHeader(context))
					.chain(
						key_bindings.map(|(key_chord, action)| Line::KeyBinding(key_chord, action)),
					)
					.chain(iter::once(Line::Blank))
			})
			.skip(scroll as usize);

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
