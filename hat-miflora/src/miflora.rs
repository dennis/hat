static DATA_SERVICE_UUID: &'static str = "00001204-0000-1000-8000-00805f9b34fb";
static FIRMWARE_CHAR_UUID: &'static str = "00001a02-0000-1000-8000-00805f9b34fb";
static DEVICE_MODE_CHAR_UUID: &'static str = "00001a00-0000-1000-8000-00805f9b34fb";
static DATA_CHAR_UUID: &'static str = "00001a01-0000-1000-8000-00805f9b34fb";

// This is the values for DEVICE_MODE_CHAR_UUID
static DEVICE_MODE_REALTIME: [u8; 2] = [0xa0, 0x1f];
static DEVICE_MODE_BLINK: [u8; 2] = [0xfd, 0xff];

use std::error::Error;
use std::fmt;

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
struct DeviceModeChangeError;

impl Error for DeviceModeChangeError {
}

impl fmt::Display for DeviceModeChangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed changing device mode")
    }
}

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

        thread::sleep(Duration::from_millis(5000));
        let services = device.get_gatt_services()?;

        let data_service =
            services
               .iter()
               .map( |service| Service::new(bt_session, service.clone()) )
               .find(find_data_service)
               .ok_or("data service not found")?;

        let characteristics = data_service.get_gatt_characteristics()?;

        let characteristics_iter = || characteristics
            .clone()
            .into_iter()
            .map(|characteristic| Characteristic::new(bt_session, characteristic.clone()) );

        let firmware_char    = characteristics_iter().find(find_firmware_characteristic).ok_or("firmware characteristic not found")?;
        let device_mode_char = characteristics_iter().find(find_device_mode_characteristic).ok_or("device mode characteristic not found")?;
        let data_char        = characteristics_iter().find(find_data_characteristic).ok_or("data characteristic not found")?;

        Ok(ConnectedMiflora { device, firmware_char, device_mode_char, data_char })
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
    pub device : Device<'a>,
    firmware_char : Characteristic<'a>,
    device_mode_char : Characteristic<'a>,
    data_char : Characteristic<'a>,
}

fn find_data_service(service : &Service) -> bool {
    if let Ok(result) = service.get_uuid() {
        result == DATA_SERVICE_UUID
    }
    else {
        false
    }
}

fn find_device_mode_characteristic(characteristic: &Characteristic) -> bool {
    return characteristic.get_uuid().unwrap_or("".to_string()) == DEVICE_MODE_CHAR_UUID;
}

fn find_firmware_characteristic(characteristic : &Characteristic) -> bool {
    return characteristic.get_uuid().unwrap_or("".to_string()) == FIRMWARE_CHAR_UUID;
}

fn find_data_characteristic(characteristic : &Characteristic) -> bool {
    return characteristic.get_uuid().unwrap_or("".to_string()) == DATA_CHAR_UUID;
}

impl<'a> ConnectedMiflora<'a> {
    pub fn get_address(&self) -> Result<String, Box<Error>> {
        return self.device.get_address();
    }

    pub fn get_name(&self) -> Result<String, Box<Error>> {
        return self.device.get_alias();
    }

    pub fn blink(&self, debug : bool) -> Result<(), Box<Error>> {
        if debug { eprintln!("  BLINK"); }

        self.set_device_mode(DEVICE_MODE_BLINK, debug)?;

        Ok(())
    }

    pub fn realtime(&self, debug : bool) -> Result<(), Box<Error>> {
        if debug { eprintln!("  REALTIME READINGS"); }

        self.set_device_mode(DEVICE_MODE_REALTIME, debug)?;

        let value = self.data_char.read_value(None)?;

        if debug { eprintln!("  parsing data: {:?}", value.clone()); }

        let mut rdr          = Cursor::new(value.clone());

        let temperature      = rdr.read_u16::<LittleEndian>()? as f32 * 0.1; // byte 0-1
        let _unknown         = rdr.read_u8()?;                               //      2
        let lux              = rdr.read_u32::<LittleEndian>()?;              //      3-6
        let moisture         = rdr.read_u8()?;                               //      7
        let conductivity     = rdr.read_u16::<LittleEndian>()?;              //      8-9

        let value = self.firmware_char.read_value(None)?;

        if value.len() != 7 {
            if debug { eprintln!("  Unexpected value for firmware char. value"); }
            return Ok(())
        }

        let battery = value[0];
        let version = String::from_utf8(value[2 .. 7].to_vec())?;

        let serial = hex::encode(self.data_char.read_value(None)?);

        let readings = MifloraReadings {
            source: &"hat-miflora".to_string(),
            name: &self.get_name()?,
            address : &self.get_address()?,
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

    fn set_device_mode(&self, command : [u8; 2], debug : bool) -> Result<(), Box<Error>> {
        self.device_mode_char.write_value(command.to_vec(), None)?;

        if self.device_mode_char.read_value(None)? != command {
            if debug { eprintln!("  Unexpected value from command char. uuid"); }
            return Err(Box::new(DeviceModeChangeError))
        }

        Ok(())
    }

}
