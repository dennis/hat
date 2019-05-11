extern crate btlepasvscan;

#[macro_use]
extern crate enum_primitive;

use structopt::StructOpt;

mod bluetooth;
mod mibcs; // Mi Body Composition Scale

use mibcs::scanner::MIBCSScanner;
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

#[derive(StructOpt)]
pub struct Cli {
    /// Wait for data and exit when received
    #[structopt(short = "1")]
    until_data: bool,
    /// Enable debug (pretty verbose)
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// How many seconds should it wait for weight data. 0 is forever
    #[structopt(short = "s", long = "seconds", default_value = "60")]
    duration: u64,
}

fn main() {
    let cli = Cli::from_args();
    let mut scanner = MIBCSScanner::new(&cli);

    match scanner.scan() {
        Ok(_) => {}
        Err(error) => {
            println!("ERROR: {:?}", error.message);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mibcs::weight_data::WeightData;

    #[test]
    fn decoding_test1() {
        let data = [
            0x02, 0x01, 0x06, 0x03, 0x02, 0x1b, 0x18, 0x10, 0x16, 0x1b, 0x18, 0x02, 0xa6, 0xe3,
            0x07, 0x05, 0x01, 0x16, 0x20, 0x2a, 0x96, 0x01, 0xb0, 0x4a,
        ];
        let scale_data = WeightData::parse(
            "FF:FF:FF:FF:FF:FF",
            bluetooth::advertising_data::parse_ad(&data).as_slice(),
        )
        .unwrap();
        scale_data.dump();
        assert!(scale_data.weight == 95.6);
        assert!(scale_data.impedance == 406);
        assert!(scale_data.got_impedance == true);
        assert!(scale_data.got_weight == true);
        assert!(scale_data.impedance_stabilized == true);
        assert!(scale_data.weight_stabilized == true);
    }

    #[test]
    fn decoding_test2() {
        // https://community.home-assistant.io/t/integrating-xiaomi-mi-scale/9972/81
        let data = [
            0x02, 0x01, 0x06, 0x03, 0x02, 0x1b, 0x18, 0x10, 0x16, 0x1b, 0x18, 0x02, 0xa6, 0xe2,
            0x07, 0x05, 0x15, 0x0a, 0x25, 0x1d, 0xf4, 0x01, 0x44, 0x3e,
        ];
        let scale_data = WeightData::parse(
            "FF:FF:FF:FF:FF:FF",
            bluetooth::advertising_data::parse_ad(&data).as_slice(),
        )
        .unwrap();
        scale_data.dump();
        assert!(scale_data.weight == 79.7);
        assert!(scale_data.impedance == 500);
        assert!(scale_data.got_impedance == true);
        assert!(scale_data.got_weight == true);
        assert!(scale_data.impedance_stabilized == true);
        assert!(scale_data.weight_stabilized == true);

        let data = [
            0x02, 0x01, 0x06, 0x03, 0x02, 0x1b, 0x18, 0x10, 0x16, 0x1b, 0x18, 0x02, 0xa6, 0xe2,
            0x07, 0x05, 0x18, 0x0c, 0x0d, 0x04, 0xd7, 0x01, 0x94, 0x3e,
        ];
        let scale_data = WeightData::parse(
            "FF:FF:FF:FF:FF:FF",
            bluetooth::advertising_data::parse_ad(&data).as_slice(),
        )
        .unwrap();
        scale_data.dump();
        assert!(scale_data.weight == 80.1);
        assert!(scale_data.impedance == 471);
        assert!(scale_data.got_impedance == true);
        assert!(scale_data.got_weight == true);
        assert!(scale_data.impedance_stabilized == true);
        assert!(scale_data.weight_stabilized == true);
    }
}
