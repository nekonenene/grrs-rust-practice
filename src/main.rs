// https://rust-cli.github.io/book/tutorial/index.html
use anyhow::{Context, Result, Ok};
use clap::Parser;
use log::{trace, info};
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

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        trace!("line: {}", line);

        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

#[test]
fn check_answer_validity() {
    assert_eq!(42, 42);
}
