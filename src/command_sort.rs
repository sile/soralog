use crate::{jsonl, message::Message};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct SortCommand {}

impl SortCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut messages = jsonl::input_items::<Message>()
            .collect::<orfail::Result<Vec<_>>>()
            .or_fail()?;
        messages.sort_by(|a, b| get_timestamp(a).cmp(&get_timestamp(b)));
        jsonl::output_items(messages.into_iter().map(Ok)).or_fail()?;
        Ok(())
    }
}

fn get_timestamp(m: &Message) -> Option<&str> {
    if let Some(serde_json::Value::String(t)) = m.get_value("timestamp") {
        Some(t)
    } else {
        None
    }
}
