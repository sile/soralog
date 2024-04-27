use crate::{
    jsonl,
    messages::{MessageKind, RawClusterMessage},
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
