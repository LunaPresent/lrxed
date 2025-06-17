use std::{
	fs, io,
	path::{Path, PathBuf},
};

use clap::Parser;
use cli::Args;
use color_eyre::{Result, eyre::OptionExt};
use directories::ProjectDirs;
use directories::UserDirs;
use song::Song;
use state::{AppState, Config};
use tui::{App, View};

mod audio;
mod cli;
mod config;
mod lyrics;
mod song;
mod state;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	color_eyre::install()?;

	let mut config = Config::default();

	let user_dirs = UserDirs::new();
	let project_dirs = ProjectDirs::from("", "LunaPresent", "lrxed");
	let config_dir = if let Some(project_dirs) = &project_dirs {
		Some(project_dirs.config_dir())
	} else if let Some(home_dir) = user_dirs.as_ref().map(|x| x.home_dir()) {
		Some(home_dir)
	} else {
		None
	};

	if let Some(file_type) = args.print_config_path {
		let config_dir = &config_dir.ok_or_eyre("Config directory could not be determined")?;
		match file_type.unwrap_or_default() {
			cli::ConfigFiletype::Toml => println!("{}/config.toml", config_dir.to_string_lossy()),
			cli::ConfigFiletype::Json => println!(
				"{}",
				first_existing_file(
					config_dir,
					&[&"config.json", &"config.jsonc", &"config.json5"],
				)
				.map_or(
					format!("{}/config.json", config_dir.to_string_lossy()),
					|path| path.to_string_lossy().into_owned()
				)
			),
			cli::ConfigFiletype::Yaml => println!(
				"{}",
				first_existing_file(config_dir, &[&"config.yaml", &"config.yml"],).map_or(
					format!("{}/config.yaml", config_dir.to_string_lossy()),
					|path| path.to_string_lossy().into_owned()
				)
			),
		};
		return Ok(());
	}

	let config_path = if args.config.is_some() {
		args.config
	} else if let Some(config_dir) = config_dir {
		first_existing_file(
			config_dir,
			&[
				&"config.toml",
				&"config.json",
				&"config.jsonc",
				&"config.json5",
				&"config.yaml",
				&"config.yml",
			],
		)
	} else {
		None
	};

	if let Some(config_path) = config_path {
		match config_path.extension().and_then(|ext| ext.to_str()) {
			Some("toml") => {
				let toml_str = fs::read_to_string(config_path)?;
				config = toml::from_str(&toml_str)?;
			}
			Some("json" | "jsonc" | "json5") => {
				config = serde_json::from_reader(io::BufReader::new(fs::File::open(config_path)?))?;
			}
			Some("yaml" | "yml") => {
				config = serde_yml::from_reader(io::BufReader::new(fs::File::open(config_path)?))?;
			}
			_ => (),
		}
	}

	if let Some(file_type) = args.print_config {
		match file_type.unwrap_or_default() {
			cli::ConfigFiletype::Toml => println!("{}", toml::to_string_pretty(&config)?),
			cli::ConfigFiletype::Json => {
				println!("{}", serde_json::to_string_pretty(&config)?)
			}
			cli::ConfigFiletype::Yaml => {
				println!("{}", serde_yml::to_string(&config)?)
			}
		};
		return Ok(());
	}

	let mut path = if let Some(path) = args.path {
		path
	} else if let Some(path) = &config.settings.default_path {
		path.to_owned()
	} else if let Some(path) = user_dirs.as_ref().and_then(|x| x.audio_dir()) {
		path.to_owned()
	} else {
		PathBuf::from("/")
	};

	if let Ok(remaining) = path.strip_prefix("~") {
		if let Some(home_dir) = user_dirs.as_ref().map(|dirs| dirs.home_dir()) {
			path = home_dir.join(remaining);
		}
	}

	let mut state: AppState;

	if path.is_file() {
		let song = Song::from_file(&path)?;
		state = AppState::new(View::Editor);

		state.audio.audio_player = Some(state.audio.audio_device.try_play(song.mp3_file)?);

		state.lyrics.load_file_if_exists(song.lrc_file)?;
	} else {
		state = AppState::new(View::FileTree);
		state.file_browser.open_directory(&path)?;
	}

	state.config = config;

	let terminal = ratatui::init();
	let app_result = App.run(terminal, &mut state).await;
	ratatui::restore();
	app_result
}

fn first_existing_file<P>(directory: &Path, file_names: &[&P]) -> Option<PathBuf>
where
	P: AsRef<Path>,
{
	file_names
		.iter()
		.map(|f| directory.join(f))
		.find(|path| path.is_file() && path.exists())
}
