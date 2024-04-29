use crate::{
    json_stream,
    message::{Message, MessageKind},
};
use orfail::OrFail;
use std::path::PathBuf;

/// `soralog list` コマンドの出力結果を標準入力から受け取り、ログファイルの中身を JSONL 形式で標準出力に出力します
///
/// 通常の `cat` コマンドとは異なり、以下の特別な処理を行います。
///
/// ### 1. 各メッセージには @domain, @type, @path という特別なフィールドが追加される
///
/// @domain フィールドには `{{ ログファイルの種類 }}_{{ sora.jsonl などの domain の値を _ で連結したもの }}` が
/// 値として格納されます。
///
/// @type フィールドには、各ログファイル毎に異なるメッセージの種別を表す項目を統一的に扱うためのフィールドの
/// 値が格納されます。
/// 例えばイベントウェブフックログなら type フィールドが、API ログなら operation フィールドがこれに該当します。
///
/// @path フィールドには、ログファイルのパスが格納されます。
///
///
/// ### 2. crash.log の中身はパースされ、JSONL 形式に変換される
///
/// @domain, @path, @raw_report を持つメッセージが出力されます
#[derive(Debug, clap::Args)]
pub struct CatCommand {}

impl CatCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for path in json_stream::input_items::<PathBuf>() {
            let path = path.or_fail()?;
            let kind = MessageKind::from_path(&path).or_fail()?;
            match kind {
                MessageKind::Api
                | MessageKind::AuthWebhook
                | MessageKind::Cluster
                | MessageKind::Connection
                | MessageKind::Debug
                | MessageKind::EventWebhook
                | MessageKind::EventWebhookError
                | MessageKind::Internal
                | MessageKind::SessionWebhook
                | MessageKind::SessionWebhookError
                | MessageKind::Signaling
                | MessageKind::Sora
                | MessageKind::StatsWebhook
                | MessageKind::StatsWebhookError => {
                    let messages = jsonl_messages(kind, path).or_fail()?;
                    json_stream::output_items(messages).or_fail()?;
                }
                MessageKind::Crash => {
                    let messages = crash_log_messages(&path).or_fail()?;
                    json_stream::output_items(messages).or_fail()?;
                }
            }
        }

        Ok(())
    }
}

fn jsonl_messages(
    kind: MessageKind,
    path: PathBuf,
) -> orfail::Result<impl Iterator<Item = orfail::Result<Message>>> {
    let path = path.clone();
    let file = std::fs::File::open(&path).or_fail()?;
    let reader = std::io::BufReader::new(file);
    let messages = serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .map(move |result| {
            result
                .or_fail_with(|e| format!("Failed to read JSON from {:?}: {}", path.display(), e))
                .map(|message| Message::from_jsonl_message(kind, path.clone(), message))
        });
    Ok(messages)
}

fn crash_log_messages(
    path: &PathBuf,
) -> orfail::Result<impl Iterator<Item = orfail::Result<Message>>> {
    let text = std::fs::read_to_string(path).or_fail()?;
    let messages = Message::vec_from_crash_log(path.clone(), &text)
        .or_fail()?
        .into_iter()
        .map(Ok);
    Ok(messages)
}
