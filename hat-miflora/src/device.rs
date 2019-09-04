use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::{error, thread};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use dbus::arg::RefArg;
use serde::export::Formatter;

use dbus_common::org_bluez_device1::OrgBluezDevice1;
use dbus_common::org_bluez_gatt_characteristic1::OrgBluezGattCharacteristic1;

use crate::dbus_bluez::{
    BluezManager, DBusPath, TypedDbusError, BLUEZ_GATT_CHARACTERISTIC_INTERFACE, BLUEZ_SERVICE,
};
use std::time::Duration;

pub(crate) const XIAOMI_MIFLORA_SERVICE_UUID: &str = "0000fe95-0000-1000-8000-00805f9b34fb";

#[derive(Debug)]
pub(crate) enum Error {
    GATTAttributeNotFound {
        name: String,
        uuid: String,
    },
    InvalidData {
        cause: std::io::Error,
    },
    ErrorConnecting {
        cause: TypedDbusError,
    },
    ErrorDisconnecting {
        cause: TypedDbusError,
    },
    ErrorReadingData {
        name: String,
        uuid: String,
        path: String,
        cause: TypedDbusError,
    },
    ErrorWritingData {
        name: String,
        uuid: String,
        path: String,
        cause: TypedDbusError,
    },
    DBusError {
        cause: TypedDbusError,
    },
    ThisShouldNeverHappend,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub(crate) struct Miflora<'a> {
    manager: &'a mut BluezManager,
    device: DBusPath,
    firmware: Option<DBusPath>,
    firmware_raw: Option<Vec<u8>>,
    device_mode: Option<DBusPath>,
    device_data: Option<DBusPath>,
    device_time: Option<DBusPath>,
    history_mode: Option<DBusPath>,
    history_data: Option<DBusPath>,
}

enum MifloraDeviceMode {
    Realtime,
    Blink,
}

enum MifloraDeviceHistoryMode {
    Init,
    Clear,
    ReadRecord(u16),
}

pub(crate) struct RealtimeReadings {
    pub temperature: f32,
    pub lux: u32,
    pub moisture: u8,
    pub conductivity: u16,
}

pub(crate) struct HistoryReadings {
    pub record_number: u16,
    pub epoch: u32,
    pub temperature: f32,
    pub lux: u32,
    pub moisture: u8,
    pub conductivity: u16,
}

impl<'a> Miflora<'a> {
    const FIRMWARE_CHARACTERISTIC_UUID: &'static str = "00001a02-0000-1000-8000-00805f9b34fb";
    const DEVICE_MODE_CHARACTERISTIC_UUID: &'static str = "00001a00-0000-1000-8000-00805f9b34fb";
    const DEVICE_DATA_CHARACTERISTIC_UUID: &'static str = "00001a01-0000-1000-8000-00805f9b34fb";
    const DEVICE_TIME_CHARACTERISTIC_UUID: &'static str = "00001a12-0000-1000-8000-00805f9b34fb";
    const HISTORY_MODE_CHARACTERISTIC_UUID: &'static str = "00001a10-0000-1000-8000-00805f9b34fb";
    const HISTORY_DATA_CHARACTERISTIC_UUID: &'static str = "00001a11-0000-1000-8000-00805f9b34fb";

    pub fn new(
        device: DBusPath,
        manager: &'a mut BluezManager,
    ) -> Result<Miflora, Box<dyn error::Error>> {
        Ok(Miflora {
            manager,
            device,
            firmware: None,
            firmware_raw: None,
            device_mode: None,
            device_data: None,
            device_time: None,
            history_mode: None,
            history_data: None,
        })
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn error::Error>> {
        info!("{:} connect()", self.device.path);

        self.device.connect()?;

        if !self.device.get_connected()? {
            error!("Can't connect to device");

            return Ok(()); // FIXME
        }

        self.find_gatt_attributes();

        // debug!("DMP DEBUG DeviceMode={:?}, HistoryMode={:?}",
        //        self.read_attr(
        //            &self.device_mode,
        //            "device mode",
        //            Self::DEVICE_MODE_CHARACTERISTIC_UUID,
        //            |v| Ok(v.get_ref().clone())
        //        ),
        //        self.read_attr(
        //            &self.history_mode,
        //            "history mode",
        //            Self::HISTORY_MODE_CHARACTERISTIC_UUID,
        //            |v| Ok(v.get_ref().clone())
        //        )
        // );

        debug!("connected - looking up UUIDs");

        thread::sleep(Duration::from_millis(200));

        debug!("connected");

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), Error> {
        debug!("disconnect: {:?}", self.device.path);
        self.device.disconnect().map_err(|err| Error::DBusError {
            cause: TypedDbusError::from(err),
        })
    }

    pub fn get_address(&self) -> Result<String, Error> {
        OrgBluezDevice1::get_address(&self.device).map_err(|err| Error::DBusError {
            cause: TypedDbusError::from(err),
        })
    }

    pub fn get_rssi(&self) -> Result<i16, Error> {
        OrgBluezDevice1::get_rssi(&self.device).map_err(|err| Error::DBusError {
            cause: TypedDbusError::from(err),
        })
    }

    pub fn get_name(&self) -> Result<String, Error> {
        OrgBluezDevice1::get_name(&self.device).map_err(|err| Error::DBusError {
            cause: TypedDbusError::from(err),
        })
    }

    pub fn get_alias(&self) -> Result<String, Error> {
        OrgBluezDevice1::get_alias(&self.device).map_err(|err| Error::DBusError {
            cause: TypedDbusError::from(err),
        })
    }

    pub fn get_firmware_version(&self) -> Result<String, Error> {
        // TODO: Don't reread the same info twice
        self.read_attr(
            &self.firmware,
            "Firmware info",
            Self::FIRMWARE_CHARACTERISTIC_UUID,
            |v| {
                String::from_utf8(v.get_ref()[2..7].to_vec())
                    .map_err(|_| panic!("Cannot convert to utf8"))
            },
        )
    }

    pub fn get_battery_pct(&self) -> Result<u8, Error> {
        // TODO: Don't reread the same info twice
        self.read_attr(
            &self.firmware,
            "Firmware info",
            Self::FIRMWARE_CHARACTERISTIC_UUID,
            |v| Ok(v.get_ref()[0]),
        )
    }

    pub fn get_realtime_reading(&self) -> Result<RealtimeReadings, Error> {
        self.set_device_mode(MifloraDeviceMode::Realtime)?;

        self.read_attr(
            &self.device_data,
            "Device Realtime readout",
            Self::DEVICE_DATA_CHARACTERISTIC_UUID,
            |v| self.decode_realtime_data(&mut v.clone()),
        )
    }

    pub fn blink(&self) -> Result<(), Error> {
        self.set_device_mode(MifloraDeviceMode::Blink)?;
        thread::sleep(Duration::from_millis(1000));

        Ok(())
    }

    pub fn get_device_time(&self) -> Result<u32, Error> {
        self.read_attr(
            &self.device_time,
            "Device time",
            Self::DEVICE_TIME_CHARACTERISTIC_UUID,
            |mut v| v.read_u32::<LittleEndian>(),
        )
    }

    pub fn get_history_record_count(&self) -> Result<u16, Error> {
        debug!("get_connected: {:?}", self.device.get_connected());
        debug!(
            "get_services_resolved: {:?}",
            self.device.get_services_resolved()
        );

        self.set_device_history_mode(MifloraDeviceHistoryMode::Init)?;

        // byte 0-1 is history record count
        self.read_attr(
            &self.history_data,
            "history date",
            Self::HISTORY_DATA_CHARACTERISTIC_UUID,
            |mut v| v.read_u16::<LittleEndian>(),
        )
    }

    pub fn get_history_records(&self, from: u16, to: u16) -> Result<Vec<HistoryReadings>, Error> {
        let mut result = Vec::new();

        self.set_device_history_mode(MifloraDeviceHistoryMode::Init)?;

        for idx in from..to {
            let record = self.read_history_record(idx)?;

            if let Some(mut record) = record {
                record.record_number = idx;

                result.push(record);
            }
        }

        Ok(result)
    }

    pub fn clear_history(&self) -> Result<(), Error> {
        self.set_device_history_mode(MifloraDeviceHistoryMode::Init)?;
        self.set_device_history_mode(MifloraDeviceHistoryMode::Clear)?;

        Ok(())
    }

    fn read_history_record(&self, idx: u16) -> Result<Option<HistoryReadings>, Error> {
        debug!("Reading history record #{:?}", idx);

        self.set_device_history_mode(MifloraDeviceHistoryMode::ReadRecord(idx))?;

        self.read_attr(
            &self.history_data,
            "history record",
            Self::HISTORY_DATA_CHARACTERISTIC_UUID,
            |v| self.decode_history_data(&mut v.clone()),
        )
    }

    fn decode_history_data(
        &self,
        data: &mut Cursor<Vec<u8>>,
    ) -> Result<Option<HistoryReadings>, std::io::Error> {
        debug!("read: {:?}", data);

        // Lets check if its invalid data
        {
            let mut raw: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            data.clone().read(&mut raw)?;

            if raw == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
                debug!("Zero record, ignoring");

                return Ok(None);
            }
            if raw == [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255] {
                debug!("0xFF record, ignoring");

                return Ok(None);
            }
        }


        // byte 0-3
        let history_epoch_time = data.read_u32::<LittleEndian>()?;

        // byte 4-5 temperature in 0.1 degree celcius
        let temperature = data.read_u16::<LittleEndian>()? as f32 * 0.1;

        // byte 6 - unknown
        let _unknown = data.read_u8()?;

        // byte 7-9 brightness in lux
        let lux1: u32 = data.read_u8()?.into();
        let lux2: u32 = data.read_u8()?.into();
        let lux3: u32 = data.read_u8()?.into();
        let mut lux = lux3;
        lux = lux << 8;
        lux = lux + lux2;
        lux = lux << 8;
        lux = lux + lux1;
        lux = u32::from_le(lux);

        // byte 10 - unknown
        let _unknown2 = data.read_u8()?;

        // byte 11 - moisture in procent
        let moisture = data.read_u8()?;

        // byte 12-13 - conductivity µS/cm
        let conductivity = data.read_u16::<LittleEndian>()?;

        // byte 14-15 - unknown
        let _unknown3 = data.read_u16::<LittleEndian>()?;

        Ok(Some(HistoryReadings {
            record_number: 0, // will be populated later
            epoch: history_epoch_time,
            temperature,
            lux,
            moisture,
            conductivity,
        }))
    }

    fn decode_realtime_data(
        &self,
        data: &mut Cursor<Vec<u8>>,
    ) -> Result<RealtimeReadings, std::io::Error> {
        // byte 0-1
        let temperature = data.read_u16::<LittleEndian>()? as f32 * 0.1;

        // byte 2
        let _unknown = data.read_u8()?;

        // byte 3-6
        let lux = data.read_u32::<LittleEndian>()?;

        // byte 7
        let moisture = data.read_u8()?;

        // byte 8-9
        let conductivity = data.read_u16::<LittleEndian>()?;

        // byte 10-15 unknown

        Ok(RealtimeReadings {
            temperature,
            lux,
            moisture,
            conductivity,
        })
    }

    fn set_device_mode(&self, mode: MifloraDeviceMode) -> Result<(), Error> {
        let (value, name) = match mode {
            MifloraDeviceMode::Realtime => ([0xa0, 0x1f], "device mode -> realtime"),
            MifloraDeviceMode::Blink => ([0xfd, 0xff], "device mode -> blink"),
        };

        self.write_attr(
            &self.device_mode,
            value,
            name,
            Self::DEVICE_MODE_CHARACTERISTIC_UUID,
            |w| w.to_vec(),
        )?;

        Ok(())
    }

    fn set_device_history_mode(&self, mode: MifloraDeviceHistoryMode) -> Result<(), Error> {
        let (value, name) = match mode {
            MifloraDeviceHistoryMode::Init => ([0xa0, 0x00, 0x00], "history mode -> init".to_string()),
            MifloraDeviceHistoryMode::Clear => ([0xa2, 0x00, 0x00], "history mode -> clear".to_string()),
            MifloraDeviceHistoryMode::ReadRecord(idx) => {
                let mut cmd = [0; 3];

                cmd[0] = 0xa1;
                LittleEndian::write_u16(&mut cmd[1..3], idx);

                (cmd, format!("history mode -> read record {:?}", idx))
            }
        };

        self.write_attr(
            &self.history_mode,
            value,
            &name,
            Self::HISTORY_MODE_CHARACTERISTIC_UUID,
            |w| w.to_vec(),
        )?;

        Ok(())
    }



    fn read_attr<T, F>(
        &self,
        attr: &Option<DBusPath>,
        name: &str,
        uuid: &str,
        parser: F,
    ) -> Result<T, Error>
        where
            F: FnOnce(Cursor<Vec<u8>>) -> Result<T, std::io::Error>,
    {
        debug!("read_attr attr={:?} name={:?} uuid={:?}", attr.as_ref().map(|f| f.path.clone()), name, uuid);

        let mut result = Err(Error::ThisShouldNeverHappend);

        for retry in 0..2 {
            debug!(" - try {:?}", retry);

            if retry > 0 {
                debug!("   disconnect");
                self.device.disconnect().map_err(|error| {
                    Error::ErrorDisconnecting {
                        cause: TypedDbusError::from(error),
                    }
                })?;

                let duration = Duration::from_millis(retry * 30_000);
                debug!("   sleeping for {:} s", duration.as_secs());
                thread::sleep(duration);

                debug!("   connect");
                self.device.connect().map_err(|error| {
                    Error::ErrorConnecting {
                        cause: TypedDbusError::from(error),
                    }
                })?;

                thread::sleep(Duration::from_millis(200));
                debug!("   sleeping for {:} ms", 200);
            }

            result = attr.as_ref()
                .ok_or(Error::GATTAttributeNotFound {
                    name: name.to_string(),
                    uuid: uuid.to_string(),
                })
                .and_then(|c| {
                    OrgBluezGattCharacteristic1::read_value(c, HashMap::new()).map_err(|error| {
                        Error::ErrorReadingData {
                            name: name.to_string(),
                            uuid: uuid.to_string(),
                            path: c.path.to_string(),
                            cause: TypedDbusError::from(error),
                        }
                    })
                });

            match result {
                Ok(_) => {
                    debug!("   - read_attr success!");
                    break
                },
                Err(ref err) => {
                    debug!("   - read_attr errored with: {:?}", err);
                }
            }
        }

        result.map(Cursor::new)
            .and_then(|v| parser(v).map_err(|err| Error::InvalidData { cause: err }))

    }

    fn write_attr<T, F>(
        &self,
        attr: &Option<DBusPath>,
        value: T,
        name: &str,
        uuid: &str,
        writer: F,
    ) -> Result<(), Error>
    where
        F: FnOnce(T) -> Vec<u8>,
    {
        attr.as_ref()
            .ok_or(Error::GATTAttributeNotFound {
                name: name.to_string(),
                uuid: uuid.to_string(),
            })
            .and_then(|c| {
                OrgBluezGattCharacteristic1::write_value(c, writer(value), HashMap::new()).map_err(
                    |err| Error::ErrorWritingData {
                        name: name.to_string(),
                        uuid: uuid.to_string(),
                        path: c.path.to_string(),
                        cause: TypedDbusError::from(err),
                    },
                )
            })
    }

    fn find_gatt_attributes(&mut self) {
        let mut firmware: Option<DBusPath> = None;
        let mut device_mode: Option<DBusPath> = None;
        let mut device_data: Option<DBusPath> = None;
        let mut device_time: Option<DBusPath> = None;
        let mut history_mode: Option<DBusPath> = None;
        let mut history_data: Option<DBusPath> = None;

        let conn = self.device.conn.clone();

        self.manager.find_objects(
            |path, obj| {
                if let Some(props) = obj.get(BLUEZ_GATT_CHARACTERISTIC_INTERFACE.into()) {
                    props
                        .get("UUID")
                        .and_then(dbus::arg::Variant::as_str)
                        .and_then(|uuid| match uuid {
                            Self::FIRMWARE_CHARACTERISTIC_UUID => Some(&mut firmware),
                            Self::DEVICE_MODE_CHARACTERISTIC_UUID => Some(&mut device_mode),
                            Self::DEVICE_DATA_CHARACTERISTIC_UUID => Some(&mut device_data),
                            Self::DEVICE_TIME_CHARACTERISTIC_UUID => Some(&mut device_time),
                            Self::HISTORY_MODE_CHARACTERISTIC_UUID => Some(&mut history_mode),
                            Self::HISTORY_DATA_CHARACTERISTIC_UUID => Some(&mut history_data),
                            _ => None,
                        })
                        .map(|char| {
                            *char = Some(dbus::ConnPath {
                                conn: conn.clone(),
                                dest: dbus::BusName::from(BLUEZ_SERVICE),
                                path: path.clone().into_static(),
                                timeout: 30_000,
                            })
                        });
                }

                firmware.is_some()
                    && device_mode.is_some()
                    && device_data.is_some()
                    && device_time.is_some()
                    && history_mode.is_some()
                    && history_data.is_some()
            },
            Some(30_000),
        );

        debug!("firmware: {:?}", firmware);
        debug!("device_mode: {:?}", device_mode);
        debug!("device_data: {:?}", device_data);
        debug!("device_time: {:?}", device_time);
        debug!("history_mode: {:?}", history_mode);
        debug!("history_data: {:?}", history_data);

        self.firmware = firmware;
        self.device_mode = device_mode;
        self.device_data = device_data;
        self.device_time = device_time;
        self.history_mode = history_mode;
        self.history_data = history_data;
    }
}

impl<'a> Drop for Miflora<'a> {
    fn drop(&mut self) {
        self.disconnect().ok();
    }
}
