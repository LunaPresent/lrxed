use std::path::PathBuf;

use ::config::{Config, ConfigError, File, FileFormat};
use clap::error::Result;
use notify::{RecommendedWatcher, RecursiveMode, Watcher as _};
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::system::{Commands, Query, Res, ResMut};
use oprabeli::ecs::*;
use oprabeli::event::DispatchMethod;
use thiserror::Error;

use super::{AppConfig, Keys, Settings, Theme};
use crate::event::AppEvent;

#[derive(Debug, Error)]
pub enum ConfigManagerError {
	#[error("error in file watch on config file")]
	FailedFileWatch(#[from] notify::Error),
	#[error("failed to parse config file")]
	FailedToParse(#[from] ConfigError),
	#[error("failed to convert config file path to string")]
	InvalidPath,
}

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ConfigManager {
	file_path: Option<PathBuf>,
	watcher: Option<RecommendedWatcher>,
}

impl UiComponent for ConfigManager {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::init), UiSystem::new(Self::update)]
	}
}

impl ConfigManager {
	pub fn new(file_path: Option<PathBuf>) -> Self {
		Self {
			file_path,
			watcher: None,
		}
	}

	fn init(
		context: InitContext,
		async_events: Res<AsyncEventQueue>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> Result<(), ConfigManagerError> {
		let mut comp = query
			.get_mut(context.entity)
			.expect("Self type component should be present on the entity");

		let config = comp.parse_config()?;

		cmd.insert_resource(config.keys);
		cmd.insert_resource(config.theme);
		cmd.insert_resource(config.settings);

		if let Some(file_path) = comp
			.file_path
			.as_ref()
			.and_then(|p| p.parent())
			.filter(|p| p.exists())
		{
			let mut async_events = async_events.clone();
			let mut watcher = notify::recommended_watcher(move |event| {
				if let Ok(event) = event {
					async_events.send(DispatchMethod::Target(context.entity), event);
				}
			})?;
			watcher.watch(file_path, RecursiveMode::Recursive)?;
			comp.watcher = Some(watcher);
		}

		Ok(())
	}

	fn update(
		context: EventContext<notify::Event>,
		query: Query<&Self>,
		mut keys: ResMut<Keys>,
		mut theme: ResMut<Theme>,
		mut settings: ResMut<Settings>,
		mut event_queue: ResMut<EventQueue>,
	) -> Result<EventFlow, ConfigManagerError> {
		let comp = query
			.get(context.entity)
			.expect("Self type component should be present on the entity");

		match context.event.kind {
			notify::EventKind::Create(_)
			| notify::EventKind::Modify(_)
			| notify::EventKind::Remove(_) => {
				let config = comp.parse_config()?;
				*keys = config.keys;
				*theme = config.theme;
				*settings = config.settings;
				event_queue.send(DispatchMethod::Broadcast, AppEvent::UpdateKeymap);
				Ok(EventFlow::Consume)
			}
			_ => Ok(EventFlow::Propagate),
		}
	}

	fn parse_config(&self) -> Result<AppConfig, ConfigManagerError> {
		let mut builder = Config::builder().add_source(File::from_str(
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.config/config.toml")),
			FileFormat::Toml,
		));
		if let Some(file_path) = &self.file_path {
			builder = builder.add_source(
				File::with_name(
					file_path
						.as_path()
						.to_str()
						.ok_or(ConfigManagerError::InvalidPath)?,
				)
				.required(false),
			);
		}

		Ok(builder.build()?.try_deserialize::<AppConfig>()?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_config_ok() {
		let config_manager = ConfigManager::new(None);
		let config = config_manager.parse_config();
		assert!(config.is_ok());
		let config = config.unwrap();
		assert!(config.keys.global.try_generate_key_map().is_ok());
		assert!(config.keys.editor.try_generate_key_map().is_ok());
		assert!(config.keys.file_browser.try_generate_key_map().is_ok());
		assert!(config.keys.confirm_box.try_generate_key_map().is_ok());
	}
}
