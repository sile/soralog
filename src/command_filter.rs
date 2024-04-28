use crate::{
    jsonl,
    message::{LogLevel, Message2},
};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct FilterCommand {
    #[clap(long, short)]
    pub level: Option<LogLevel>,
}

impl FilterCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let messages = jsonl::input_items::<Message2>()
            .filter(|m| m.as_ref().map_or(true, |m| self.filter(m)));
        jsonl::output_items(messages).or_fail()?;
        Ok(())
    }

    fn filter(&self, message: &Message2) -> bool {
        if let Some(level) = self.level {
            if message.level() < level {
                return false;
            }
        }
        true
    }
}
