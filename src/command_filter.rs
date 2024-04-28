use crate::{
    jsonl,
    messages::{LogLevel, Message},
};
use orfail::OrFail;

#[derive(Debug, clap::Args)]
pub struct FilterCommand {
    #[clap(long, short)]
    pub level: Option<LogLevel>,
}

impl FilterCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let messages =
            jsonl::input_items::<Message>().filter(|m| m.as_ref().map_or(true, |m| self.filter(m)));
        jsonl::output_items(messages).or_fail()?;
        Ok(())
    }

    fn filter(&self, message: &Message) -> bool {
        if let Some(level) = self.level {
            if message.level() < level {
                return false;
            }
        }
        true
    }
}
