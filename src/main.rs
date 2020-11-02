use clap::Clap;

mod cli;
mod formatter;

use cli::{CliArguments, SubCommand};
use formatter::time::format_time;

fn main() {
    // Parse commandline options.
    let args = CliArguments::parse();

    match args.cmd {
        SubCommand::Time(time) => format_time(time),
    }
}
