## 命名規則

https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md

## サンプルコードを作る場合のディレクトリ構成

参考：https://yabutan.com/posts/221128_rust_snippets_code_project

#### 背景

src 配下に配置したサンプルコードに対して、コード補完やドキュメントヒントなどの拡張機能が働かない

#### 原因

rust のコンパイラは、 mod で指定されて初めてファイルをモジュールとして読み込む。それが拡張機能の補完にも繋がっているため。

#### 対応

##### 方法 1

`src/main.rs` から mod として読み込む

##### 方法 2

`src/bin`配下に`<FILE_NAME>.rs`、または、`<DIRECTORY_NAME/main.rs>`で配置。

```
cargo run --bin <FILE_NAME>
cargo run --bin <DIRECTORY_NAME>
```

で実行する。

##### 方法 3

examples ディレクトリ配下に`<FILE_NAME>.rs`、または、`<DIRECTORY_NAME/main.rs>`で配置。

```
cargo run --example <FILE_NAME>
cargo run --example <DIRECTORY_NAME>
```

で実行する。
