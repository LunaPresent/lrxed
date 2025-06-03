use std::path::PathBuf;

use clap::Parser;
use cli::Args;
use directories::UserDirs;
use state::AppState;
use tui::{App, View};

use color_eyre::Result;

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
	let terminal = ratatui::init();

	let path = if let Some(path) = args.path {
		path
	} else if let Some(path) = UserDirs::new().as_ref().and_then(|x| x.audio_dir()) {
		path.to_owned()
	} else {
		PathBuf::from("/")
	};

	let mut state: AppState;

	if path.is_file() {
		state = AppState::new(View::Editor);

		let lrc_path = path.with_extension("lrc");
		state.audio.audio_player = Some(state.audio.audio_device.try_play(path)?);

		if lrc_path.exists() {
			state.lyrics.load_file(lrc_path)?;
		}
	} else {
		state = AppState::new(View::FileTree);
		state.file_browser.open_directory(&path);
	}

	let app_result = App.run(terminal, &mut state).await;
	ratatui::restore();
	app_result
}
