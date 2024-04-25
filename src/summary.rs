use crate::{
    messages::{load_jsonl, ClusterLogMessage, LogLevel, Message},
    LogFileKind, LogFilePathIterator,
};
use orfail::OrFail;
use std::{collections::BTreeMap, path::PathBuf};

// TODO: stats
#[derive(Debug)]
pub struct SummaryCommand {
    root_dir: PathBuf,
    summary: Summary,
}

impl SummaryCommand {
    pub fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
            summary: Summary::default(),
        }
    }

    pub fn run(&mut self) -> orfail::Result<Summary> {
        for entry in LogFilePathIterator::new(&self.root_dir) {
            let (kind, path) = entry.or_fail()?;
            if path.metadata().or_fail()?.len() == 0 {
                continue;
            }

            self.summary.handle_file(kind);

            match kind {
                // LogFileKind::Sora => todo!(),
                LogFileKind::Cluster => self.handle_cluster_log(&path).or_fail()?,
                // LogFileKind::Debug => todo!(),
                // LogFileKind::Internal => todo!(),
                // LogFileKind::Api => todo!(),
                // LogFileKind::Crash => todo!(),
                // LogFileKind::Signaling => todo!(),
                // LogFileKind::Connection => todo!(),
                // LogFileKind::EventWebhook => todo!(),
                // LogFileKind::SessionWebhook => todo!(),
                _ => {}
            }
        }

        Ok(self.summary.clone())
    }

    fn handle_cluster_log(&mut self, path: &PathBuf) -> orfail::Result<()> {
        let messages = load_jsonl::<ClusterLogMessage>(path).or_fail()?;
        for message in messages.iter() {
            self.summary.handle_message(message);
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Summary {
    pub files: usize,
    pub files_per_kind: BTreeMap<LogFileKind, usize>,
    pub messages: usize,
    pub messages_per_level: BTreeMap<LogLevel, usize>,
    pub messages_per_kind: BTreeMap<LogFileKind, usize>,
    // TODO: messages_per_domain
}

impl Summary {
    fn handle_file(&mut self, kind: LogFileKind) {
        self.files += 1;
        *self.files_per_kind.entry(kind).or_default() += 1;
    }

    fn handle_message(&mut self, message: &impl Message) {
        self.messages += 1;
        *self.messages_per_level.entry(message.level()).or_default() += 1;
        *self.messages_per_kind.entry(message.kind()).or_default() += 1;
    }
}
