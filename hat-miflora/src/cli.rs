use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short = "d", long = "debug")]
    pub debug: bool
}

