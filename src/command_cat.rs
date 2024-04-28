use crate::{
    jsonl,
    message::{Message, MessageKind},
};
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct CatCommand {
    #[clap(long)]
    pub disable_timestamp_sort: bool,
}

impl CatCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut all_messages = Vec::new();

        for path in jsonl::input_items::<PathBuf>() {
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
                    let messages = jsonl_messages(kind, &path).or_fail()?;
                    if self.disable_timestamp_sort {
                        jsonl::output_items(messages).or_fail()?;
                    } else {
                        for message in messages {
                            all_messages.push(message.or_fail()?);
                        }
                    }
                }
                MessageKind::Crash => {
                    let messages = crash_log_messages(&path).or_fail()?;
                    if self.disable_timestamp_sort {
                        jsonl::output_items(messages).or_fail()?;
                    } else {
                        for message in messages {
                            all_messages.push(message.or_fail()?);
                        }
                    }
                }
            }
        }

        if !self.disable_timestamp_sort {
            all_messages.sort_by(|a, b| get_timestamp(a).cmp(&get_timestamp(b)));
            jsonl::output_items(all_messages.into_iter().map(Ok)).or_fail()?;
        }

        Ok(())
    }
}

fn jsonl_messages(
    kind: MessageKind,
    path: &PathBuf,
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

fn get_timestamp(m: &Message) -> Option<&str> {
    if let Some(serde_json::Value::String(t)) = m.get_value("timestamp") {
        Some(t)
    } else {
        None
    }
}
