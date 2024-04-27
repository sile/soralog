use clap::Parser;
use orfail::OrFail;
use soralog::command_cat::CatCommand;
use soralog::command_count::CountCommand;
use soralog::command_list::ListCommand;
use soralog::summary::SummaryCommand;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
enum Args {
    List(ListCommand),
    Cat(CatCommand),
    Count(CountCommand),
    // TODO: pack / unpack
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::List(command) => {
            command.run().or_fail()?;
        }
        Args::Cat(command) => {
            command.run().or_fail()?;
        }
        Args::Count(command) => {
            command.run().or_fail()?;
        }
    }
    Ok(())
}
