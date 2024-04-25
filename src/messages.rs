use orfail::OrFail;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::LogFileKind;

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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ClusterLogMessage {
    pub id: String,
    pub level: LogLevel,
    pub msg: String,
    pub domain: Vec<String>,
    pub sora_version: String,
    pub node: String,
    pub timestamp: Timestamp,
    pub testcase: Option<String>,
}

impl Message for ClusterLogMessage {
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub struct Timestamp(String);
