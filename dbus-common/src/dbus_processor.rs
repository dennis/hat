use std::error::Error;

use crate::org_bluez_device1::OrgFreedesktopDBusProperties;
use crate::utils::{SERVICE_NAME, DEVICE_INTERFACE, get_managed_objects_with_interface};

use dbus::Connection;
use dbus::arg::RefArg;

pub struct DbusProcessor {
    pub root_service_uuid: String,
    pub debug: bool,
}

impl DbusProcessor {
    // This will return known devices (no scanning)
    pub fn process_known_devices(&mut self, connection : &Connection) -> Result<Vec<String>, Box<dyn Error>> {
        if self.debug { eprintln!("Scanning for known devices"); }

        let r : Vec<String> =
            get_managed_objects_with_interface(connection, &DEVICE_INTERFACE, "", "")?
                .iter()
                .filter(|&device_path| self.probe_device(connection, device_path).unwrap_or(false) )
                .map(|device_path| device_path.clone())
                .collect();

        Ok(r)
    }

    fn probe_device(&mut self, connection: &Connection, path: &String) -> Result<bool, Box<dyn Error>> {
        let device = connection.with_path(SERVICE_NAME, path, 1000);
        let properties = device.get_all(DEVICE_INTERFACE)?;

        let btaddr = if let Some(btaddr) = properties.get("Address") { btaddr } else { return Ok(false) };
        let name   = if let Some(name)   = properties.get("Name")    { name }   else { return Ok(false) };
        let uuids  = if let Some(uuids)  = properties.get("UUIDs")   { uuids }  else { return Ok(false) };

        if self.debug {
            eprintln!("  found device:");
            eprintln!("    path   {:?}", path);
            eprintln!("    btaddr {:?}", btaddr);
            eprintln!("    name   {:?}", name);
            eprintln!("    uuids  {:?}", uuids);
        }

        let iter = uuids.as_iter();

        if iter.is_none() {
            return Ok(false);
        }

        let mut iter = iter.unwrap();
        if iter.any(|a| a.as_str() == Some(&self.root_service_uuid)) {
            if self.debug { eprintln!("    this looks applicable"); }
            return Ok(true);
        }

        if self.debug { eprintln!("    ignoring"); }
        Ok(false)
    }
}
