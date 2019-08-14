extern crate blurz;
extern crate byteorder;
extern crate hex;

mod scanner;
mod cli;
mod miflora;

// https://github.com/ChrisScheffler/miflora/wiki/The-Basics

use cli::Cli;
use miflora::Miflora;
use miflora::ConnectedMiflora;
use structopt::StructOpt;
use scanner::Scanner;
use blurz::bluetooth_session::BluetoothSession as Session;

fn inquery_miflora(bt_session : &Session, miflora : &ConnectedMiflora, cli : &Cli) {
    // check if we want readings from a specific address
    if let Some(addr) = &cli.address {
        if let Ok(miflora_address) = miflora.get_address() {
            if addr != &miflora_address {
                if cli.debug { eprintln!("  address {:?} doesn't match, skipping", miflora_address); }
                return;
            }
        }
    }

    // check if we want readings from a specific name (can be multiple!)
    if let Some(name) = &cli.name {
        if let Ok(miflora_name) = miflora.get_name() {
            if name != &miflora_name {
                if cli.debug { eprintln!("  name {:?} doesn't match, skipping", miflora_name); }
                return;
            }
        }
    }

    match miflora.read(bt_session, cli.debug) {
        Ok(_) => (),
        Err(err) => eprintln!("{:?}", err)
    }
}

fn main() {
    let cli = Cli::from_args();

    let scanner = Scanner::new(&cli);
    let bt_session = &Session::create_session(None).unwrap();

    let mifloras = scanner.find_mifloras(bt_session).unwrap();

    for miflora in mifloras.iter() {
        if cli.debug { eprintln!("checking: {:?}", miflora); }

        miflora
            .connect(bt_session)
            .map(|miflora| inquery_miflora(bt_session, &miflora, &cli));
    }

    ()
}
