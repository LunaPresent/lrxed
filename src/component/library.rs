use std::iter;

use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::entity::Entity;
use oprabeli::bevy_ecs::system::{Commands, Query, Res, ResMut};
use oprabeli::ratatui::layout::{Flex, Layout};
use oprabeli::ratatui::style::Stylize as _;
use oprabeli::ratatui::widgets::{Block, Widget as _};
use oprabeli::{ecs::*, event::DispatchMethod};

use super::AlbumCardComponent;
use crate::{app_event::AppEvent, config::Theme, util::Direction};

const CARD_WIDTH: u16 = 22;
const CARD_HEIGHT: u16 = 14;
const HORIZONTAL_GAP: u16 = 3;
const VERTICAL_GAP: u16 = 1;

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct LibraryComponent {
	album_cards: Vec<Entity>,
	cards_per_row: u16,
	selected_idx: u16,
}

impl UiComponent for LibraryComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::new(Self::init),
			UiSystem::new(Self::update),
			UiSystem::new(Self::render),
		]
	}
}

impl Default for LibraryComponent {
	fn default() -> Self {
		Self {
			album_cards: Vec::default(),
			cards_per_row: 1,
			selected_idx: 0,
		}
	}
}

impl LibraryComponent {
	fn init(
		context: InitContext,
		mut focus: ResMut<Focus>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(context.entity)?;
		let mut ec = cmd.entity(context.entity);

		comp.album_cards.reserve(50);
		for _ in 0..50 {
			comp.album_cards
				.push(ec.spawn_child(AlbumCardComponent::default()).id());
		}
		focus.target = *comp.album_cards.first().unwrap();

		Ok(())
	}

	fn update(
		context: EventContext<AppEvent>,
		mut focus: ResMut<Focus>,
		mut event_queue: ResMut<EventQueue>,
		mut query: Query<&mut Self>,
		areas: Query<&Area>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(context.entity)?;
		let flow = match context.event {
			AppEvent::MoveCursor(direction) => {
				comp.move_cursor(*direction);
				if let Some(target) = comp.album_cards.get(comp.selected_idx as usize) {
					focus.target = *target;
					let area = areas.get(*target)?;
					event_queue.send(
						DispatchMethod::Target(context.entity),
						AppEvent::ScrollTo(**area),
					);
				}
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		};
		Ok(flow)
	}

	fn render(
		context: RenderContext,
		theme: Res<Theme>,
		mut query: Query<&mut Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(context.entity)?;
		let area = **areas.get(context.entity)?;

		Block::new()
			.bg(theme.colours.background)
			.render(area, context.buffer);

		let horizontal_fit = (area.width / (CARD_WIDTH + HORIZONTAL_GAP)) as usize;
		let vertical_fit = (area.height / (CARD_HEIGHT + VERTICAL_GAP)) as usize;

		comp.cards_per_row = horizontal_fit as u16;

		let columns = Layout::horizontal(iter::repeat_n(CARD_WIDTH, horizontal_fit))
			.spacing(HORIZONTAL_GAP)
			.flex(Flex::Center);
		let rows =
			Layout::vertical(iter::repeat_n(CARD_HEIGHT, vertical_fit)).spacing(VERTICAL_GAP);

		for i in 0..50 {
			**areas.get_mut(comp.album_cards[i])? = Default::default();
		}
		for (y, &row) in rows.split(area).iter().enumerate() {
			for (x, &column) in columns.split(row).iter().enumerate() {
				let i = x + y * horizontal_fit;
				if i >= comp.album_cards.len() {
					break;
				}
				**areas.get_mut(comp.album_cards[i])? = column;
			}
		}

		Ok(())
	}

	fn move_cursor(&mut self, direction: impl Direction) {
		if self.album_cards.is_empty() {
			return;
		}
		let height = (self.album_cards.len() as u16 - 1) / self.cards_per_row + 1;
		let x = (self.selected_idx % self.cards_per_row)
			.saturating_add_signed(direction.x())
			.min(self.cards_per_row - 1);
		let y = (self.selected_idx / self.cards_per_row)
			.saturating_add_signed(direction.y())
			.min(height - 1);
		self.selected_idx = (x + self.cards_per_row * y).min(self.album_cards.len() as u16 - 1);
	}
}
