use crate::jsonl;
use orfail::OrFail;
use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct CatCommand {
    #[clap(long, default_value = ".")]
    pub root: PathBuf,
}

impl CatCommand {
    pub fn run(&self) -> orfail::Result<()> {
        for path in jsonl::input_items::<PathBuf>() {
            let path = path.or_fail()?;
        }
        Ok(())
    }
}
