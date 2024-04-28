use crate::{jsonl, message::Message};
use orfail::OrFail;
use std::cmp::Ordering;

#[derive(Debug, clap::Args)]
pub struct SortCommand {
    #[clap(default_value = "timestamp")]
    pub keys: Vec<String>,
}

impl SortCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut messages = jsonl::input_items::<Message>()
            .collect::<orfail::Result<Vec<_>>>()
            .or_fail()?;
        messages.sort_by(|a, b| {
            self.keys
                .iter()
                .copied()
                .map(|key| a.get_value(key).cmp(&b.get_value(key)))
                .find(|order| !order.is_eq())
                .unwrap_or(Ordering::Equal)
        });
        jsonl::output_items(messages.into_iter().map(Ok)).or_fail()?;
        Ok(())
    }
}
