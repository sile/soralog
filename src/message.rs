use orfail::OrFail;
use std::path::PathBuf;

pub type JsonMap = serde_json::Map<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message(JsonMap);

impl Message {
    pub fn from_jsonl_message(kind: MessageKind, path: PathBuf, mut message: JsonMap) -> Self {
        let mut domain = kind.to_string();
        if let Some(serde_json::Value::Array(domain_array)) = message.get("domain") {
            for subdomain in domain_array.iter().filter_map(|v| v.as_str()) {
                domain.push('.');
                domain.push_str(subdomain);
            }
        }
        message.insert(
            "@domain".to_string(),
            serde_json::Value::String(domain.to_string()),
        );

        message.insert(
            "@path".to_string(),
            serde_json::Value::String(path.display().to_string()),
        );

        if let Some(serde_json::Value::String(ty)) = message
            .get("type")
            .or_else(|| message.get("operation"))
            .or_else(|| {
                if let Some(serde_json::Value::Object(m)) = message.get("req") {
                    m.get("type")
                } else {
                    None
                }
            })
        {
            message.insert(
                "@type".to_string(),
                serde_json::Value::String(ty.to_owned()),
            );
        } else if let Some(serde_json::Value::String(msg)) = message.get("msg") {
            if let Some(tag) = get_message_tag(msg) {
                message.insert(
                    "@type".to_string(),
                    serde_json::Value::String(tag.to_owned()),
                );
            }
        }

        Self(message)
    }

    pub fn vec_from_crash_log(path: PathBuf, mut text: &str) -> orfail::Result<Vec<Self>> {
        if text.is_empty() {
            return Ok(vec![]);
        }

        const MARKER: &str = "=CRASH REPORT ";
        text.starts_with(MARKER).or_fail()?;

        fn message(path: &PathBuf, report: &str) -> Message {
            let mut message = JsonMap::new();
            message.insert(
                "@domain".to_string(),
                serde_json::Value::String(MessageKind::Crash.to_string()),
            );
            message.insert(
                "@path".to_string(),
                serde_json::Value::String(path.display().to_string()),
            );
            message.insert(
                "@raw_report".to_string(),
                serde_json::Value::String(report.trim().to_string()),
            );
            Self(message)
        }

        let mut messages = vec![];
        while let Some(end) = text[MARKER.len()..].find(MARKER) {
            let end = end + MARKER.len();
            messages.push(message(&path, &text[..end]));
            text = &text[end..];
        }
        messages.push(message(&path, text));

        Ok(messages)
    }

    pub fn get_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.0.get(key)
    }

    pub fn get_value_string(&self, key: &str) -> Option<String> {
        let Some(v) = self.0.get(key) else {
            return None;
        };
        match v {
            serde_json::Value::Null => Some("null".to_string()),
            serde_json::Value::Bool(v) => Some(v.to_string()),
            serde_json::Value::Number(v) => Some(v.to_string()),
            serde_json::Value::String(v) => Some(v.to_owned()),
            serde_json::Value::Array(_) => Some("__ARRAY__".to_string()),
            serde_json::Value::Object(_) => Some("__OBJECT__".to_string()),
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
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum MessageKind {
    Api,
    AuthWebhook,
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
