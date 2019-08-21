extern crate byteorder;
extern crate dbus_common;
extern crate hex;

mod cli;
mod miflora;
mod scanner;

// https://github.com/ChrisScheffler/miflora/wiki/The-Basics

use std::error::Error;

use cli::Cli;
use miflora::ConnectedMiflora;
use miflora::Miflora;
use scanner::Scanner;
use structopt::StructOpt;

fn inquery_miflora(miflora: &ConnectedMiflora, cli: &Cli) {
    // check if we want readings from a specific address
    if let Some(addr) = &cli.address {
        if let Ok(miflora_address) = miflora.get_address() {
            if addr != &miflora_address {
                if cli.debug {
                    eprintln!("  address {:?} doesn't match, skipping", miflora_address);
                }
                return;
            }
        }
    }

    // check if we want readings from a specific name (can be multiple!)
    if let Some(name) = &cli.name {
        if let Ok(miflora_name) = miflora.get_name() {
            if name != &miflora_name {
                if cli.debug {
                    eprintln!("  name {:?} doesn't match, skipping", miflora_name);
                }
                return;
            }
        }
    }

    if cli.realtime {
        match miflora.realtime(cli.debug) {
            Ok(_) => (),
            Err(err) => eprintln!("{:?}", err),
        }
    }

    if cli.blink {
        match miflora.blink(cli.debug) {
            Ok(_) => (),
            Err(err) => eprintln!("{:?}", err),
        }
    }
}

fn do_main(cli: &Cli) -> Result<(), Box<Error>> {
    let scanner = Scanner::new(&cli)?;
    let mifloras = scanner.find_mifloras()?;

    for miflora in mifloras.iter() {
        if cli.debug {
            eprintln!("{:?}", miflora);
        }

        let r = miflora
            .connect()
            .map(|miflora| inquery_miflora(&miflora, &cli));

        if let Err(e) = r {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}

fn main() {
    let mut cli = Cli::from_args();

    if !cli.blink && !cli.realtime {
        cli.realtime = true;
    }

    match do_main(&cli) {
        Ok(_) => (),
        Err(err) => eprintln!("{:?}", err),
    }

    ()
}
