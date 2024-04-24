use clap::Parser;
use orfail::OrFail;
use soralog::LogFilePathIterator;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
enum Args {
    List {
        #[clap(long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::List { root } => {
            let files = LogFilePathIterator::new(&root)
                .map(|item| item.map(|(_, path)| path))
                .collect::<orfail::Result<Vec<_>>>()
                .or_fail()?;
            output_json(files).or_fail()?;
        }
    }
    Ok(())
}

fn output_json<T: serde::Serialize>(value: T) -> orfail::Result<()> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    serde_json::to_writer_pretty(&mut stdout, &value).or_fail()?;
    writeln!(stdout).or_fail()?;
    Ok(())
}
