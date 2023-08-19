// https://rust-cli.github.io/book/tutorial/index.html
use anyhow::{Context, Result, Ok};
use clap::Parser;
use log::info;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
}

// 実行例: argo run -- version Cargo.lock
fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    info!("pattern: \"{}\"", args.pattern);
    info!("path: \"{}\"", args.path);

    let pathbuf = PathBuf::from(&args.path);

    let content = std::fs::read_to_string(&pathbuf)
        .with_context(|| format!("could not read file `{}`", args.path))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
