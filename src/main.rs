use clap::Parser;
use orfail::OrFail;
use soralog::command_list::ListCommand;
use soralog::summary::SummaryCommand;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
enum Args {
    List(ListCommand),
    Summary {
        #[clap(long, default_value = ".")]
        root: PathBuf,
    },
    // TODO: pack / unpack
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::List(command) => {
            command.run().or_fail()?;
        }
        Args::Summary { root } => {
            let summary = SummaryCommand::new(root).run().or_fail()?;
            output_json(summary).or_fail()?;
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
