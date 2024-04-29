soralog
=======

[![soralog](https://img.shields.io/crates/v/soralog.svg)](https://crates.io/crates/soralog)
[![Actions Status](https://github.com/sile/soralog/workflows/CI/badge.svg)](https://github.com/sile/soralog/actions)
![License](https://img.shields.io/crates/l/soralog)

[WebRTC SFU Sora のログファイル](https://sora-doc.shiguredo.jp/LOG)の調査を行いやすくするためのコマンドラインツールです。

インストール
------------

```console
$ cargo install soralog
```

使い方
------

現時点では以下のように四つのコマンドが用意されています。

```console
$ soralog -h
WebRTC SFU Sora のログファイルの調査を行いやすくするためのコマンドラインツール

Usage: soralog <COMMAND>

Commands:
  list   ディレクトリを再帰的に辿って Sora のログファイルのパスを JSONL 形式で標準出力に列挙します
  cat    `soralog list` コマンドの出力結果を標準入力から受け取り、ログファイルの中身を JSONL 形式で標準出力に出力します
  count  ログメッセージ群を標準入力から受け取り、指定されたフィールドの値の出現回数をカウントします
  table  ログメッセージ群を標準入力から受け取り、指定されたフィールド群を列とした  Markdown のテーブル形式に変換して出力します。
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
