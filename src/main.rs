use clap::Parser;
use orfail::OrFail;
use soralog::command_cat::CatCommand;
use soralog::command_count::CountCommand;
use soralog::command_list::ListCommand;
use soralog::command_table::TableCommand;

#[derive(Parser)]
enum Args {
    List(ListCommand),
    Cat(CatCommand),
    Count(CountCommand),
    Table(TableCommand),
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::List(command) => command.run().or_fail()?,
        Args::Cat(command) => command.run().or_fail()?,
        Args::Count(command) => command.run().or_fail()?,
        Args::Table(command) => command.run().or_fail()?,
    }
    Ok(())
}
