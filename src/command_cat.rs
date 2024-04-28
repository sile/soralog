use crate::{
    jsonl,
    message::{Message, MessageKind},
};
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct CatCommand {}

impl CatCommand {
    pub fn run(&self) -> orfail::Result<()> {
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
                | MessageKind::StatsWebhookError => cat_jsonl(kind, &path).or_fail()?,
                MessageKind::Crash => cat_crash_log(&path).or_fail()?,
            }
        }
        Ok(())
    }
}

fn cat_jsonl(kind: MessageKind, path: &PathBuf) -> orfail::Result<()> {
    let file = std::fs::File::open(path).or_fail()?;
    let reader = std::io::BufReader::new(file);
    let messages = serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .map(|result| {
            result
                .or_fail_with(|e| format!("Failed to read JSON from {:?}: {}", path.display(), e))
                .map(|message| Message::from_jsonl_message(kind, path.clone(), message))
        });
    jsonl::output_items(messages).or_fail()?;
    Ok(())
}

fn cat_crash_log(path: &PathBuf) -> orfail::Result<()> {
    let text = std::fs::read_to_string(path).or_fail()?;
    let messages = Message::vec_from_crash_log(path.clone(), &text)
        .or_fail()?
        .into_iter()
        .map(Ok);
    jsonl::output_items(messages).or_fail()?;
    Ok(())
}
