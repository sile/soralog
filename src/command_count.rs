use crate::{
    jsonl,
    messages::{ClusterMessage, MessageKind, WithMetadata},
};
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct CountCommand {}

impl CountCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for path in jsonl::input_items::<PathBuf>() {
            let path = path.or_fail()?;
            let kind = MessageKind::from_path(&path).or_fail()?;
            match kind {
                // MessageKind::Api => todo!(),
                // MessageKind::AuthWebhook => todo!(),
                // MessageKind::AuthWebhookError => todo!(),
                MessageKind::Cluster => cat_jsonl::<ClusterMessage>(kind, &path).or_fail()?,
                // MessageKind::Connection => todo!(),
                // MessageKind::Crash => todo!(),
                // MessageKind::Debug => todo!(),
                // MessageKind::EventWebhook => todo!(),
                // MessageKind::EventWebhookError => todo!(),
                // MessageKind::Internal => todo!(),
                // MessageKind::SessionWebhook => todo!(),
                // MessageKind::SessionWebhookError => todo!(),
                // MessageKind::Signaling => todo!(),
                // MessageKind::Sora => todo!(),
                // MessageKind::StatsWebhook => todo!(),
                // MessageKind::StatsWebhookError => todo!(),
                _ => eprintln!("[WARNING] Not implemented: {} ({:?})", path.display(), kind),
            }
        }
        Ok(())
    }
}

fn cat_jsonl<T>(kind: MessageKind, path: &PathBuf) -> orfail::Result<()>
where
    T: serde::Serialize + for<'a> serde::Deserialize<'a>,
{
    let file = std::fs::File::open(path).or_fail()?;
    let reader = std::io::BufReader::new(file);
    let messages = serde_json::Deserializer::from_reader(reader)
        .into_iter::<T>()
        .map(|result| {
            result.or_fail().map(|message| WithMetadata {
                kind,
                path: path.clone(),
                message,
            })
        });
    jsonl::output_items(messages).or_fail()?;
    Ok(())
}
