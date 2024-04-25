#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClusterLogMessage {
    pub id: String,
    pub level: LogLevel,
    pub msg: String,
    pub domain: Vec<String>,
    pub sora_version: String,
    pub node: String,
    pub timestamp: String, // TODO: Use Timestamp type
    pub testcase: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Emergency,
}
