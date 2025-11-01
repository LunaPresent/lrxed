use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::system::{Query, Res};
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Constraint, Layout};
use oprabeli::ratatui::widgets::{Block, BorderType, Borders, WidgetRef as _};

use crate::config::Theme;

#[derive(Debug, Component, Default)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct AlbumCardComponent {}

impl UiComponent for AlbumCardComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::render)]
	}
}

impl AlbumCardComponent {
	fn render(
		context: RenderContext,
		theme: Res<Theme>,
		focus: Res<Focus>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (_comp, area) = query.get(context.entity)?;
		let area = **area;
		let has_focus = focus.target == context.entity;
		let border_colour = if has_focus {
			theme.colours.border_active
		} else {
			theme.colours.border_inactive
		};

		let block = Block::bordered()
			.border_type(BorderType::Rounded)
			.border_style(border_colour);
		block.render_ref(area, context.buffer);
		let area = block.inner(area);

		let [_image_area, info_area] = Layout::vertical([
			Constraint::Length((area.width as f32 / 2.2) as u16),
			Constraint::Fill(1),
		])
		.areas(area);

		let block = Block::new()
			.borders(Borders::TOP)
			.border_style(border_colour);
		block.render_ref(info_area, context.buffer);

		Ok(())
	}
}
