use dbus::arg::RefArg;
use dbus::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::MessageType::Signal;
use dbus::{BusType, Connection, ConnectionItem, Message, MessageItem, SignalArgs};
use std::boxed::Box;
use std::error::Error;
use std::time::SystemTime;
use std::time::Duration;

use crate::cli::Cli;
use crate::org_bluez_adapter1::OrgBluezAdapter1;
use crate::org_bluez_device1::OrgFreedesktopDBusProperties;
use crate::weight_data::WeightData;

static ADAPTER_INTERFACE: &'static str = "org.bluez.Adapter1";
static DEVICE_INTERFACE: &'static str = "org.bluez.Device1";
static SERVICE_NAME: &'static str = "org.bluez";
static BODY_COMPOSITION_UUID: &'static str = "0000181b-0000-1000-8000-00805f9b34fb";

pub struct Scanner<'a> {
    connection: Connection,
    cli: &'a Cli,
}

impl<'a> Scanner<'a> {
    pub fn new(cli: &'a Cli) -> Result<Scanner, Box<Error>> {
        let connection = Connection::get_private(BusType::System)?;

        Ok(Scanner { connection, cli })
    }

    pub fn listen_for_signals(&self) -> Result<(), Box<Error>> {
        self.connection
            .add_match(&PropertiesPropertiesChanged::match_str(None, None))?;

        let now = SystemTime::now();
        let adapter = self
            .connection
            .with_path(SERVICE_NAME, self.get_adapter()?, 1000);

        let mut last_weight_data_seen = SystemTime::now();
        let mut last_weight_data : Option<WeightData> = None;

        adapter.start_discovery()?;

        for n in self.connection.iter(1000) {
            match n {
                ConnectionItem::Signal(signal) => {
                    match self.handle_signal(&signal)? {
                        Some(weight_data) => {
                            if self.cli.debug { eprintln!("  got data, debouncing it"); }
                            if weight_data.weight.is_none() {
                                if self.cli.debug { eprintln!("  empty reading, ignoring"); }
                            }
                            else {
                                last_weight_data = Some(weight_data);
                                last_weight_data_seen = SystemTime::now();
                            }
                        },
                        None => {}
                    }
                }
                _ => (),
            }

            if let Some(weight_data) = &last_weight_data {
                if weight_data.done() || last_weight_data_seen.elapsed()? > Duration::new(30,0) {
                    if self.cli.debug {
                        eprintln!("  outputing weight data");
                    }

                    weight_data.dump()?;
                    last_weight_data = None;

                    if self.cli.until_data {
                        if self.cli.debug {
                            eprintln!("  stopping as requested via params");
                        }
                        break;
                    }
                }
            }

            if self.cli.duration > 0 {
                let elapsed = now.elapsed()?;

                if elapsed.as_secs() > self.cli.duration {
                    if self.cli.debug {
                        eprintln!("  stopping as exceeding max duration defined via params");
                    }
                    break;
                }
            }
        }

        adapter.stop_discovery()?;

        Ok(())
    }

    fn get_adapter(&self) -> Result<String, Box<Error>> {
        let adapters = self.get_adapters()?;

        if adapters.is_empty() {
            return Err(Box::from("Bluetooth adapter not found"));
        }

        Ok(adapters[0].clone())
    }

    fn get_adapters(&self) -> Result<Vec<String>, Box<Error>> {
        let mut adapters: Vec<String> = Vec::new();
        let objects: Vec<MessageItem> = self.get_managed_objects()?;
        let z: &[MessageItem] = objects.get(0).unwrap().inner().unwrap();

        for y in z {
            let (path, interfaces) = y.inner().unwrap();
            let x: &[MessageItem] = interfaces.inner().unwrap();
            for interface in x {
                let (i, _) = interface.inner().unwrap();
                let name: &str = i.inner().unwrap();
                if name == ADAPTER_INTERFACE {
                    let p: &str = path.inner().unwrap();
                    adapters.push(String::from(p));
                }
            }
        }
        Ok(adapters)
    }

    fn handle_signal(&self, signal: &dbus::Message) -> Result<Option<WeightData>, Box<Error>> {
        let (message_type, path, interface, member) = signal.headers();

        if message_type == Signal
            && interface == Some("org.freedesktop.DBus.Properties".to_string())
            && member == Some("PropertiesChanged".to_string())
        {
            let items = signal.get_items();

            if items[0] == dbus::MessageItem::Str(DEVICE_INTERFACE.to_string()) && path.is_some()
            {
                if let dbus::MessageItem::Array(e) = &items[1] {
                    return self.inquiry_changed_properties(&path.unwrap(), e.to_vec());
                }
            }
        }
        Ok(None)
    }

    fn get_managed_objects(&self) -> Result<Vec<MessageItem>, Box<Error>> {
        let m = Message::new_method_call(
            SERVICE_NAME,
            "/",
            "org.freedesktop.DBus.ObjectManager",
            "GetManagedObjects",
        )?;

        let r = self.connection.send_with_reply_and_block(m, 1000)?;

        Ok(r.get_items())
    }

    fn inquiry_changed_properties(
        &self,
        path: &String,
        item_vec: Vec<dbus::MessageItem>,
    ) -> Result<Option<WeightData>, Box<Error>> {
        let device = self.connection.with_path(SERVICE_NAME, path, 1000);
        let properties = device.get_all(DEVICE_INTERFACE)?;

        let btaddr = if let Some(btaddr) = properties.get("Address") { btaddr } else { return Ok(None) };
        let name   = if let Some(name)   = properties.get("Name")    { name }   else { return Ok(None) };
        let uuids  = if let Some(uuids)  = properties.get("UUIDs")   { uuids }  else { return Ok(None) };

        if self.cli.debug {
            eprintln!("changed properties:");
            eprintln!("  btaddr {:?}", btaddr);
            eprintln!("  name   {:?}", name);
            eprintln!("  uuids  {:?}", uuids);
        }

        if let dbus::arg::Variant(uuids) = uuids {
            let iter = (*uuids).as_iter();

            if iter.is_none() {
                return Ok(None);
            }

            let mut iter = iter.unwrap();

            if !iter.any(|a| a.as_str() == Some(BODY_COMPOSITION_UUID)) {
                if self.cli.debug { eprintln!("  discarding due to missing uuid"); }

                return Ok(None);
            }
        }
        else {
            if self.cli.debug { eprintln!("  discarding - wrong type"); }

            return Ok(None);
        }

        if self.cli.debug {
            eprintln!("  found correct UUID");
            eprintln!("  item changed:");
        }

        for item in item_vec {
            if self.cli.debug {
                eprintln!("    {:?}", item);
            }

            if let dbus::MessageItem::DictEntry(key, value) = item {
                if let Some(btaddr_str) = (*btaddr).as_str() {
                    return self.inquiry_service_data(&key, &value, btaddr_str);
                }
            }
        }

        Ok(None)
    }

    fn inquiry_service_data_values(
        &self,
        value: &dbus::MessageItem,
        btaddr: &str,
    ) -> Result<Option<WeightData>, Box<Error>> {
        if let dbus::MessageItem::Array(value) = value {
            for v in value.as_ref() {
                if let dbus::MessageItem::DictEntry(key, value) = v {
                    return self.extract_body_composition(&key, &value, btaddr);
                }
            }
        }

        Ok(None)
    }

    fn inquiry_service_data(
        &self,
        key: &dbus::MessageItem,
        value: &dbus::MessageItem,
        btaddr: &str,
    ) -> Result<Option<WeightData>, Box<Error>> {
        if let dbus::MessageItem::Str(key) = key {
            if key == "ServiceData" {
                if self.cli.debug {
                    eprintln!("  Got service data!");
                }

                if let dbus::MessageItem::Variant(value) = value {
                    return self.inquiry_service_data_values(value, btaddr);
                }
            }
        }

        Ok(None)
    }

    fn extract_body_composition(
        &self,
        key: &dbus::MessageItem,
        value: &dbus::MessageItem,
        btaddr: &str,
    ) -> Result<Option<WeightData>, Box<Error>> {
        if let dbus::MessageItem::Str(key) = key {
            if self.cli.debug {
                eprintln!("  service-data uuid : {:?}", key);
            }

            if key != BODY_COMPOSITION_UUID {
                return Ok(None);
            }

            if let dbus::MessageItem::Variant(value) = value {
                let value: &dbus::MessageItem = value;

                if let dbus::MessageItem::Array(value) = value {
                    let bytes = value
                        .as_ref()
                        .to_vec()
                        .into_iter()
                        .filter_map(|x| x.inner::<u8>().ok())
                        .collect();

                    return Ok(Some(WeightData::decode(&bytes, btaddr, self.cli.debug)?));
                }
            }
        }

        Ok(None)
    }
}
