use vergen_gitcl::{
    BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder, SysinfoBuilder,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&GitclBuilder::all_git()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()?;
    Ok(())
}
