use crate::{
    jsonl,
    messages::{FieldName, Message},
};
use orfail::OrFail;
use std::cmp::Ordering;

#[derive(Debug, clap::Args)]
pub struct SortCommand {
    #[clap(default_value = "timestamp")]
    pub sort_keys: Vec<FieldName>,
}

impl SortCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let mut messages = jsonl::input_items::<Message>()
            .collect::<orfail::Result<Vec<_>>>()
            .or_fail()?;
        messages.sort_by(|a, b| {
            self.sort_keys
                .iter()
                .copied()
                .map(|key| a.get_field_value(key).cmp(&b.get_field_value(key)))
                .find(|order| !order.is_eq())
                .unwrap_or(Ordering::Equal)
        });
        jsonl::output_items(messages.into_iter().map(Ok)).or_fail()?;
        Ok(())
    }
}
