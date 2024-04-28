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
    Api(ApiMessage),
    AuthWebhook(AuthWebhookMessage),
    Cluster(ClusterMessage),
}

impl Message {
    pub fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match self {
            Self::Api(m) => m.get_field_value(field_name),
            Self::AuthWebhook(m) => m.get_field_value(field_name),
            Self::Cluster(m) => m.get_field_value(field_name),
        }
    }

    // TODO
    pub fn level(&self) -> LogLevel {
        if let Some(FieldValue::Level(v)) = self.get_field_value(FieldName::Level) {
            v
        } else {
            LogLevel::Info
        }
    }
}

impl From<(PathBuf, ApiMessage)> for Message {
    fn from((path, mut message): (PathBuf, ApiMessage)) -> Self {
        message.path = Some(path);
        Self::Api(message)
    }
}

impl From<(PathBuf, AuthWebhookMessage)> for Message {
    fn from((path, mut message): (PathBuf, AuthWebhookMessage)) -> Self {
        message.path = Some(path);
        Self::AuthWebhook(message)
    }
}

impl From<(PathBuf, ClusterMessage)> for Message {
    fn from((path, mut message): (PathBuf, ClusterMessage)) -> Self {
        message.path = Some(path);
        Self::Cluster(message)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiMessage {
    pub timestamp: Timestamp,
    pub operation: String,
    pub json: serde_json::Value,

    // Extra field added by soralog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

impl ApiMessage {
    fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match field_name {
            FieldName::Kind => Some(FieldValue::Kind(MessageKind::Api)),
            FieldName::Timestamp => Some(FieldValue::String(&self.timestamp.0)),
            FieldName::Operation => Some(FieldValue::String(&self.operation)),
            FieldName::Json => Some(FieldValue::Json(&self.json)),
            FieldName::Path => self.path.as_ref().map(FieldValue::Path),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthWebhookMessage {
    pub id: String,
    pub timestamp: Timestamp,
    pub url: String,
    pub req: serde_json::Value,
    pub res: serde_json::Value,

    // Extra field added by soralog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

impl AuthWebhookMessage {
    fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match field_name {
            FieldName::Kind => Some(FieldValue::Kind(MessageKind::AuthWebhook)),
            FieldName::Timestamp => Some(FieldValue::String(&self.timestamp.0)),
            FieldName::Path => self.path.as_ref().map(FieldValue::Path),
            FieldName::Url => Some(FieldValue::String(&self.url)),
            FieldName::Req => Some(FieldValue::Json(&self.req)),
            FieldName::Res => Some(FieldValue::Json(&self.res)),
            _ => None,
        }
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

    // Extra field added by soralog.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

impl ClusterMessage {
    fn get_field_value(&self, field_name: FieldName) -> Option<FieldValue> {
        match field_name {
            FieldName::Kind => Some(FieldValue::Kind(MessageKind::Cluster)),
            FieldName::Level => Some(FieldValue::Level(self.level)),
            FieldName::Timestamp => Some(FieldValue::String(&self.timestamp.0)),
            FieldName::Msg => Some(FieldValue::String(&self.msg)),
            FieldName::MsgTag => Some(FieldValue::String(
                get_message_tag(&self.msg).unwrap_or("<untagged>"),
            )),
            FieldName::Path => self.path.as_ref().map(FieldValue::Path),
            // TODO: domain, sora_version, node, testcase
            _ => None,
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
    let tag = tag.trim();

    if tag.chars().any(|c| !matches!(c, '-' | 'A'..='Z'|'0'..='9')) {
        return None;
    }
    Some(tag)
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    clap::ValueEnum,
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
    Timestamp,
    Operation,
    Json,
    Path,
    Msg,
    #[clap(name = "msg.tag")]
    MsgTag,
    Url,
    Req,
    Res,
}

impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kind => write!(f, "kind"),
            Self::Level => write!(f, "level"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Msg => write!(f, "msg"),
            Self::MsgTag => write!(f, "msg.tag"),
            Self::Json => write!(f, "json"),
            Self::Path => write!(f, "path"),
            Self::Operation => write!(f, "operation"),
            Self::Url => write!(f, "url"),
            Self::Req => write!(f, "req"),
            Self::Res => write!(f, "res"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue<'a> {
    String(&'a str),
    Kind(MessageKind),
    Level(LogLevel),
    Json(&'a serde_json::Value),
    Path(&'a PathBuf),
}

impl<'a> FieldValue<'a> {
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            Self::String(v) => serde_json::Value::String(v.to_string()),
            Self::Kind(v) => serde_json::Value::String(v.to_string()),
            Self::Level(v) => serde_json::Value::String(v.to_string()),
            Self::Json(v) => (*v).clone(),
            Self::Path(v) => serde_json::Value::String(v.display().to_string()),
        }
    }
}

impl std::fmt::Display for FieldValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::Kind(v) => write!(f, "{v}"),
            Self::Level(v) => write!(f, "{v}"),
            Self::Json(v) => write!(f, "{v}"),
            Self::Path(v) => write!(f, "{}", v.display()),
        }
    }
}

impl<'a> PartialOrd for FieldValue<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for FieldValue<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::String(a), Self::String(b)) => a.cmp(b),
            (Self::Kind(a), Self::Kind(b)) => a.cmp(b),
            (Self::Level(a), Self::Level(b)) => a.cmp(b),
            (Self::Path(a), Self::Path(b)) => a.cmp(b),
            _ => self.to_string().cmp(&other.to_string()),
        }
    }
}
