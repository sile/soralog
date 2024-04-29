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

Linux および macOS の場合には[リリースページ](https://github.com/sile/soralog/releases)からビルド済みバイナリを取得することも可能です。

使い方
------

現時点では `list`, `cat`, `count`, `table` の四つのコマンドが用意されています。

```console
$ soralog --help
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

基本的には `soralog list` と `soralog cat` が出力した JSON 形式のメッセージ群を、
（必要に応じて） `grep` や `jq` でフィルターや変形した上で、
`soralog count` や `soralog table` を使って分析する、という流れになります。

### `soralog list`

現在のディレクトリ以下を再帰的に辿って Sora のログファイルのパスを出力するコマンドです。

```console
$ soralog list
"sora/log/session_webhook.jsonl"
"sora/log/cluster.jsonl"
"sora/log/debug.jsonl"
"sora/log/internal.jsonl"
"sora/log/event_webhook.jsonl"
"sora/log/sora.jsonl"
"sora/log/crash.log"
"sora/log/signaling.jsonl"
"sora/log/auth_webhook.jsonl"
```

### `soralog cat`

`soralog list` の結果を受け取って、各ログファイルに含まれる JSON メッセージを
（独自フィールドをいくつか追加した上で）出力するコマンドです。

```console
$ soralog cat --help
`soralog list` コマンドの出力結果を標準入力から受け取り、ログファイルの中身を JSONL 形式で標準出力に出力します

通常の `cat` コマンドとは異なり、以下の特別な処理を行います。

### 1. 各メッセージには @domain, @type, @path という特別なフィールドが追加される

@domain フィールドには `{{ ログファイルの種類 }}_{{ sora.jsonl などの domain の値を _ で連結したもの }}` が 値として格納されます。

@type フィールドには、各ログファイル毎に異なるメッセージの種別を表す項目を統一的に扱うためのフィールドの 値が格納されます。 例えばイベントウェブフックログなら type フィールドが、API ログなら operation フィールドがこれに該当します。

@path フィールドには、ログファイルのパスが格納されます。

### 2. crash.log の中身はパースされ、JSONL 形式に変換される

@domain, @path, @raw_report を持つメッセージが出力されます

Usage: soralog cat

Options:
  -h, --help
          Print help (see a summary with '-h')

$ soralog list | soralog cat | head -1 | jq .
{
  "id": "EZDA1YTXJ10TQ3E2TKFFRTPS54",
  "timestamp": "2024-04-25T07:15:19.333877Z",
  "req": {
    "id": "JMWEH9SEPS0TQEK7AC6RW507FM",
    "label": "WebRTC SFU Sora",
    "timestamp": "2024-04-25T07:15:19.333854Z",
    "type": "session.created",
    "version": "2024.1.0-canary.50",
    "node_name": "sora@127.0.0.1",
    "session_id": "5RFT9ZWGA9003DDG099WZY1JRR",
    "channel_id": "sora",
    "multistream": true,
    "spotlight": false,
    "created_time": 1714029319,
    "created_timestamp": "2024-04-25T07:15:19.326856Z"
  },
  "@domain": "session_webhook",
  "@path": "sora/log/session_webhook.jsonl",
  "@type": "session.created"
}
```

### `soralog count`

`soralog cat` の結果を受け取って、指定のフィールドの各値の出現数をカウントするためのコマンドです。
ログファイル群の内容を要約して、全体像を把握しつつ詳細を絞り込んでいくために利用可能です。

```console
$ soralog count --help
ログメッセージ群を標準入力から受け取り、指定されたフィールドの値の出現回数をカウントします

Usage: soralog count [KEYS]...

Arguments:
  [KEYS]...  カウント対象のフィールド名（複数指定時にはその分だけ出力オブジェクトの階層が深くなる）

Options:
  -h, --help  Print help

// ログレベル毎にメッセージの出力数をカウントする（ログレベルがないものは _OTHER_ 扱い)
$ soralog list | soralog cat | soralog count level
{
  "_OTHER_": 40,
  "error": 6,
  "info": 78,
  "notice": 10,
  "warning": 6
}

// さらに @domain で細分化する
$ soralog list | soralog cat | soralog count level @domain
{
  "_OTHER_": {
    "auth_webhook": 5,
    "crash": 3,
    "event_webhook": 3,
    "session_webhook": 2,
    "signaling": 27
  },
  "error": {
    "internal": 1,
    "sora_otp_sasl": 3,
    "sora_sora_rtp": 2
  },
  "info": {
    "cluster_ra": 25,
    "cluster_sora_cluster": 35,
    "sora_sora": 18
  },
  "notice": {
    "cluster_ra": 10
  },
  "warning": {
    "sora_sora": 6
  }
}

// internal.jsonl のエラーログの中身を確認する
$ soralog list | soralog cat | soralog count level @domain msg | jq .error.internal
{
  "Ranch listener {swidden_http_api,3000}, ...省略...": 1
}
```

### `soralog table`

`soralog cat` の結果を受け取って markdown のテーブル形式で出力するためのコマンドです。
複数のログファイルのメッセージ群を時系列順に並べて追う場合などに利用可能です。

```console
$ soralog table --help
ログメッセージ群を標準入力から受け取り、指定されたフィールド群を列とした  Markdown のテーブル形式に変換して出力します。

結果のテーブルの各行は、一番左の列の値を使ってソートされます。 （同じ値の場合にはそれ以降の列の値を使って順々にソートされる）

Usage: soralog table [OPTIONS] [COLUMN_KEYS]...

Arguments:
  [COLUMN_KEYS]...
          テーブルに含める列名を指定する

Options:
  -m, --max-column-width <MAX_COLUMN_WIDTH>
          一つの列内の最大文字数を指定する（超過時には、それ以降は ... で置換される）

          [default: 50]

  -h, --help
          Print help (see a summary with '-h')

$ soralog list | soralog cat | jq 'select(.connection_id != null)' | soralog table timestamp @domain @type connection_id
| timestamp                   | @domain       | @type                        | connection_id              |
|-----------------------------|---------------|------------------------------|----------------------------|
| 2024-04-25T07:15:19.321882Z | signaling     | connect                      | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:19.336770Z | signaling     | offer                        | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:19.351127Z | signaling     | answer                       | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:19.453967Z | signaling     | candidate                    | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:19.458040Z | event_webhook | connection.created           | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:19.458502Z | signaling     | switched                     | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:30.629742Z | signaling     | connect                      | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:30.650128Z | signaling     | offer                        | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:30.650850Z | signaling     | re-offer                     | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:30.670219Z | signaling     | re-answer                    | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:30.680854Z | signaling     | answer                       | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:30.735593Z | signaling     | candidate                    | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:30.740491Z | event_webhook | connection.created           | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:30.741255Z | signaling     | switched                     | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:35.845295Z | signaling     | connect                      | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:35.866287Z | signaling     | offer                        | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:35.866867Z | signaling     | re-offer                     | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:35.867346Z | signaling     | re-offer                     | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:35.881508Z | signaling     | re-answer                    | CSCDTKCAXX20N5M5G594KTCRFW |
| 2024-04-25T07:15:35.888270Z | signaling     | re-answer                    | KW4PHE2R4H5TD93MEAQAXS9D4C |
| 2024-04-25T07:15:35.902236Z | signaling     | answer                       | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:35.954267Z | signaling     | candidate                    | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:35.957305Z | event_webhook | connection.created           | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:35.957934Z | signaling     | switched                     | 3SXW9CTYPD17V7TGFW8V1AZZBM |
| 2024-04-25T07:15:51.298956Z | signaling     | connect                      | MD5DW3G50H799A4VY33KRCJ1T0 |
| 2024-04-25T07:15:51.310006Z | signaling     | offer                        | MD5DW3G50H799A4VY33KRCJ1T0 |
| 2024-04-25T07:15:51.318627Z | signaling     | answer                       | MD5DW3G50H799A4VY33KRCJ1T0 |
| 2024-04-25T07:15:51.319444Z | sora_sora_rtp | RTP-MESSAGE-QUEUE-OVERFLOWED | MD5DW3G50H799A4VY33KRCJ1T0 |
| 2024-04-25T07:15:53.914145Z | signaling     | connect                      | T4XDMJH07X22Q2B43WQGS3WMY8 |
| 2024-04-25T07:15:53.930577Z | signaling     | offer                        | T4XDMJH07X22Q2B43WQGS3WMY8 |
| 2024-04-25T07:15:53.937797Z | signaling     | answer                       | T4XDMJH07X22Q2B43WQGS3WMY8 |
| 2024-04-25T07:15:53.938490Z | sora_sora_rtp | RTP-MESSAGE-QUEUE-OVERFLOWED | T4XDMJH07X22Q2B43WQGS3WMY8 |      ```
