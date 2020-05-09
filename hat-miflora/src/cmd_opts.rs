use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CmdOpts {
    /// Output JSON
    #[structopt(short, long)]
    pub json: bool,

    /// Don't show headers (ignored for JSON output)
    #[structopt(short = "H", long)]
    pub no_headers: bool,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Scan for Miflora Devices
    Scan {
        /// How long should we listen for Mifloras
        #[structopt(short, long = "duration", default_value = "10")]
        duration_sec: u8,
    },

    /// Read realtime data from Miflora device
    Read { addr: String },

    /// Make Miflora device blink
    Blink { addr: String },

    /// Read historical data from Miflora device
    History {
        addr: String,

        /// Read from record number (defaults to first record)
        #[structopt(short, long)]
        from: Option<u16>,

        /// Read until record number (defaults to last record)
        #[structopt(short, long)]
        to: Option<u16>,

        /// Number of records to batch together, before reconnecting
        #[structopt(short, long)]
        page: Option<u16>,

        /// Clear after successful reading (only if you read everything)
        #[structopt(short, long)]
        clear: bool,
    },

    /// Read number of historical records from Miflora Device
    HistoryCount { addr: String },

    /// Clear historical data from Miflora device
    HistoryClear { addr: String },
}
