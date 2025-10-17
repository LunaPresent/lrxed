use core::mem;
use core::num::NonZero;

use bevy_ecs::entity::Entity;
use bevy_ecs::hierarchy::{ChildOf, Children};
use bevy_ecs::query::Without;
use bevy_ecs::system::{In, InMut, Query, RunSystemOnce};
use bevy_ecs::world::World;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::{Area, Viewport, ViewportError, ZOrder};
use crate::ecs::error_handling::UiSystemResultExt as _;
use crate::ecs::{BevyErrorWrapper, ui_component::*};

#[derive(Debug, Clone, Copy)]
struct EntityRenderInfo {
	entity: Entity,
	system: Option<RenderSystemId>,
	z_order: u32,
	next_queue_idx: Option<NonZero<u32>>,
}

#[derive(Debug)]
pub(crate) struct Renderer {
	queues_per_viewport: Vec<Vec<EntityRenderInfo>>,
	viewport_count: usize,
}

impl Default for Renderer {
	fn default() -> Self {
		Self {
			queues_per_viewport: vec![Vec::new()],
			viewport_count: 0,
		}
	}
}

impl Renderer {
	pub fn render(&mut self, buf: &mut Buffer, area: Rect, world: &mut World) -> eyre::Result<()> {
		for queue in &mut self.queues_per_viewport {
			queue.clear();
		}
		self.viewport_count = 0;
		world
			.run_system_once_with(Self::set_area_sizes, area)
			.map_err(BevyErrorWrapper::from)?;
		world
			.run_system_once_with(Self::find_render_targets, self)
			.map_err(BevyErrorWrapper::from)??;

		for queue in &mut self.queues_per_viewport {
			queue.sort_unstable_by(|a, b| {
				a.z_order
					.cmp(&b.z_order)
					.then(a.entity.cmp(&b.entity))
					.then(a.next_queue_idx.cmp(&b.next_queue_idx))
			});
		}
		self.render_queue(0, buf, world)?;

		Ok(())
	}

	fn set_area_sizes(In(frame_area): In<Rect>, areas: Query<&mut Area, Without<ChildOf>>) {
		for mut area in areas {
			**area = frame_area;
		}
	}

	#[allow(clippy::type_complexity, reason = "query is injected by bevy")]
	fn find_render_targets(
		InMut(this): InMut<Renderer>,
		root_entities: Query<Entity, Without<ChildOf>>,
		query: Query<(
			Option<&RenderSystemCollection>,
			Option<&Children>,
			Option<&ZOrder>,
			Option<&Viewport>,
		)>,
	) -> eyre::Result<()> {
		for root in root_entities {
			this.recurse_render_targets(root, 0, 0, query)?;
		}
		Ok(())
	}

	#[allow(clippy::type_complexity, reason = "query is injected by bevy")]
	fn recurse_render_targets(
		&mut self,
		entity: Entity,
		queue_idx: usize,
		z_parent: u32,
		query: Query<(
			Option<&RenderSystemCollection>,
			Option<&Children>,
			Option<&ZOrder>,
			Option<&Viewport>,
		)>,
	) -> eyre::Result<()> {
		let (systems, children, z_order, viewport) = query.get(entity)?;
		let z_order = z_parent.max(z_order.map(|x| **x as u32).unwrap_or_default() << 16);
		let next_queue_idx = if viewport.is_some() {
			self.viewport_count += 1;
			if self.viewport_count >= self.queues_per_viewport.len() {
				self.queues_per_viewport
					.resize_with(self.viewport_count + 1, Vec::new);
			}
			NonZero::new(self.viewport_count as u32)
		} else {
			None
		};

		if let Some(systems) = systems {
			for &system in systems.iter() {
				let context = EntityRenderInfo {
					entity,
					system: Some(system),
					z_order,
					next_queue_idx: None,
				};
				self.queues_per_viewport[queue_idx].push(context);
			}
			self.queues_per_viewport[queue_idx]
				.last_mut()
				.unwrap()
				.next_queue_idx = next_queue_idx;
		} else {
			let context = EntityRenderInfo {
				entity,
				system: None,
				z_order,
				next_queue_idx,
			};
			self.queues_per_viewport[queue_idx].push(context);
		}

		let queue_idx = next_queue_idx
			.map(|x| x.get() as usize)
			.unwrap_or(queue_idx);
		if let Some(children) = children {
			for &child in children {
				self.recurse_render_targets(child, queue_idx, z_order + 1, query)?;
			}
		}

		Ok(())
	}

	fn render_queue(
		&self,
		queue_idx: usize,
		buf: &mut Buffer,
		world: &mut World,
	) -> eyre::Result<()> {
		for render_info in &self.queues_per_viewport[queue_idx] {
			if let Some(system) = render_info.system {
				world
					.run_system_with(
						system,
						RenderContext {
							entity: render_info.entity,
							buffer: buf,
						},
					)?
					.handle(render_info.entity, world)?;
			}
			if let Some(next_queue_idx) = render_info.next_queue_idx {
				world
					.run_system_once_with(Self::prepare_viewport, (render_info.entity, buf))
					.map_err(BevyErrorWrapper::from)??;
				self.render_queue(next_queue_idx.get() as usize, buf, world)?;
				world
					.run_system_once_with(Self::cleanup_viewport, (render_info.entity, buf))
					.map_err(BevyErrorWrapper::from)??;
			}
		}

		Ok(())
	}

	fn prepare_viewport(
		(In(entity), InMut(buf)): (In<Entity>, InMut<Buffer>),
		mut query: Query<&mut Viewport>,
	) -> Result<(), ViewportError> {
		let mut viewport = query.get_mut(entity).map_err(|_| ViewportError::Missing)?;
		mem::swap(viewport.resize_and_get_buf_mut(), buf);
		buf.reset();
		Ok(())
	}

	fn cleanup_viewport(
		(In(entity), InMut(buf)): (In<Entity>, InMut<Buffer>),
		mut query: Query<(&mut Viewport, &Area)>,
	) -> Result<(), ViewportError> {
		let (mut viewport, area) = query.get_mut(entity).map_err(|_| ViewportError::Missing)?;
		mem::swap(&mut viewport.buf, buf);
		Self::combine_viewports(buf, &viewport, **area)?;
		Ok(())
	}

	fn combine_viewports(
		dst: &mut Buffer,
		src: &Viewport,
		area: Rect,
	) -> Result<(), ViewportError> {
		let rect = area.intersection(dst.area);
		if src.size.width < rect.width || src.size.height < rect.height {
			return Err(ViewportError::TooSmall);
		}
		if src.offset.x > src.size.width - rect.width
			|| src.offset.y > src.size.height - rect.height
		{
			return Err(ViewportError::OffsetOutOfBounds);
		}

		for y_off in 0..rect.height {
			let y_src = src.offset.y + y_off;
			let i_src = src.buf.index_of(src.offset.x, y_src);
			let i_dst = dst.index_of(rect.x, rect.y + y_off);

			dst.content[i_dst..i_dst + rect.width as usize]
				.clone_from_slice(&src.buf.content[i_src..i_src + rect.width as usize]);
		}

		Ok(())
	}
}
