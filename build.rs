use anyhow::Result;
use vergen_git2::{BuildBuilder, CargoBuilder, Emitter, Git2Builder};

fn main() -> Result<()> {
	let build = BuildBuilder::all_build()?;
	let git = Git2Builder::all_git()?;
	let cargo = CargoBuilder::all_cargo()?;
	Emitter::default()
		.add_instructions(&build)?
		.add_instructions(&git)?
		.add_instructions(&cargo)?
		.emit()
}
