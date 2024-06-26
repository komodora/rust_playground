# 読書ノート

## Chapter1

### Section1

三大要素

- 安全性
- 速度
- 並行性

Rust とは「さび」のこと

### Section2

- Rust
  - コンパイラー方式
    - Python の何倍も高速に動く
    - OS やデータベースなど、システム寄りの開発に向いている

#### パッケージ管理

| 言語   | パッケージ管理システム |
| ------ | ---------------------- |
| Python | PyPI(pip)              |
| Rust   | Cargo(cargo)           |

#### 型

- Python
  - 動的型付き言語
  - 型ヒントはある
- Rust
  - 静的型付き言語
  - 型推論がある

#### メモリ管理

- Python
  - GC(ガベージコレクター)
    - 実行時のオーバーヘッドが大きい
    - 「参照カウント」を採用
- Rust
  - 所有権システム
    - 所有権とライフタイム

どちらも明示的にメモリを解放する必要はない

### Section3 開発環境

#### インストール方法

- Rust のインストール方法：省略
- VSCode のおすすめ拡張機能
  - (Rust の devcontainer を使ったときに、最初から入っていたもの)
  - rust-lang.rust-analyzer
  - serayuzgur.crates
  - vadimcn.vscode-lldb
  - tamasfe.even-better-toml

#### 実行方法

1. .rs ファイルにソースを記述
2. `rustc <file_path>`
3. `./<exe>`

### Section4 はじめての Rust

`println!`などの末尾の感嘆符は**マクロ**であることを示す

**値を変数に束縛する**

Rust は型に厳しい言語。整数型と実数型を明確に区別する

### Section5

- main 関数から始まる
- 関数定義は`fn`
- 1 行コメントは`//`
- 複数行コメントは`/* */`
- 文末にはセミコロンが必要
- for 文は 「for 変数 in 開始値..終了値+1」
- if 文は if, else if, else
- 条件文に丸括弧はいらない

### Section6

- 改行を出力しない`print!`マクロがある
- `"{:3}"`のように書くことで、出力の仕方を制御することができる

### Section7

- 変数定義は
  - `let 変数名 = 値;`
  - 値を変数に束縛するともいう
- 変更可能な変数を定義する場合
  - `let mut 変数名 = 値;`

**不変な変数が基本となっている => 「安全なプログラムを作ることが最優先」**

### Section8

- 型推論が有効になるのは関数の内側だけ
- Rust で扱える数値型は大きく分けて「符号あり整数」、「符号なし整数」、「浮動小数点数」
- 利用可能な数値の範囲を知る方法
  - `i8::MIN`, `i8::MAX`など
- 型変数を明示する場合
  - `let 変数名: 型名 = 値;`

### Section9

- 「文字」と「文字列」を明確に区別する
  - 文字
    - 例: 'A'
    - 型: char
    - 意味: 文字を表す
    - シングルクォートで囲まれた 1 文字
  - 文字列
    - 例: "A"
    - 型: &str
    - 意味: 文字列リテラル
    - ダブルクォートで囲まれた 0 文字以上の文字列
- **「as」は強制的な型変換を行う機能**
  - `let 変数名 = 変数 as 型`
- 関数宣言時は、引数や戻り値のデータ型を明示しないといけない
- 関数から値を戻す際に、2 通りの書き方がある
  - `値`
  - `return 値;`
- クロージャーの書き方
  - `let 名前 = |引数| 定義;`

### Section10

- for の範囲指定
  - range(1, n) <=> 1..n
- 関数の型の指定の仕方
  - `[usize; 100]`: 100 個の usize 型の配列
- 配列に対する操作
  - `arr[index] = value;`
  - `let v = arr[index];`

#### 参照とは

- **値そのものではなく、値を表す参照情報**
  - C 言語で言うポインター
- 関数呼び出しのとき
  - `&mut`を指定することでそれが可変参照であることを明示する
- 引数を定義するとき
  - `arg: &mut u32`
- 参照を実態に戻すとき
  - `*arg`
  - **デリファレンス**と呼ぶ

#### for 文の書き方

- ```
  for 変数 in イテレーター {
    // 繰り返し処理
  }
  ```
- `1..11`
  - 1 から 10 を順に返す
- `1..=10`
  - 1 から 10 を順に返す

## Chapter2

### Section1

Cargo とは

- ビルドシステム兼パッケージ管理システム
  - Rust ではライブラリのことを**クレート**と呼ぶ

プロジェクトの作り方

- `cargo new <hoge>`で新しいプロジェクトが作成できる
  - プロジェクト直下には`Cargo.toml`が作成される
    - **マニフェストファイル**と呼ばれる
- プログラムは`src/main.rc`から書き始める
- `cargo run`でビルドと実行を同時にできる

クレートの使い方

- `Cargo.toml`に依存関係を記述
  - `cargo add クレート名`でもよい
- `use`を使ってクレート内のモジュールを取り込む
- `cargo run`で実行する
  - インストールされていないクレートは自動でインストールされる

use 宣言の書き方

```
use クレート名::モジュール名;
use クレート名::モジュール名1::モジュール名2;
use クレート名::{モジュール名A, モジュール名B};
```

などがあるが、これらの宣言は**必須ではない**。これらの宣言をすることで、クレート名の指定をせずに使えるようになる

### Section2

範囲オブジェクト

- `std::ops::Range`に構造体として定義されている

```
let r = 3..15;
r.start # <- 3
r.end   # <- 15
```

定数の宣言方法

- 型の省略はできない

```
const 定数名: 型 = 値;
```

match 文の書き方

```rust
match 条件式 {
    値1 => 処理1,
    値2 => 処理2,
    値3 => 処理3,
    _ => 上記以外のときの処理,
}
```

### Section3

rand クレートの rand::seq:SliceRandom を使うと配列やベクターで shuffle メソッドが使えるようになる

> なぜ？

サイズが不定の配列を使いたいときは、ベクター型を利用する。
`vec!`マクロで生成できる

```rust
// マクロを使う場合
let mut nums = vec![1,2,3];
// マクロを使わない場合
let mut nums = Vec::new();
nums.push(1);
nums.push(2);
nums.push(3);
```

#### ジェネリクス

- 抽象的な方を指定して、さまざまなデータ型の操作を可能にするプログラミング手法
- ベクターの型`Vec<T>`の T の部分

### Section4

- `loop`文を使用することで、繰り返しの処理を書ける
  - `while true`は非推奨
- 関数の最後に書いた値を関数の戻り値にできる。その場合、末尾のセミコロンは省略する

#### 標準入力の受け取り方

- `std::io::stdin()`関数の`read_line()`メソッドを使う
  - 引数にはミュータブルな String 型を渡す
  - Result 型を返す

#### Result 型

Result 型は以下のような enum 型で定義されている

> enum 型とは

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 成功か失敗のいずれかを返す
- 成功時は T 型を保持する Ok を、失敗時は E 型を保持する Err が返される
- 以下のような match 文を使って処理を書き分ける
- もしくは、エラー時に`panic!`を呼びだす`expect()`, `unwrap()`を使うこともできる

> `panic!`とは

```rust
match r {
  Ok(t) => t,
  Err(f) => f,
}
```

[参考ページ](https://doc.rust-jp.rs/book-ja/ch09-00-error-handling.html)
