use clap::Parser;

#[derive(Parser)]
struct Args {
    pattern: String,
    path: std::path::PathBuf,
}

// 実行例: argo run -- version Cargo.lock
fn main() {
    let args = Args::parse();

    println!("pattern: \"{}\"", args.pattern);

    // メモ： expect でエラーを出力する方法はあまりきれいじゃないらしい
    // https://rust-cli.github.io/book/tutorial/impl-draft.html
    let content = std::fs::read_to_string(&args.path).expect("ファイルが見つかりませんでした");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
