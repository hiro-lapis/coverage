# coverage

## About

[cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) を使ったカバレッジ可視化検証リポジトリです。

## set up

```
cargo llvm-cov
```

上記コマンドでテスト実行しカバレッジを取得できます。

初回実行時は以下のようにツールチェーンのインストールを要求されるので、Yesにしましょう。

```
info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
I will run `rustup component add llvm-tools-preview --toolchain stable-aarch64-apple-darwin` to install the `llvm-tools-preview` component for the selected toolchain.
Proceed? [Y/n]
```

以下のような出力を得られれば成功です。

<img width="500" src="img/coverage-capture.png">

### その他

- HTMLで出力する

```
// target/llvm-cov/html
cargo llvm-cov --html && open target/llvm-cov/html/index.html

// add/test-result
cargo llvm-cov --html --output-dir test-result
```

- プレーンテキスト出力する

```
cargo llvm-cov --output-path result_text
```


- カバレッジをエディタ内で行単位で確認

1. 下記コマンドでカバレッジ出力

```
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

2. Plugin[Coverage Gutters](https://open-vsx.org/extension/ryanluker/vscode-coverage-gutters)をインストール

3. インストールすると、エディタ下部(Git Graphの右横あたり)に⚪︎Watchnのアイコンが出るのでクリック。ファイルを開くとカバレッジが見れる
