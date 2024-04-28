use crate::jsonl;
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct PpCommand {}

impl PpCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let messages = jsonl::input_items::<serde_json::Value>();
        jsonl::output_items_pp(messages).or_fail()?;
        Ok(())
    }
}
