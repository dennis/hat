static ROOT_SERVICE_UUID: &'static str = "0000fe95-0000-1000-8000-00805f9b34fb";

use std::error::Error;

use dbus::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::{BusType, Connection, SignalArgs};
use dbus_common::dbus_processor::DbusProcessor;

use crate::Cli;
use crate::Miflora;

pub struct Scanner<'a> {
    cli : &'a Cli,
    connection: Connection,
}

impl<'a> Scanner<'a> {
    pub fn new(cli : &'a Cli) -> Result<Scanner, Box<Error>> {
        let connection = Connection::get_private(BusType::System)?;

        Ok(Scanner { cli, connection })
    }

    pub fn find_mifloras(&self) -> Result<Vec<Miflora>, Box<Error>> {
        self.connection.add_match(&PropertiesPropertiesChanged::match_str(None, None))?;

        let mut processor = DbusProcessor { root_service_uuid: ROOT_SERVICE_UUID.to_string(), debug: self.cli.debug };
        let devices = processor
            .process_known_devices(&self.connection)?
            .iter()
            .map( |device| Miflora::new(device.clone()))
            .collect();

        Ok(devices)
    }
}
