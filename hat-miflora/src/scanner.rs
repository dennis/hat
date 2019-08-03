static ROOT_SERVICE_UUID: &'static str = "0000fe95-0000-1000-8000-00805f9b34fb";

static DATA_SERVICE_UUID: &'static str = "00001204-0000-1000-8000-00805f9b34fb";
static FIRMWARE_CHAR_UUID: &'static str = "00001a02-0000-1000-8000-00805f9b34fb";
static COMMAND_CHAR_UUID: &'static str = "00001a00-0000-1000-8000-00805f9b34fb";
static DATA_CHAR_UUID: &'static str = "00001a01-0000-1000-8000-00805f9b34fb";

use std::error::Error;

use std::thread;
use std::time::Duration;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as Characteristic;
use blurz::bluetooth_gatt_service::BluetoothGATTService as Service;
use blurz::bluetooth_session::BluetoothSession as Session;
use chrono::prelude::*;
use serde::Serialize;

use crate::Cli;

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


pub struct Scanner<'a> {
    cli : &'a Cli,
}

impl<'a> Scanner<'a> {
    pub fn new(cli : &'a Cli) -> Scanner {
        Scanner { cli }
    }

    pub fn scan(&self) -> Result<(), Box<Error>> {
        let bt_session = &Session::create_session(None)?;
        let adapter: Adapter = Adapter::init(bt_session)?;
        let session = DiscoverySession::create_session(
            &bt_session,
            adapter.get_id()
        )?;

        session.start_discovery()?;

        thread::sleep(Duration::from_millis(1000));

        session.stop_discovery()?;

        let devices = adapter.get_device_list()?;
        if devices.is_empty() {
            if self.cli.debug {
                eprintln!("No devices found");
            }
            return Ok(());
        }

        if self.cli.debug {
            eprintln!("devices: {:?}", devices);
        }

        for d in devices {
            if let Err(err) = self.inquiry_device(bt_session, &d) {
                if self.cli.debug { eprintln!("  inquiry_device failed: {:?}", err); }
            }
        }

        Ok(())
    }

    fn inquiry_device(&self, bt_session : &Session, device_path : &String) -> Result<(), Box<Error>> {
        if self.cli.debug { eprintln!("device: {:?}", device_path); }

        let device = Device::new(bt_session, device_path.clone());
        let uuids = device.get_uuids()?;
        'uuid_loop: for uuid in uuids {
            if uuid == ROOT_SERVICE_UUID {
                if self.cli.debug { eprintln!("  Found correct UUID"); }
                device.connect(20000)?;

                if self.cli.debug { eprintln!("  connected"); }

                // We need to wait a bit after calling connect to safely
                // get the gatt services
                thread::sleep(Duration::from_millis(5000));

                device.get_gatt_services()?;

                if let Err(error) = self.scan_device(bt_session, &device) {
                    if self.cli.debug { eprintln!("  scan_device error: {:?}", error); }
                }

                break;
            }
        }

        Ok(())
    }

    fn scan_device(&self, bt_session : &Session, device : &Device) -> Result<(), Box<Error>> {
        let mut command_char : Option<Characteristic> = None;
        let mut data_char : Option<Characteristic> = None;
        let mut firmware_char : Option<Characteristic> = None;

        let services = device.get_gatt_services()?;
        for service in services {
            let s = Service::new(bt_session, service.clone());
            if s.get_uuid()? == DATA_SERVICE_UUID {
                if self.cli.debug { eprintln!("  found data service"); }
                let characteristics = s.get_gatt_characteristics()?;
                for characteristic in characteristics {
                    let c = Characteristic::new(bt_session, characteristic.clone());
                    if c.get_uuid()? == FIRMWARE_CHAR_UUID {
                        if self.cli.debug { eprintln!("    reading firmware char. uuid"); }
                        firmware_char = Some(c.clone())
                    }

                    if c.get_uuid()? == COMMAND_CHAR_UUID {
                        if self.cli.debug { eprintln!("    reading command char. uuid"); }
                        command_char = Some(c.clone())
                    }

                    if c.get_uuid()? == DATA_CHAR_UUID {
                        if self.cli.debug { eprintln!("    reading data char. uuid"); }
                        data_char = Some(c.clone());
                    }
                }
            }
        }

        if command_char.is_none() || data_char.is_none() || firmware_char.is_none() {
            if self.cli.debug { eprintln!("  not all characteristics are available, aborting"); }
            return Ok(())
        }

        let command_char = command_char.unwrap();
        let data_char = data_char.unwrap();
        let firmware_char = firmware_char.unwrap();

        command_char.write_value([0xa0, 0x1f].to_vec(), None)?;

        if command_char.read_value(None)? != [0xa0, 0x1f] {
            if self.cli.debug { eprintln!("  Unexpected value from command char. uuid"); }
            return Ok(())
        }

        let value = data_char.read_value(None)?;

        if self.cli.debug { eprintln!("  parsing data: {:?}", value.clone()); }

        let mut rdr      = Cursor::new(value.clone());

        let temperature      = rdr.read_u16::<LittleEndian>()? as f32 * 0.1; // byte 0-1
        let _unknown         = rdr.read_u8()?;                               //      2
        let lux              = rdr.read_u32::<LittleEndian>()?;              //      3-6
        let moisture         = rdr.read_u8()?;                               //      7
        let conductivity     = rdr.read_u16::<LittleEndian>()?;              //      8-9

        let value = firmware_char.read_value(None)?;

        if value.len() != 7 {
            if self.cli.debug { eprintln!("  Unexpected value for firmware char. value"); }
            // Something went wrong
            return Ok(())
        }
        let battery = value[0];
        let version = String::from_utf8(value[2 .. 7].to_vec())?;

        command_char.write_value([0xb0, 0xff].to_vec(), None)?;

        if command_char.read_value(None)? != [0xb0, 0xff] {
            if self.cli.debug { eprintln!("  Couldn't change mode"); }
            return Ok(())
        }

        let serial = hex::encode(data_char.read_value(None)?);

        let readings = MifloraReadings {
            source: &"hat-miflora".to_string(),
            name: &device.get_alias()?,
            address : &device.get_address()?,
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

        device.disconnect()?;

        Ok(())
    }

}
