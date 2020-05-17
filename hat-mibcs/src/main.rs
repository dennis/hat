extern crate byteorder;
extern crate dbus_common;
extern crate chrono;

#[macro_use]
extern crate log;

mod cli;
mod scanner;
mod weight_data;

use cli::Cli;
use scanner::Scanner;

use structopt::StructOpt;

// data from bluetooth scan will look like:
// length: 1
//   type: 1 "Flags"
//   data: [6]
// length: 2
//   type: 2 "Incomplete List of 16-bit Service Class UUIDs"
//   data: [27, 24]
// length: 15
//   type: 22 "Service Data/Service Data - 16-bit UUID"
//   data: [  27   24    2 166   227    7    5    1   22   32   42  150    1  176   74]
//   data: [0x1b 0x18 0x02 0xa6 0xe3 0x07 0x05 0x01 0x16 0x20 0x2a 0x96 0x01 0xb0 0x4a
//          id1  id2  ctr1 ctr2 yea1 yea2 mon  day  hh   mm   ss   i1   i2   w1   w2
// id: ?
//   id = 0x1b 0x18
//   seems to be fixed
// measure unit:
//   ctr1
//   ctr2
//     bit 0 = ??
//     bit 1 = have impedance
//     bit 2 = have weight?
//     bit 3 = ??
//     bit 4 = ??
//     bit 5 = weight stabilized
//     bit 6 = ??
//     bit 7 = impedance stabilized?
// year
//   yea1 = 0xe3
//   yea2 = 0x07
//   year = yea2+yea1 = 0x07e3 = 2019
// impedance ?
//   i1 = 150 (0x96)
//   i2 =   1 (0x01)
//   implance = i2+i1 = 0x0196 = 406
// weight: (weight is delivered as jin, to convert it to kg, devide by two)
//   w1 = 176 (0xb0)
//   w2 =  72 (0x4a)
//   weight = w2+w1 = 0x4ab0 = 191200 * 0.01 / 2 = 95.6 kg
//   *100 for pounds and catty, *200 for kilograms


fn main() {
    env_logger::init();

    let cli = Cli::from_args();

    match Scanner::new(&cli) {
        Ok(scanner) => {
            match scanner.listen_for_signals() {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("ERROR: {:?}", error);
                }
            }
        }
        Err(error) => {
            eprintln!("ERROR: {:?}", error);
        }
    }
}
