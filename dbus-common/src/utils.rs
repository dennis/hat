use dbus::{Connection, Message, MessageItem};

use std::error::Error;

pub static SERVICE_NAME: &'static str = "org.bluez";
pub static ADAPTER_INTERFACE: &'static str = "org.bluez.Adapter1";
pub static DEVICE_INTERFACE: &'static str = "org.bluez.Device1";

fn get_managed_objects(service_name : &str, connection : &Connection) -> Result<Vec<MessageItem>, Box<Error>> {
    let m = Message::new_method_call(
        service_name,
        "/",
        "org.freedesktop.DBus.ObjectManager",
        "GetManagedObjects",
    )?;

    let r = connection.send_with_reply_and_block(m, 1000)?;

    Ok(r.get_items())
}

pub fn get_managed_objects_with_interface(connection : &Connection, requested_interface : &String) -> Result<Vec<String>, Box<Error>> {
    let mut adapters: Vec<String> = Vec::new();
    let objects: Vec<MessageItem> = get_managed_objects(SERVICE_NAME, connection)?;
    let z: &[MessageItem] = objects.get(0).unwrap().inner().unwrap();

    for y in z {
        let (path, interfaces) = y.inner().unwrap();
        let x: &[MessageItem] = interfaces.inner().unwrap();
        for interface in x {
            let (i, _) = interface.inner().unwrap();
            let name: &str = i.inner().unwrap();
            if name == requested_interface {
                let p: &str = path.inner().unwrap();
                adapters.push(String::from(p));
            }
        }
    }
    Ok(adapters)
}

fn get_adapters(connection : &Connection) -> Result<Vec<String>, Box<Error>> {
    get_managed_objects_with_interface(connection, &ADAPTER_INTERFACE.to_string())
}

pub fn get_adapter(connection : &Connection) -> Result<String, Box<Error>> {
    let adapters = get_adapters(connection)?;

    if adapters.is_empty() {
        return Err(Box::from("Bluetooth adapter not found"));
    }

    Ok(adapters[0].clone())
}
