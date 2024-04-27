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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Message {
    Cluster(ClusterMessage),
}

impl Message {
    pub fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match self {
            Message::Cluster(m) => m.get_field_value(field_name),
        }
    }
}

impl From<(PathBuf, ClusterMessage)> for Message {
    fn from((path, mut message): (PathBuf, ClusterMessage)) -> Self {
        message.path = Some(path);
        Self::Cluster(message)
    }
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

    // TODO: doc
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

impl ClusterMessage {
    fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match field_name {
            FieldName::Kind => Some(FieldValue::Kind(MessageKind::Cluster)),
            FieldName::Level => Some(FieldValue::Level(self.level)),
            FieldName::Msg => Some(FieldValue::String(&self.msg)),
            FieldName::MsgTag => Some(FieldValue::String(
                get_message_tag(&self.msg).unwrap_or("<untagged>"),
            )),
        }
    }
}

fn get_message_tag(msg: &str) -> Option<&str> {
    if !msg.contains('|') {
        return None;
    }

    let Some(tag) = msg.splitn(2, '|').next() else {
        unreachable!();
    };
    Some(tag.trim())
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

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Notice => write!(f, "notice"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Emergency => write!(f, "emergency"),
        }
    }
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

impl std::fmt::Display for MessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::AuthWebhook => write!(f, "auth_webhook"),
            Self::AuthWebhookError => write!(f, "auth_webhook_error"),
            Self::Cluster => write!(f, "cluster"),
            Self::Connection => write!(f, "connection"),
            Self::Crash => write!(f, "crash"),
            Self::Debug => write!(f, "debug"),
            Self::EventWebhook => write!(f, "event_webhook"),
            Self::EventWebhookError => write!(f, "event_webhook_error"),
            Self::Internal => write!(f, "internal"),
            Self::SessionWebhook => write!(f, "session_webhook"),
            Self::SessionWebhookError => write!(f, "session_webhook_error"),
            Self::Signaling => write!(f, "signaling"),
            Self::Sora => write!(f, "sora"),
            Self::StatsWebhook => write!(f, "stats_webhook"),
            Self::StatsWebhookError => write!(f, "stats_webhook_error"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum FieldName {
    Kind,
    Level,
    Msg,
    #[clap(name = "msg.tag")]
    MsgTag,
}

#[derive(Debug, Clone)]
pub enum FieldValue<'a> {
    String(&'a str),
    Kind(MessageKind),
    Level(LogLevel),
}

impl std::fmt::Display for FieldValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::Kind(v) => write!(f, "{v}"),
            Self::Level(v) => write!(f, "{v}"),
        }
    }
}
