use clap::Parser;
use cli::Args;
use state::AppState;
use tui::{App, View};

use color_eyre::Result;

mod audio;
mod cli;
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
		state.audio_state.audio_player = Some(state.audio_state.audio_device.try_play(filename)?);
	} else {
		app = App::new(View::FileTree);
	}

	let app_result = app.run(terminal, &mut state).await;
	ratatui::restore();
	app_result
}
