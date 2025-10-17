mod controls;
mod entity_builder;

pub(crate) use controls::AppControls;
pub use entity_builder::EntityBuilder;

use std::{io, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc;

use super::{
	ecs::{ComponentSystem, DynEventDispatch},
	event::{EventSystem, SystemEvent},
	terminal::Terminal,
};
use controls::{AppSignalReceiver, SignalType};

/// Simple interface to setup and run your application
///
/// The `App` features a simple builder-style API with two actions:
/// - Add top-level components using [App::with_component] and [App::with_main_component]
/// - Run the app until completion with [App::run]
///
/// # Examples
/// ```no_run
/// # use std::time::Duration;
/// # use oprabeli::app::App;
/// # #[derive(Default, bevy_ecs::component::Component)]
/// # struct ConfigManager;
/// # #[derive(Default, bevy_ecs::component::Component)]
/// # struct RootComponent;
/// # #[derive(Default, bevy_ecs::component::Component)]
/// # struct FpsComponent;
/// #[tokio::main]
/// async fn main() -> eyre::Result<()> {
///     App::new()
///         .with_tick_interval(Duration::from_secs_f64(0.25))
///         .with_frame_interval(Duration::from_secs_f64(1./60.))
///         .with_entity(|e| {
///             e.with_component(ConfigManager::default())?
///                 .with_component(RootComponent::default())
///         })?
///         .with_entity(|e| e.with_component(FpsComponent::default()))?
///         .run()
///         .await
/// }
/// ```
#[derive(Debug)]
pub struct App {
	controls: AppControls,
	signal_receiver: AppSignalReceiver,
	event_system: EventSystem,
	async_events: mpsc::UnboundedReceiver<DynEventDispatch>,
	ecs: ComponentSystem,
	tick_interval: Duration,
	frame_interval: Duration,
}

impl Default for App {
	fn default() -> Self {
		Self::new()
	}
}

impl App {
	/// Creates a new `App`
	pub fn new() -> Self {
		let (controls, signal_receiver) = AppControls::new();
		let event_system = EventSystem::new();
		let (async_event_sender, async_events) = mpsc::unbounded_channel();
		let ecs = ComponentSystem::new(controls.clone(), async_event_sender);
		Self {
			controls,
			signal_receiver,
			event_system,
			async_events,
			ecs,
			tick_interval: Duration::from_millis(100),
			frame_interval: Duration::from_nanos(1),
		}
	}

	pub fn with_entity(
		mut self,
		f: impl FnOnce(EntityBuilder) -> eyre::Result<EntityBuilder>,
	) -> eyre::Result<Self> {
		Ok((f)(EntityBuilder::new(self.ecs.add_entity(), self))?.app())
	}

	pub fn with_tick_interval(mut self, interval: Duration) -> Self {
		self.tick_interval = interval.max(Duration::from_millis(1));
		self
	}

	pub fn with_frame_interval(mut self, interval: Duration) -> Self {
		self.frame_interval = interval.max(Duration::from_nanos(1));
		self
	}

	/// Runs the `App` until completion
	pub async fn run(mut self) -> eyre::Result<()> {
		let mut tui = Terminal::new()?;
		tui.enter()?;
		self.ecs.init()?;
		self.event_system.start(self.tick_interval)?;
		let mut frame_interval = tokio::time::interval(self.frame_interval);
		frame_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
		loop {
			tokio::select! {
				biased;
				Some(_) = Self::recv_stop(
					self.event_system.is_running(),
					&mut self.signal_receiver.stop,
				) => {
					self.event_system.stop().await?;
				}
				Some(ed) = self.async_events.recv() => {
					self.ecs.dispatch_dyn_event(ed)?;
				}
				ed = self.event_system.next() => {
					if let Some(event) = self.ecs.dispatch_system_event(ed?)? {
						self.handle_propagated_event(event)?;
					}
				}
				Some(signal) = self.signal_receiver.signal.recv() => {
					match signal {
						SignalType::Quit => break,
						SignalType::Suspend => self.suspend(&mut tui).await?,
					}
				}
				_ = frame_interval.tick() => {
					tui.try_draw(|frame| self.ecs.draw(frame).map_err(io::Error::other))?;
				}
			}
		}
		tui.exit()?;
		Ok(())
	}

	async fn recv_stop(
		is_running: bool,
		stop_receiver: &mut mpsc::UnboundedReceiver<()>,
	) -> Option<()> {
		if is_running {
			stop_receiver.recv().await
		} else {
			None
		}
	}

	async fn suspend(&mut self, tui: &mut Terminal) -> eyre::Result<()> {
		tui.suspend()?;

		tui.clear()?;
		tui.resume()?;
		self.event_system.start(self.tick_interval)?;
		Ok(())
	}

	fn handle_propagated_event(&mut self, event: SystemEvent) -> eyre::Result<()> {
		match event {
			SystemEvent::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
				..
			}) => self.controls.quit()?,
			SystemEvent::Key(KeyEvent {
				code: KeyCode::Char('z'),
				modifiers: KeyModifiers::CONTROL,
				..
			}) => self.controls.suspend()?,
			_ => (),
		}
		Ok(())
	}
}
