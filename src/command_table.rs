use crate::{jsonl, messages::Message};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct TableCommand {}

impl TableCommand {
    pub fn run(&self) -> orfail::Result<()> {
        // let messages = jsonl::input_items::<Message>().map(|m| {
        //     m.and_then(|m| {
        //         let mut value = serde_json::to_value(m).or_fail()?;
        //         let map = value.as_object_mut().or_fail()?;
        //         map.retain(|k, _| fields.contains(k));
        //         Ok(value)
        //     })
        // });
        // jsonl::output_items(messages).or_fail()?;
        Ok(())
    }
}
