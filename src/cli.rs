use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "Humanizer",
    about = "Humanize your stuff",
    author = "Arne Beer <contact@arne.beer>"
)]
pub struct CliArguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Format something time related
    Time(Time),
}

#[derive(Clap, Debug)]
pub struct Time {
    /// Convert nano seconds to human readable time
    #[clap(short, long)]
    pub nanos: Option<i64>,

    /// Convert nano seconds to human readable time
    #[clap(short, long)]
    pub seconds: Option<i64>,
}
