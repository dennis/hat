use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use std::{error, fmt};

use dbus::arg::RefArg;
use dbus::stdintf::org_freedesktop_dbus::{
    ObjectManager, ObjectManagerInterfacesAdded, ObjectManagerInterfacesRemoved,
};
use dbus::{BusType, Connection, SignalArgs};
use dbus_common::org_bluez_adapter1::OrgBluezAdapter1;

pub(crate) static BLUEZ_SERVICE: &'static str = "org.bluez";
pub(crate) static BLUEZ_INTERFACE_DEVICE1: &'static str = "org.bluez.Device1";
pub(crate) static BLUEZ_INTERFACE_ADAPTER1: &'static str = "org.bluez.Adapter1";

pub(crate) static BLUEZ_GATT_CHARACTERISTIC_INTERFACE: &'static str =
    "org.bluez.GattCharacteristic1";

type BoxErr = Box<dyn error::Error>;
pub(crate) type DBusPath = dbus::ConnPath<'static, Rc<dbus::Connection>>;

type DBusProperties = HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>;
type DBusObject = HashMap<String, DBusProperties>;

#[derive(Debug)]
pub struct TypedDbusError {
    pub cause: dbus::Error,
    pub kind: TypedDbusErrorKind,
}

#[derive(Debug)]
pub enum TypedDbusErrorKind {
    InvalidArgs,
    AccessDenied,
    NoReply,
    Failed,
    Other,
}

impl std::error::Error for TypedDbusError {}

impl Display for TypedDbusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "D-Bus {:?} error: {}", self.kind, self.cause)
    }
}

impl From<dbus::Error> for TypedDbusError {
    fn from(cause: dbus::Error) -> Self {
        let kind = if let Some(name) = cause.name() {
            match name {
                "org.freedesktop.DBus.Error.InvalidArgs" => TypedDbusErrorKind::InvalidArgs,
                "org.freedesktop.DBus.Error.AccessDenied" => TypedDbusErrorKind::AccessDenied,
                "org.freedesktop.DBus.Error.NoReply" => TypedDbusErrorKind::NoReply,
                "org.bluez.Error.Failed" => TypedDbusErrorKind::Failed,
                _ => TypedDbusErrorKind::Other,
            }
        } else {
            TypedDbusErrorKind::Other
        };

        TypedDbusError { cause, kind }
    }
}

#[derive(Debug)]
pub(crate) struct BluezManager {
    conn: Rc<dbus::Connection>,
    objects: HashMap<dbus::Path<'static>, DBusObject>,
    adapter: Option<DBusPath>,
}

impl BluezManager {
    pub fn new() -> Result<Self, TypedDbusError> {
        info!("Creating BluezManager");

        let conn = Rc::new(Connection::get_private(BusType::System)?);

        let bus_name = dbus::BusName::from(BLUEZ_SERVICE);
        let root_path = dbus::Path::from("/");

        let bluez = conn.with_path(BLUEZ_SERVICE, &root_path, 1000);
        let objects = bluez.get_managed_objects()?;

        conn.add_match(&ObjectManagerInterfacesAdded::match_str(
            Some(&bus_name),
            Some(&root_path),
        ))?;
        conn.add_match(&ObjectManagerInterfacesRemoved::match_str(
            Some(&bus_name),
            Some(&root_path),
        ))?;

        let adapter = None;

        Ok(BluezManager {
            conn,
            objects,
            adapter,
        })
    }

    pub fn start_discovery(&mut self, timeout_ms: Option<u32>) -> Result<(), BoxErr> {
        let adapter = self.find_adapter(timeout_ms)?;

        debug!("Discoverying using {:?}", adapter);

        adapter.start_discovery()?;

        self.adapter = Some(adapter);

        Ok(())
    }

    pub fn find_objects<F: FnMut(&dbus::Path, &DBusObject) -> bool>(
        &mut self,
        mut f: F,
        timeout_ms: Option<u32>,
    ) {
        self.find_object(
            |path, obj| if f(path, obj) { Some(()) } else { None },
            timeout_ms,
        );
    }

    fn find_object<T, F: FnMut(&dbus::Path, &DBusObject) -> Option<T>>(
        &mut self,
        mut f: F,
        timeout_ms: Option<u32>,
    ) -> Option<T> {
        let r = self
            .objects
            .iter()
            .find_map(|(path, obj)| f(path, obj))
            .or_else(|| {
                (dbus::ConnMsgs {
                    conn: self.conn.clone(),
                    timeout_ms,
                })
                .find_map(|msg| {
                    let mut t = None;
                    self.process_interface_signal(&msg, |path, obj| {
                        t = f(path, obj);
                    });
                    t
                })
            });

        (dbus::ConnMsgs {
            conn: self.conn.clone(),
            timeout_ms: None,
        })
        .for_each(|msg| self.process_interface_signal(&msg, |_, _| ()));

        r
    }

    fn process_interface_signal<F: FnOnce(&dbus::Path, &DBusObject)>(
        &mut self,
        msg: &dbus::Message,
        f: F,
    ) {
        if let Some(ObjectManagerInterfacesAdded {
            object: path,
            interfaces,
        }) = ObjectManagerInterfacesAdded::from_message(&msg)
        {
            let all_interfaces = self.objects.entry(path.clone()).or_default();
            all_interfaces.extend(interfaces);

            f(&path, &all_interfaces);
        } else if let Some(ObjectManagerInterfacesRemoved {
            object: path,
            interfaces,
        }) = ObjectManagerInterfacesRemoved::from_message(&msg)
        {
            match self.objects.entry(path) {
                Entry::Occupied(mut e) => {
                    let obj = e.get_mut();
                    interfaces.iter().for_each(|i| {
                        obj.remove(i);
                    });
                    if obj.is_empty() {
                        e.remove();
                    }
                }
                _ => (),
            };
        }
    }

    pub fn scan(
        &mut self,
        required_uuid: &str,
        timeout_ms: Option<u32>,
    ) -> Result<Vec<DBusPath>, BoxErr> {
        let mut result: Vec<DBusPath> = Vec::new();
        let conn = self.conn.clone();

        self.find_objects(
            |path, obj| {
                obj.get(BLUEZ_INTERFACE_DEVICE1)
                    .and_then(|props| props.get("UUIDs"))
                    .filter(|uuids| {
                        let mut count = 0;

                        uuids.0.as_iter().as_mut().map(|f| {
                            f.for_each(|f| {
                                if f.as_str() == Some(required_uuid) {
                                    count += 1;
                                }
                            });
                        });

                        count > 0
                    })
                    .map(|_| {
                        result.push(DBusPath {
                            conn: conn.clone(),
                            dest: dbus::BusName::from(BLUEZ_SERVICE),
                            path: path.clone().into_static(),
                            timeout: 30_000,
                        });
                    });

                false
            },
            timeout_ms,
        );

        Ok(result)
    }

    pub fn find_by_address(
        &mut self,
        hw_addr: &str,
        timeout_ms: Option<u32>,
    ) -> Result<DBusPath, BoxErr> {
        let conn = self.conn.clone();
        self.find_object(
            |path, obj| {
                obj.get(BLUEZ_INTERFACE_DEVICE1)
                    .and_then(|props| props.get("Address"))
                    .and_then(dbus::arg::Variant::as_str)
                    .and_then(|a| if a == hw_addr { Some(()) } else { None })
                    .map(|_| DBusPath {
                        conn: conn.clone(),
                        dest: dbus::BusName::from(BLUEZ_SERVICE),
                        path: path.clone().into_static(),
                        timeout: 60_000,
                    })
            },
            timeout_ms,
        )
        .ok_or(Box::new(Error::new(ErrorKind::Other, "Device not found")))
    }

    fn find_adapter(&mut self, timeout_ms: Option<u32>) -> Result<DBusPath, BoxErr> {
        let conn = self.conn.clone();

        self.find_object(
            |path, obj| {
                obj.get(BLUEZ_INTERFACE_ADAPTER1).map(|_| DBusPath {
                    conn: conn.clone(),
                    dest: dbus::BusName::from(BLUEZ_SERVICE),
                    path: path.clone().into_static(),
                    timeout: 60_000,
                })
            },
            timeout_ms,
        )
        .ok_or(Box::new(Error::new(ErrorKind::Other, "Adapter not found")))
    }
}

impl Drop for BluezManager {
    fn drop(&mut self) {
        if let Some(ref adapter) = self.adapter {
            adapter.stop_discovery().ok();
        }
    }
}
