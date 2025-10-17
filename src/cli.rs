use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Args {}

pub struct Cli {
	#[allow(dead_code)]
	pub args: Args,
	pub proj_dirs: Option<ProjectDirs>,
}

impl Cli {
	pub fn new() -> Self {
		let args = Args::parse();
		let proj_dirs = ProjectDirs::from("net", "LunaPresent", "lrxed");
		Self { args, proj_dirs }
	}

	pub fn config_path(&self) -> Option<PathBuf> {
		Some(self.proj_dirs.as_ref()?.config_dir().join("config"))
	}
}

const VERSION_MESSAGE: &str = concat!(
	env!("CARGO_PKG_VERSION"),
	"-",
	env!("VERGEN_GIT_DESCRIBE"),
	" (",
	env!("VERGEN_BUILD_DATE"),
	")"
);

fn version() -> String {
	let author = clap::crate_authors!();

	format!(
		"\
{VERSION_MESSAGE}

Authors: {author}"
	)
}
