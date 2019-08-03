extern crate blurz;
extern crate byteorder;
extern crate hex;

mod scanner;
mod cli;
mod miflora;

// https://github.com/ChrisScheffler/miflora/wiki/The-Basics

use cli::Cli;
use miflora::Miflora;
use structopt::StructOpt;
use scanner::Scanner;
use blurz::bluetooth_session::BluetoothSession as Session;

fn main() {
    let cli = Cli::from_args();

    let scanner = Scanner::new(&cli);
    let bt_session = &Session::create_session(None).unwrap();

    let mifloras = scanner.find_mifloras(bt_session).unwrap();

    for miflora in mifloras.iter() {
        println!(" {:?}", miflora);
        miflora.connect(bt_session).unwrap().read(bt_session, cli.debug);
    }

    ()
}
