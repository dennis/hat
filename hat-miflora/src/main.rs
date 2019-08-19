extern crate blurz;
extern crate byteorder;
extern crate hex;
extern crate dbus_common;

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

fn inquery_miflora(miflora : &ConnectedMiflora, cli : &Cli) {
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

    if cli.realtime {
        match miflora.realtime(cli.debug) {
            Ok(_) => (),
            Err(err) => eprintln!("{:?}", err)
        }
    }

    if cli.blink {
        match miflora.blink(cli.debug) {
            Ok(_) => (),
            Err(err) => eprintln!("{:?}", err)
        }
    }
}

fn main() {
    let mut cli = Cli::from_args();

    if !cli.blink && !cli.realtime {
        cli.realtime = true;
    }

    let scanner = Scanner::new(&cli).unwrap();
    let bt_session = &Session::create_session(None).unwrap();

    let mifloras = scanner.find_mifloras().unwrap();

    for miflora in mifloras.iter() {
        if cli.debug { eprintln!("checking: {:?}", miflora); }

        miflora
            .connect(bt_session)
            .map(|miflora| inquery_miflora(&miflora, &cli));
    }

    ()
}
