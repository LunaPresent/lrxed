use bevy_ecs::component::Component;
use derive_more::{Deref, DerefMut};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};

use super::ViewportError;

// TODO: documentation
#[derive(Debug, Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct Area(pub Rect);

/// The relative z-order of a [UI component][u]
///
/// A larger z-order will cause an [entity][e]'s [UI component][u]s to be rendered later
/// in the pipeline. If an [entity][e] does not have a `ZOrder` [Component], it will be
/// considered to have a z-order of 0. A child's z-order is relative to its parent,
/// as such it is not possible to render a child before its parent.
///
/// Z-order is local to the [Viewport] in which the [entity][e] is rendered. Other than
/// that, and the *child z >= parent z* restriction, the render pipeline ignores
/// the usual tree structure in terms of render order, making sure entities are
/// rendered in a strictly increasing z order. The render pipeline **does not** clear
/// the area prior to rendering entities with a `ZOrder` [Component]. This is up to
/// the user's consideration as it can be trivially included at the start of the
/// [UI component][u]'s render system.
///
/// [u]: crate::tui::ecs::ui_component::UiComponent
/// [e]: bevy_ecs::entity::Entity
#[derive(Debug, Component, Default, Clone, Copy, Deref, DerefMut)]
pub struct ZOrder(pub u16);

// TODO: documentation
#[derive(Debug, Component, Default)]
#[require(Area)]
pub struct Viewport {
	pub(crate) buf: Buffer,
	pub offset: Position,
	pub size: Size,
}

impl Viewport {
	pub fn area(&self) -> Rect {
		(Position::ORIGIN, self.size).into()
	}

	pub fn clamp_offset(&mut self, target_area_size: Size) -> Result<(), ViewportError> {
		if self.size.width < target_area_size.width || self.size.height < target_area_size.height {
			return Err(ViewportError::TooSmall);
		}

		self.offset.x = self.offset.x.min(self.size.width - target_area_size.width);
		self.offset.y = self
			.offset
			.y
			.min(self.size.height - target_area_size.height);
		Ok(())
	}

	pub(crate) fn resize_and_get_buf_mut(&mut self) -> &mut Buffer {
		self.buf.resize(self.area());
		&mut self.buf
	}
}
