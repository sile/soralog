use orfail::OrFail;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct LogFilePathIterator {
    stack: Vec<PathBuf>,
    visited: HashSet<PathBuf>,
}

impl LogFilePathIterator {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            stack: vec![root_dir.as_ref().to_path_buf()],
            visited: HashSet::new(),
        }
    }

    fn next_item(&mut self) -> orfail::Result<Option<(LogFileKind, PathBuf)>> {
        while let Some(path) = self.stack.pop() {
            if self.visited.contains(&path) {
                continue;
            }
            self.visited.insert(path.clone());

            if path.is_dir() {
                for entry in std::fs::read_dir(path).or_fail()? {
                    let child_path = entry.or_fail()?.path();
                    if let Ok(child_path) = child_path.canonicalize() {
                        self.stack.push(child_path);
                    }
                }
                continue;
            }

            let Some(kind) = LogFileKind::from_path(&path) else {
                continue;
            };

            return Ok(Some((kind, path)));
        }
        Ok(None)
    }
}

impl Iterator for LogFilePathIterator {
    type Item = orfail::Result<(LogFileKind, PathBuf)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().or_fail().transpose()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFileKind {
    Api,
    AuthWebhook,
    AuthWebhookError,
    Cluster,
    Connection,
    Crash,
    Debug,
    EventWebhook,
    EventWebhookError,
    Internal,
    SessionWebhook,
    SessionWebhookError,
    Signaling,
    Sora,
    StatsWebhook,
    StatsWebhookError,
}

impl LogFileKind {
    fn from_path(path: &PathBuf) -> Option<Self> {
        let Some(name) = path.file_name() else {
            return None;
        };
        let Some(name) = name.to_str() else {
            return None;
        };

        match name {
            "api.jsonl" => Some(Self::Api),
            "auth_webhook.jsonl" => Some(Self::AuthWebhook),
            "auth_webhook_error.jsonl" => Some(Self::AuthWebhookError),
            "cluster.jsonl" => Some(Self::Cluster),
            "connection.jsonl" => Some(Self::Connection),
            "crash.log" => Some(Self::Crash),
            "debug.jsonl" => Some(Self::Debug),
            "event_webhook.jsonl" => Some(Self::EventWebhook),
            "event_webhook_error.jsonl" => Some(Self::EventWebhookError),
            "internal.jsonl" => Some(Self::Internal),
            "session_webhook.jsonl" => Some(Self::SessionWebhook),
            "session_webhook_error.jsonl" => Some(Self::SessionWebhookError),
            "signaling.jsonl" => Some(Self::Signaling),
            "sora.jsonl" => Some(Self::Sora),
            "stats_webhook.jsonl" => Some(Self::StatsWebhook),
            "stats_webhook_error.jsonl" => Some(Self::StatsWebhookError),
            _ => None,
        }
    }
}
