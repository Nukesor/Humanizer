use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Humanizer",
    about = "Humanize your stuff",
    author = "Arne Beer <contact@arne.beer>"
)]
pub struct Opt {
    /// Convert nano seconds to human readable time
    #[structopt(short, long)]
    pub nanos: Option<i64>,

    /// Convert nano seconds to human readable time
    #[structopt(short, long)]
    pub seconds: Option<i64>,
}
