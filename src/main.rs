// https://rust-cli.github.io/book/tutorial/index.html
use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

// 実行例: argo run -- version Cargo.lock
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("pattern: \"{}\"", args.pattern);

    let result = std::fs::read_to_string(&args.path);

    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); },
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
