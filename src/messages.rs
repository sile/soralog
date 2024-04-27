use crate::log_file::LogFileKind;
use orfail::OrFail;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn load_jsonl<M>(path: &PathBuf) -> orfail::Result<Vec<M>>
where
    M: for<'a> serde::Deserialize<'a>,
{
    let mut messages = Vec::new();
    let reader = BufReader::new(std::fs::File::open(path).or_fail()?);
    for line in reader.lines() {
        let line = line.or_fail()?;
        let message = serde_json::from_str(&line).or_fail()?;
        messages.push(message);
    }
    Ok(messages)
}

pub trait Message {
    fn kind(&self) -> LogFileKind;
    fn level(&self) -> LogLevel;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WithMetadata<T> {
    pub kind: MessageKind,
    pub path: PathBuf,
    #[serde(flatten)]
    pub message: T,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterMessage {
    pub id: String,
    pub level: LogLevel,
    pub msg: String,
    pub domain: Vec<String>,
    pub sora_version: String,
    pub node: String,
    pub timestamp: Timestamp,
    pub testcase: Option<String>,
}

impl Message for ClusterMessage {
    fn kind(&self) -> LogFileKind {
        LogFileKind::Cluster
    }

    fn level(&self) -> LogLevel {
        self.level
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Emergency,
}

// TODO: use chrono or something
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Timestamp(String);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum MessageKind {
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

impl MessageKind {
    pub fn from_path(path: &PathBuf) -> Option<Self> {
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
