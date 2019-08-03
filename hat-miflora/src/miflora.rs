static DATA_SERVICE_UUID: &'static str = "00001204-0000-1000-8000-00805f9b34fb";
static FIRMWARE_CHAR_UUID: &'static str = "00001a02-0000-1000-8000-00805f9b34fb";
static COMMAND_CHAR_UUID: &'static str = "00001a00-0000-1000-8000-00805f9b34fb";
static DATA_CHAR_UUID: &'static str = "00001a01-0000-1000-8000-00805f9b34fb";

use std::error::Error;

use std::thread;
use std::time::Duration;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as Characteristic;
use blurz::bluetooth_gatt_service::BluetoothGATTService as Service;
use blurz::bluetooth_session::BluetoothSession as Session;

use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug)]
pub struct Miflora {
    device_path : String,
}

impl Miflora {
    pub fn new(device_path : String) -> Miflora {
        Miflora {
            device_path,
        }
    }

    pub fn connect<'a>(&self, bt_session : &'a Session) -> Result<ConnectedMiflora<'a>, Box<Error>> {
        let device = Device::new(bt_session, self.device_path.clone());

        device.connect(20000)?;

        if true { eprintln!("  connected"); }

        Ok(ConnectedMiflora { device })
    }
}
#[derive(Clone, Serialize)]
struct MifloraReadings<'t> {
    source : &'t String,
    name : &'t String,
    address : &'t String,
    #[serde(with = "my_date_format", rename = "datetime")]
    pub created_at: DateTime<Local>,
    temperature : f32,
    lux : u32,
    moisture : u8,
    conductivity : u16,
    battery : u8,
    version : &'t String,
    serial  : &'t String,
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

#[derive(Debug)]
pub struct ConnectedMiflora<'a> {
    pub device : Device<'a>
}

impl<'a> ConnectedMiflora<'a> {
    pub fn read(&self, bt_session : &Session, debug : bool) -> Result<(), Box<Error>> {
        let mut command_char : Option<Characteristic> = None;
        let mut data_char : Option<Characteristic> = None;
        let mut firmware_char : Option<Characteristic> = None;

        // We need to wait a bit after calling connect to safely
        // get the gatt services
        thread::sleep(Duration::from_millis(5000));
        let services = self.device.get_gatt_services()?;

        for service in services {
            let s = Service::new(bt_session, service.clone());
            if s.get_uuid()? == DATA_SERVICE_UUID {
                if debug { eprintln!("  found data service"); }
                let characteristics = s.get_gatt_characteristics()?;
                for characteristic in characteristics {
                    let c = Characteristic::new(bt_session, characteristic.clone());
                    if c.get_uuid()? == FIRMWARE_CHAR_UUID {
                        if debug { eprintln!("    reading firmware char. uuid"); }
                        firmware_char = Some(c.clone())
                    }

                    if c.get_uuid()? == COMMAND_CHAR_UUID {
                        if debug { eprintln!("    reading command char. uuid"); }
                        command_char = Some(c.clone())
                    }

                    if c.get_uuid()? == DATA_CHAR_UUID {
                        if debug { eprintln!("    reading data char. uuid"); }
                        data_char = Some(c.clone());
                    }
                }
            }
        }

        if command_char.is_none() || data_char.is_none() || firmware_char.is_none() {
            if debug { eprintln!("  not all characteristics are available, aborting"); }
            return Ok(())
        }

        let command_char = command_char.unwrap();
        let data_char = data_char.unwrap();
        let firmware_char = firmware_char.unwrap();

        command_char.write_value([0xa0, 0x1f].to_vec(), None)?;

        if command_char.read_value(None)? != [0xa0, 0x1f] {
            if debug { eprintln!("  Unexpected value from command char. uuid"); }
            return Ok(())
        }

        let value = data_char.read_value(None)?;

        if debug { eprintln!("  parsing data: {:?}", value.clone()); }

        let mut rdr      = Cursor::new(value.clone());

        let temperature      = rdr.read_u16::<LittleEndian>()? as f32 * 0.1; // byte 0-1
        let _unknown         = rdr.read_u8()?;                               //      2
        let lux              = rdr.read_u32::<LittleEndian>()?;              //      3-6
        let moisture         = rdr.read_u8()?;                               //      7
        let conductivity     = rdr.read_u16::<LittleEndian>()?;              //      8-9

        let value = firmware_char.read_value(None)?;

        if value.len() != 7 {
            if debug { eprintln!("  Unexpected value for firmware char. value"); }
            // Something went wrong
            return Ok(())
        }
        let battery = value[0];
        let version = String::from_utf8(value[2 .. 7].to_vec())?;

        command_char.write_value([0xb0, 0xff].to_vec(), None)?;

        if command_char.read_value(None)? != [0xb0, 0xff] {
            if debug { eprintln!("  Couldn't change mode"); }
            return Ok(())
        }

        let serial = hex::encode(data_char.read_value(None)?);

        let readings = MifloraReadings {
            source: &"hat-miflora".to_string(),
            name: &self.device.get_alias()?,
            address : &self.device.get_address()?,
            created_at : Local::now(),
            temperature: temperature,
            lux: lux,
            moisture: moisture,
            conductivity: conductivity,
            battery: battery,
            version: &version,
            serial: &serial
        };

        println!("{}", serde_json::to_string(&readings)?);

        self.device.disconnect()?;

        Ok(())
    }
}
