use std::{
	fs, io,
	path::{Path, PathBuf},
};

use clap::Parser;
use cli::Args;
use color_eyre::Result;
use directories::ProjectDirs;
use directories::UserDirs;
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

	if let Some(config_dir) = config_dir {
		if let Some(toml_path) = first_existing_file(config_dir, &[&"config.toml"]) {
			let toml_str = fs::read_to_string(toml_path)?;
			config = toml::from_str(&toml_str)?;
		} else if let Some(json_path) = first_existing_file(
			config_dir,
			&[&"config.json", &"config.jsonc", &"config.json5"],
		) {
			config = serde_json::from_reader(io::BufReader::new(fs::File::open(json_path)?))?;
		} else if let Some(yaml_path) =
			first_existing_file(config_dir, &[&"config.yaml", &"config.yml"])
		{
			config = serde_yml::from_reader(io::BufReader::new(fs::File::open(yaml_path)?))?;
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
		state = AppState::new(View::Editor);

		let lrc_path = path.with_extension("lrc");
		state.audio.audio_player = Some(state.audio.audio_device.try_play(path)?);

		state.lyrics.load_file_if_exists(lrc_path)?;
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
