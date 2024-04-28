use crate::{json_stream, message::MessageKind};
use orfail::OrFail;
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug, clap::Args)]
pub struct ListCommand {
    #[clap(long, short, default_value = ".")]
    pub root: PathBuf,

    #[clap(long, short)]
    pub absolute: bool,
}

impl ListCommand {
    pub fn run(&self) -> orfail::Result<()> {
        let paths = LogFilePathIterator::new(&self.root);
        if self.absolute {
            json_stream::output_items(paths).or_fail()?;
        } else {
            let root = self.root.canonicalize().or_fail()?;
            json_stream::output_items(paths.map(|item| {
                item.and_then(|path| {
                    path.strip_prefix(&root)
                        .map(|path| path.to_path_buf())
                        .or_fail()
                })
            }))
            .or_fail()?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LogFilePathIterator {
    stack: Vec<PathBuf>,
    visited: HashSet<PathBuf>,
}

impl LogFilePathIterator {
    fn new(root_dir: &PathBuf) -> Self {
        Self {
            stack: vec![root_dir.clone()],
            visited: HashSet::new(),
        }
    }

    fn next_item(&mut self) -> orfail::Result<Option<PathBuf>> {
        while let Some(path) = self.stack.pop() {
            if self.visited.contains(&path) {
                continue;
            }
            self.visited.insert(path.clone());

            if path.is_dir() {
                for entry in std::fs::read_dir(path).or_fail()? {
                    let child_path = entry.or_fail()?.path();
                    if let Ok(child_path) = child_path.canonicalize() {
                        self.stack.push(child_path);
                    }
                }
                continue;
            }

            if MessageKind::from_path(&path).is_none() {
                continue;
            };

            return Ok(Some(path));
        }
        Ok(None)
    }
}

impl Iterator for LogFilePathIterator {
    type Item = orfail::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().or_fail().transpose()
    }
}
