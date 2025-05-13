use std::{fs::File, path::Path};

use clap::Parser;
use cli::Args;
use state::AppState;
use tui::{App, View};

use color_eyre::Result;

mod audio;
mod cli;
mod config;
mod lyrics;
mod state;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	color_eyre::install()?;
	let terminal = ratatui::init();

	let mut state = AppState::default();
	let mut app: App;

	if let Some(filename) = args.audio_file() {
		app = App::new(View::Editor);
		state.audio.audio_player = Some(state.audio.audio_device.try_play(filename)?);
		let lrc_path = Path::new(filename).with_extension("lrc");
		if lrc_path.exists() {
			state.lyrics.load_file(File::open(lrc_path)?)?;
		}
	} else {
		app = App::new(View::FileTree);
	}

	let app_result = app.run(terminal, &mut state).await;
	ratatui::restore();
	app_result
}
