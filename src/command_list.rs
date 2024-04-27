use crate::{jsonl, log_file::LogFilePathIterator};
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct ListCommand {
    #[clap(long, default_value = ".")]
    pub root: PathBuf,
}

impl ListCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let paths = LogFilePathIterator::new(&self.root).map(|item| item.map(|(_, path)| path));
        jsonl::output_items(paths).or_fail()?;
        Ok(())
    }
}
