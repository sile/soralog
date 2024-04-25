use std::path::PathBuf;

#[derive(Debug)]
pub struct SummaryCommand {
    root_dir: PathBuf,
}

impl SummaryCommand {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub fn run(&self) -> orfail::Result<Summary> {
        todo!()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Summary {}
