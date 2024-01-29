use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use openstack_cli::Cli;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        Some("doc") => build_doc()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application and man pages
doc             builds man pages and doc
"
    )
}

fn dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary(None)?;
    dist_binary(Some("x86_64-unknown-linux-musl"))?;
    build_doc()?;

    Ok(())
}

fn dist_binary(target: Option<&str>) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut args = vec!["build", "--release"];
    if let Some(target) = target {
        args.push("--target");
        args.push(target);
    }
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(args)
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let mut dst = project_root().join("target");
    if let Some(target) = target {
        dst.push(target);
    }
    dst.push("release/osc");
    println!("release binary `{:?}`", dst);

    fs::copy(&dst, dist_dir().join("osc"))?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&dst).status()?;
        if !status.success() {
            Err("strip failed")?;
        }
    } else {
        eprintln!("no `strip` utility found")
    }

    Ok(())
}

fn dist_manpage() -> Result<(), DynError> {
    fs::write(
        dist_dir().join("osc.md"),
        clap_markdown::help_markdown::<Cli>(),
    )?;
    fs::copy(dist_dir().join("osc.md"), "doc/src/osc.md")?;
    Ok(())
}

fn build_doc() -> Result<(), DynError> {
    dist_manpage()?;
    Command::new("mdbook")
        .current_dir(project_root().join("doc"))
        .args(&["build"])
        .status()?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
