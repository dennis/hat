extern crate dbus;
extern crate byteorder;

extern crate btlepasvscan;

#[macro_use]
extern crate enum_primitive;
extern crate chrono;

use structopt::StructOpt;

mod bluetooth;
mod mibcs; // Mi Body Composition Scale

mod dbus_adapter;

use std::boxed::Box;
use std::error::Error;
use std::io::Cursor;
use dbus::{Connection, ConnectionItem, BusType, SignalArgs, Message, MessageItem};
use dbus::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::MessageType::Signal;
use byteorder::{LittleEndian, ReadBytesExt};
use chrono::prelude::*;
use serde::Serialize;

use mibcs::scanner::MIBCSScanner;

static SERVICE_NAME: &'static str = "org.bluez";
static ADAPTER_INTERFACE: &'static str = "org.bluez.Adapter1";
static DEVICE_NAME: &'static str = "org.bluez.Device1";

static BODY_COMPOSITION_UUID: &'static str = "0000181b-0000-1000-8000-00805f9b34fb";

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

#[derive(Debug)]
pub enum AppError {
    DecodeError,
    JsonSerializationError
}

#[derive(Clone, Serialize)]
pub struct WeightData {
    pub address: String,
    #[serde(with = "my_date_format", rename = "datetime")]
    pub created_at: DateTime<Local>,
    pub weight: Option<f32>,
    pub impedance: Option<u16>,
}

mod my_date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

type AppResult = Result<(), Box<AppError>>;

impl std::convert::From<std::io::Error> for Box<AppError> {
    fn from(_error: std::io::Error) -> Box<AppError> {
        Box::new(AppError::DecodeError)
    }
}

impl std::convert::From<serde_json::error::Error> for Box<AppError> {
    fn from(_error: serde_json::error::Error) -> Box<AppError> {
        Box::new(AppError::JsonSerializationError)
    }
}

fn decode_mibcs_data(value : &Vec<u8>) -> AppResult {
    let mut rdr      = Cursor::new(value.clone());

    let statusbit0  = rdr.read_u8()?;
    let statusbis1  = rdr.read_u8()?;
    let year  = rdr.read_u16::<LittleEndian>()?;
    let month  = rdr.read_u8()?;
    let day  = rdr.read_u8()?;
    let hh  = rdr.read_u8()?;
    let mm  = rdr.read_u8()?;
    let ss  = rdr.read_u8()?;
    let impedance  = rdr.read_u16::<LittleEndian>()?;
    let weight  = rdr.read_u16::<LittleEndian>()? as f32 * 0.01 / 2.0;

    // 0b001010010000000010
    let got_impedance: bool = statusbis1 & 0b00000010 != 0;
    let got_weight: bool = statusbis1 & 0b00000100 != 0;
    let weight_stabilized: bool = statusbis1 & 0b00100000 != 0;
    let impedance_stabilized: bool = statusbis1 & 0b10000000 != 0;

    // println!(" statusbit0 = {:#010b}", statusbit0);
    // println!(" statusbis1 = {:#010b}", statusbis1);
    // println!(" year = {:?}", year);
    // println!(" month = {:?}", month);
    // println!(" day = {:?}", day);
    // println!(" hh = {:?}", hh);
    // println!(" mm = {:?}", mm);
    // println!(" ss = {:?}", ss);
    // println!(" impedance = {:?}", impedance);
    // println!(" weight = {:?}", weight);
    // println!(" got_impedance = {:?}", got_impedance);
    // println!(" got_weight = {:?}", got_weight);
    // println!(" weight_stabilized = {:?}", weight_stabilized);
    // println!(" impedance_stabilized = {:?}", impedance_stabilized);

    let weight = if weight_stabilized { Some(weight) } else {  None };
    let impedance = if impedance_stabilized { Some(impedance) } else {  None };

    let data = WeightData {
        address: "??".to_string(),
        created_at: Local::now(),
        weight,
        impedance
    };

    println!("{}", serde_json::to_string(&data)?);

    Ok(())
}

fn extract_body_composition(key : &dbus::MessageItem, value : &dbus::MessageItem) -> Option<AppResult> {
    if let dbus::MessageItem::Str(key) = key {
        println!("  uuid : {:?}", key);

        if key != BODY_COMPOSITION_UUID {
            return None;
        }

        if let dbus::MessageItem::Variant(value) = value {
            let value : &dbus::MessageItem = value;
            if let dbus::MessageItem::Array(value) = value {

                let bytes = value.as_ref().to_vec().into_iter().filter_map(|x| x.inner::<u8>().ok()).collect();

                return Some(decode_mibcs_data(&bytes))
            }
        }
    }

    None
}

fn inquiry_service_data_values(value : &dbus::MessageItem) -> Option<AppResult>  {
    if let dbus::MessageItem::Array(value) = value {
        for v in value.as_ref() {
            if let dbus::MessageItem::DictEntry(key, value) = v {
                return extract_body_composition(&key, &value);
            }
        }
    }

    None
}

fn inquiry_service_data(key : &dbus::MessageItem, value : &dbus::MessageItem) -> Option<AppResult> {
    if let dbus::MessageItem::Str(key) = key {
        println!("  {:?}", key);
        if key == "ServiceData" {
            println!("  Got service data!");

            if let dbus::MessageItem::Variant(value) = value {
                return inquiry_service_data_values(value);
            }
        }
    }

    None
}

fn inquiry_changed_properties(connection : &Connection, path : &String, item_vec : Vec<dbus::MessageItem>) -> Option<AppResult> {
    // get property
    //
    // dbus-send --print-reply --type=method_call --system --dest=org.bluez
    // /org/bluez/hci0/dev_EF_FB_0D_B1_43_97 org.freedesktop.DBus.Properties.Get
    // string:org.bluez.Device1 string:Address

    let mut m = Message::new_method_call(SERVICE_NAME, path, "org.freedesktop.DBus.Properties", "Get").unwrap();
    m.append_items(&[
        MessageItem::Str("org.bluez.Device1".to_string()), // interface
        MessageItem::Str("Address".to_string()), // name
    ]);
    println!(" get address: {:?}", connection.send_with_reply_and_block(m, 1000).unwrap().get_items());

    let mut m = Message::new_method_call(SERVICE_NAME, path, "org.freedesktop.DBus.Properties", "Get").unwrap();
    m.append_items(&[
        MessageItem::Str("org.bluez.Device1".to_string()), // interface
        MessageItem::Str("UUIDs".to_string()), // name
    ]);

    // get uuids: [Variant(Array(MessageItemArray { v:
    // [Str("00001530-0000-3512-2118-0009af100700"), Str("00001800-0000-1000-8000-00805f9b34fb"),
    // Str("00001801-0000-1000-8000-00805f9b34fb"), Str("0000180a-0000-1000-8000-00805f9b34fb"),
    // Str("0000181b-0000-1000-8000-00805f9b34fb")], sig: Signature("as") }))]
    //

    println!(" get uuids: {:?}", connection.send_with_reply_and_block(m, 1000).unwrap().get_items()[0]);

    for item in item_vec {
        println!("  - {:?}", item);

        if let dbus::MessageItem::DictEntry(key, value) = item {
            return inquiry_service_data(&key, &value);
        }
    }

    None
}

fn listen_for_signals(connection : &Connection) -> Result<(), Box<Error>> {
    let ppc = PropertiesPropertiesChanged::match_str(None, None);
    println!("{:?}", ppc);

    let m = Message::new_method_call(SERVICE_NAME, "/org/bluez/hci0", ADAPTER_INTERFACE, "StartDiscovery")?;
    connection.send_with_reply_and_block(m, 1000)?;

    connection.add_match(&ppc);

    for n in connection.iter(5000) {
        match n {
            ConnectionItem::Signal(signal) => {
                let (message_type, path, interface, member) =  signal.headers();

                if message_type == Signal && interface == Some("org.freedesktop.DBus.Properties".to_string()) && member == Some("PropertiesChanged".to_string()) {
                    let items = signal.get_items();

                    if items[0] == dbus::MessageItem::Str("org.bluez.Device1".to_string()) {
                        println!(" ********* ");

                        if let dbus::MessageItem::Array(e) = &items[1] {
                            let result = inquiry_changed_properties(&connection, &path.unwrap(), e.to_vec());

                            if result.is_some() {
                                match result.unwrap() {
                                    Ok(_) => println!("Got data"),
                                    Err(err) => println!("got problem {:?}", err)
                                }
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }

    Ok(())
}

fn scan_for_mibcs() -> Result<(), Box<Error>> {
    let connection = Connection::get_private(BusType::System)?;

    listen_for_signals(&connection)?;

    Ok(())
}


fn main() {
    match scan_for_mibcs() {
        Ok(_) => {}
        Err(error) => {
            println!("ERROR: {:?}", error);
        }
    }
}
