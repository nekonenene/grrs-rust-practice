# grrs

https://rust-cli.github.io/book/tutorial/index.html をもとに作成した、  
指定したファイルのテキストから  
特定の文字列がある行を出力するCLIプログラム。

## 使い方

git clone してきたあとで、以下のようにビルドと実行。

```sh
cargo run -- use src/main.rs
```

こういう出力がされる。

```
use anyhow::{Context, Result, Ok};
use clap::Parser;
use log::info;
use std::path::PathBuf;
```

`RUST_LOG` の指定をするとより詳しい情報が出力される。

```sh
RUST_LOG=info cargo run -- use src/main.rs

[2023-08-19T10:12:30Z INFO  grrs] pattern: "use"
[2023-08-19T10:12:30Z INFO  grrs] path: "src/main.rs"
use anyhow::{Context, Result, Ok};
use clap::Parser;
use log::info;
use std::path::PathBuf;
```
