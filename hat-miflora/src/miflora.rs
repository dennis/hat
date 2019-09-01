static DATA_SERVICE_UUID: &'static str = "00001204-0000-1000-8000-00805f9b34fb";
static FIRMWARE_CHAR_UUID: &'static str = "00001a02-0000-1000-8000-00805f9b34fb";
static DEVICE_MODE_CHAR_UUID: &'static str = "00001a00-0000-1000-8000-00805f9b34fb";
static DATA_CHAR_UUID: &'static str = "00001a01-0000-1000-8000-00805f9b34fb";

// This is the values for DEVICE_MODE_CHAR_UUID
static DEVICE_MODE_REALTIME: [u8; 2] = [0xa0, 0x1f];
static DEVICE_MODE_BLINK: [u8; 2] = [0xfd, 0xff];

use std::error;
use std::fmt;

use std::io::Cursor;
use std::ops::Drop;
use std::thread;
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt};

use chrono::prelude::*;
use serde::Serialize;

use dbus::{BusType, Connection, MessageItem};

use dbus_common::org_bluez_device1::OrgBluezDevice1;
use dbus_common::org_bluez_gatt_characteristic1::OrgBluezGattCharacteristic1;
use dbus_common::utils::SERVICE_NAME;

use dbus_common::utils::get_managed_objects_with_interface;

#[derive(Debug)]
pub enum AppError {
    DBusConnectError(dbus::Error),
    DBusDeviceConnectError(dbus::Error),
    DBusDeviceModeConfirmError,
    DBusDeviceModeReadError(dbus::Error),
    DBusDeviceModeWriteError(dbus::Error),
    DBusDeviceScanConnectError(dbus::Error),
    DBusDeviceScanDisconnectError(dbus::Error),
    DBusReadingPropertyError(dbus::Error, &'static str),
    BlinkError(Box<AppError>),
    JSONError,
    ReadingMifloraDataError,
    ReadingMifloraVersionError,
    ReadingFirmwareDataError,
    ReadingDataValues,
    UnexpectedFirmwareDataLength,
    ParsingConductivityError,
    ParsingMoistureError,
    ParsingLuxError,
    ParsingUnknownError,
    ParsingTemperatureError,
    CantFindFirmwareCharacteristicUuid,
    CantFindDeviceModeCharacteristicUuid,
    CantFindDataCharacteristicUuid,
    DBusCannotGetObjects,
    CannotFindDataServiceUuid,
    DBusCannotFindCharacteristics,
}
impl error::Error for AppError {}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?}", self)
    }
}

#[derive(Debug)]
pub struct Miflora {
    device_path: String,
    connection: Connection,
}

impl Miflora {
    pub fn new(device_path: String) -> Result<Miflora, AppError> {
        Ok(Miflora {
            connection: Connection::get_private(BusType::System)
                .map_err(|err| AppError::DBusConnectError(err))?,
            device_path,
        })
    }

    pub fn connect<'a>(&self) -> Result<ConnectedMiflora, AppError> {
        let device = self
            .connection
            .with_path(SERVICE_NAME, self.device_path.clone(), 5000);

        device.connect().map_err(|err| AppError::DBusDeviceScanConnectError(err))?;
        device
            .disconnect()
            .map_err(|err| AppError::DBusDeviceScanDisconnectError(err))?;

        let services = get_managed_objects_with_interface(
            &self.connection,
            &"org.bluez.GattService1",
            &self.device_path,
            "Device",
        )
        .map_err(|_| AppError::DBusCannotGetObjects)?;
        let data_service = services
            .iter()
            .find(|path| find_data_service(&self.connection, path))
            .ok_or(AppError::CannotFindDataServiceUuid)?;
        let characteristics = get_managed_objects_with_interface(
            &self.connection,
            &"org.bluez.GattCharacteristic1",
            &data_service,
            "Service",
        )
        .map_err(|_| AppError::DBusCannotFindCharacteristics)?;

        let firmware_char = characteristics
            .iter()
            .find(|path| find_characteristic(&self.connection, path, FIRMWARE_CHAR_UUID))
            .ok_or(AppError::CantFindFirmwareCharacteristicUuid)?;
        let device_mode_char = characteristics
            .iter()
            .find(|path| find_characteristic(&self.connection, path, DEVICE_MODE_CHAR_UUID))
            .ok_or(AppError::CantFindDeviceModeCharacteristicUuid)?;
        let data_char = characteristics
            .iter()
            .find(|path| find_characteristic(&self.connection, path, DATA_CHAR_UUID))
            .ok_or(AppError::CantFindDataCharacteristicUuid)?;

        ConnectedMiflora::new(
            &self.device_path,
            firmware_char,
            device_mode_char,
            data_char,
        )
    }
}

#[derive(Clone, Serialize)]
struct MifloraReadings<'t> {
    source: &'t String,
    name: &'t String,
    address: &'t String,
    #[serde(with = "my_date_format", rename = "datetime")]
    pub created_at: DateTime<Local>,
    temperature: f32,
    lux: u32,
    moisture: u8,
    conductivity: u16,
    battery: u8,
    version: &'t String,
    serial: &'t String,
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

fn find_data_service(connection: &Connection, path: &str) -> bool {
    let p = dbus_common::utils::get_property(connection, &"org.bluez.GattService1", path, "UUID");

    if let Ok(MessageItem::Str(result)) = p {
        result == DATA_SERVICE_UUID
    } else {
        false
    }
}

fn find_characteristic(connection: &Connection, path: &str, uuid: &str) -> bool {
    let p = dbus_common::utils::get_property(
        connection,
        &"org.bluez.GattCharacteristic1",
        path,
        "UUID",
    );

    if let Ok(MessageItem::Str(result)) = p {
        result == uuid
    } else {
        false
    }
}

#[derive(Debug)]
pub struct ConnectedMiflora {
    connection: Connection,
    device_objpath: String,
    firmware_objpath: String,
    device_mode_objpath: String,
    data_objpath: String,
}

impl ConnectedMiflora {
    pub fn new(
        device_objpath: &str,
        firmware_objpath: &str,
        device_mode_objpath: &str,
        data_objpath: &str,
    ) -> Result<ConnectedMiflora, AppError> {
        let connection =
            Connection::get_private(BusType::System).map_err(|err| AppError::DBusConnectError(err))?;
        let device = connection.with_path(SERVICE_NAME, device_objpath.clone(), 10000);

        device.connect().map_err(|err| AppError::DBusDeviceConnectError(err))?;

        Ok(ConnectedMiflora {
            connection,
            device_objpath: device_objpath.to_string(),
            firmware_objpath: firmware_objpath.to_string(),
            device_mode_objpath: device_mode_objpath.to_string(),
            data_objpath: data_objpath.to_string(),
        })
    }

    pub fn get_address(&self) -> Result<String, AppError> {
        let device = self
            .connection
            .with_path(SERVICE_NAME, self.device_objpath.clone(), 5000);
        device
            .get_address()
            .map_err(|err| AppError::DBusReadingPropertyError(err, "address"))
    }

    pub fn get_name(&self) -> Result<String, AppError> {
        let device = self
            .connection
            .with_path(SERVICE_NAME, self.device_objpath.clone(), 5000);
        device
            .get_alias()
            .map_err(|err| AppError::DBusReadingPropertyError(err, "alias"))
    }

    pub fn blink(&self, debug: bool) -> Result<(), AppError> {
        if debug {
            eprintln!("  BLINK");
        }

        self.set_device_mode(DEVICE_MODE_BLINK, debug)
            .map_err(|err| AppError::BlinkError(Box::new(err)))?;

        Ok(())
    }

    pub fn realtime(&self, debug: bool) -> Result<(), AppError> {
        if debug {
            eprintln!("  REALTIME READINGS using {:?}", self.firmware_objpath);
        }

        self.set_device_mode(DEVICE_MODE_REALTIME, debug)?;

        let value = self
            .connection
            .with_path(SERVICE_NAME, &self.data_objpath, 5000)
            .read_value(std::collections::HashMap::new())
            .map_err(|_| AppError::ReadingDataValues)?;

        if debug {
            eprintln!("  parsing data: {:?}", value.clone());
        }

        let mut rdr = Cursor::new(value.clone());

        let temperature = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| AppError::ParsingTemperatureError)? as f32
            * 0.1; // byte 0-1
        let _unknown = rdr.read_u8().map_err(|_| AppError::ParsingUnknownError)?; //      2
        let lux = rdr
            .read_u32::<LittleEndian>()
            .map_err(|_| AppError::ParsingLuxError)?; //      3-6
        let moisture = rdr.read_u8().map_err(|_| AppError::ParsingMoistureError)?; //      7
        let conductivity = rdr
            .read_u16::<LittleEndian>()
            .map_err(|_| AppError::ParsingConductivityError)?; //      8-9

        let value = self
            .connection
            .with_path(SERVICE_NAME, &self.firmware_objpath, 5000)
            .read_value(std::collections::HashMap::new())
            .map_err(|_| AppError::ReadingFirmwareDataError)?;
        if value.len() != 7 {
            if debug {
                eprintln!("  Unexpected value for firmware char. value");
            }
            return Err(AppError::UnexpectedFirmwareDataLength);
        }

        let battery = value[0];
        let version = String::from_utf8(value[2..7].to_vec())
            .map_err(|_| AppError::ReadingMifloraVersionError)?;

        let value = self
            .connection
            .with_path(SERVICE_NAME, &self.data_objpath, 5000)
            .read_value(std::collections::HashMap::new())
            .map_err(|_| AppError::ReadingMifloraDataError)?;
        let serial = hex::encode(value);

        let readings = MifloraReadings {
            source: &"hat-miflora".to_string(),
            name: &self.get_name()?,
            address: &self.get_address()?,
            created_at: Local::now(),
            temperature: temperature,
            lux: lux,
            moisture: moisture,
            conductivity: conductivity,
            battery: battery,
            version: &version,
            serial: &serial,
        };

        println!(
            "{}",
            serde_json::to_string(&readings).map_err(|_| AppError::JSONError)?
        );

        Ok(())
    }

    fn set_device_mode(&self, command: [u8; 2], debug: bool) -> Result<(), AppError> {
        let device_mode_char =
            self.connection
                .with_path(SERVICE_NAME, &self.device_mode_objpath, 5000);

        device_mode_char
            .write_value(command.to_vec(), std::collections::HashMap::new())
            .map_err(|err| AppError::DBusDeviceModeWriteError(err))?;

        if device_mode_char
            .read_value(std::collections::HashMap::new())
            .map_err(|err| AppError::DBusDeviceModeReadError(err))?
            != command
        {
            if debug {
                eprintln!("  Unexpected value from command char. uuid");
            }
            return Err(AppError::DBusDeviceModeConfirmError);
        }

        Ok(())
    }
}

impl Drop for ConnectedMiflora {
    fn drop(&mut self) {
        let device = self
            .connection
            .with_path(SERVICE_NAME, self.device_objpath.clone(), 5000);

        device.disconnect().ok();
    }
}
