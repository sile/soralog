use crate::{
    jsonl,
    message::{
        ApiMessage, AuthWebhookMessage, ClusterMessage, ConnectionMessage, CrashMessage,
        JsonlMessage, Message, MessageKind,
    },
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
                MessageKind::Api => cat_jsonl::<ApiMessage>(&path).or_fail()?,
                MessageKind::AuthWebhook => cat_jsonl::<AuthWebhookMessage>(&path).or_fail()?,
                MessageKind::Cluster => cat_jsonl::<ClusterMessage>(&path).or_fail()?,
                MessageKind::Connection => cat_jsonl::<ConnectionMessage>(&path).or_fail()?,
                MessageKind::Crash => cat_crash_log(&path).or_fail()?,
                MessageKind::Debug
                | MessageKind::EventWebhook
                | MessageKind::EventWebhookError
                | MessageKind::Internal
                | MessageKind::SessionWebhook
                | MessageKind::SessionWebhookError
                | MessageKind::Signaling
                | MessageKind::Sora
                | MessageKind::StatsWebhook
                | MessageKind::StatsWebhookError => cat_jsonl2(kind, &path).or_fail()?,
                _ => eprintln!("[WARNING] Not implemented: {} ({:?})", path.display(), kind),
            }
        }
        Ok(())
    }
}

fn cat_jsonl<T>(path: &PathBuf) -> orfail::Result<()>
where
    T: for<'a> serde::Deserialize<'a>,
    Message: From<(PathBuf, T)>,
{
    let file = std::fs::File::open(path).or_fail()?;
    let reader = std::io::BufReader::new(file);
    let messages = serde_json::Deserializer::from_reader(reader)
        .into_iter()
        .map(|result| {
            result
                .or_fail_with(|e| format!("Failed to read JSON from {:?}: {}", path.display(), e))
                .map(|message| Message::from((path.clone(), message)))
        });
    jsonl::output_items(messages).or_fail()?;
    Ok(())
}

// TODO
fn cat_jsonl2(kind: MessageKind, path: &PathBuf) -> orfail::Result<()> {
    let file = std::fs::File::open(path).or_fail()?;
    let reader = std::io::BufReader::new(file);
    let messages = serde_json::Deserializer::from_reader(reader)
        .into_iter::<JsonlMessage>()
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
    let messages = CrashMessage::parse(path.clone(), &text)
        .or_fail()?
        .into_iter()
        .map(Message::Crash)
        .map(Ok);
    jsonl::output_items(messages).or_fail()?;
    Ok(())
}
