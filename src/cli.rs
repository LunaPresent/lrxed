use clap::Parser;

/// A tui application for synchronising lyrics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// Path to an audio file to open
	#[arg(short, long)]
	audio_file: String,
}

impl Args {
	pub fn audio_file(&self) -> Option<&str> {
		// this is a temporary measure; the actual field Args::audio_file
		// should be made optional when the file tree view is implemented
		Some(&self.audio_file)
	}
}
