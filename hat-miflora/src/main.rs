extern crate blurz;
extern crate byteorder;
extern crate hex;

mod scanner;
mod cli;

// https://github.com/ChrisScheffler/miflora/wiki/The-Basics

use cli::Cli;
use structopt::StructOpt;
use scanner::Scanner;

fn main() {
    let cli = Cli::from_args();

    let scanner = Scanner::new(&cli);

    match scanner.scan() {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
    }
}
