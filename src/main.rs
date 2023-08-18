use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("pattern = {}", args.pattern);
}
