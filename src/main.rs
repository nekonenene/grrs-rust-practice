// https://rust-cli.github.io/book/tutorial/index.html
use anyhow::{Context, Result, Ok};
use clap::Parser;
use log::info;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(required = true, index = 1, help = "Find if each line contains this string")]
    pattern: String,
    #[arg(required = true, index = 2, help = "Text file path")]
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
        .with_context(|| format!("could not read file `{}`", args.path))?; // with_context(|| format!...) は JavaScript 的に書くと with_context(() => { format!... }) のこと

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
