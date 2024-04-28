use clap::Parser;
use orfail::OrFail;
use soralog::command_cat::CatCommand;
use soralog::command_count::CountCommand;
use soralog::command_filter::FilterCommand;
use soralog::command_list::ListCommand;
use soralog::command_sort::SortCommand;
use soralog::command_with::WithCommand;

#[derive(Parser)]
enum Args {
    List(ListCommand),
    Cat(CatCommand),
    Count(CountCommand),
    Sort(SortCommand),
    Filter(FilterCommand),
    With(WithCommand),
    // TODO: table
}

fn main() -> orfail::Result<()> {
    let args = Args::parse();
    match args {
        Args::List(command) => command.run().or_fail()?,
        Args::Cat(command) => command.run().or_fail()?,
        Args::Count(command) => command.run().or_fail()?,
        Args::Sort(command) => command.run().or_fail()?,
        Args::Filter(command) => command.run().or_fail()?,
        Args::With(command) => command.run().or_fail()?,
    }
    Ok(())
}
