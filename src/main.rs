// https://rust-cli.github.io/book/tutorial/index.html
use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: String,
}

// 実行例: argo run -- version Cargo.lock
fn main() -> Result<()> {
    let args = Args::parse();

    let pathbuf = PathBuf::from(&args.path);

    println!("pattern: \"{}\"", args.pattern);

    let content = std::fs::read_to_string(&pathbuf)
        .with_context(|| format!("could not read file `{}`", args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
