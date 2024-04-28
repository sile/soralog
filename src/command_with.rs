use crate::{
    jsonl,
    messages::{FieldName, Message},
};
use orfail::OrFail;
use std::collections::HashSet;

#[derive(Debug, clap::Args)]
pub struct WithCommand {
    pub fields: Vec<FieldName>,
}

impl WithCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let fields = self
            .fields
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        let messages = jsonl::input_items::<Message>().map(|m| {
            m.and_then(|m| {
                let mut value = serde_json::to_value(m).or_fail()?;
                let map = value.as_object_mut().or_fail()?;
                map.retain(|k, _| fields.contains(k));
                Ok(value)
            })
        });
        jsonl::output_items(messages).or_fail()?;
        Ok(())
    }
}
