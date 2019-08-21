use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// Enable debug (pretty verbose)
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,
    /// Filter by bluetooth hardware address
    #[structopt(short = "a", long = "addr")]
    pub address: Option<String>,
    /// Filter by bluetooth name
    #[structopt(short = "n", long = "name")]
    pub name: Option<String>,
    /// Make miflora blink
    #[structopt(short = "b", long = "blink")]
    pub blink: bool,
    /// read realtime data
    #[structopt(short = "r", long = "realtime")]
    pub realtime: bool,
}
