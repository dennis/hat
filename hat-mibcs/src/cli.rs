use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Wait for data and exit when received
    #[structopt(short = "1")]
    pub until_data: bool,
    /// How many seconds should it wait for weight data. 0 is forever
    #[structopt(short = "s", long = "seconds", default_value = "60")]
    pub duration: u64,
}
