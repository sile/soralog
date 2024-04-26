use std::path::PathBuf;

#[derive(Debug, clap::Args)]
pub struct ListCommand {
    #[clap(long, default_value = ".")]
    pub root: PathBuf,
}

impl ListCommand {
    pub fn run(&self) -> orfail::Result<()> {
        Ok(())
    }
}
