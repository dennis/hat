// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus as dbus;
use dbus::arg;

pub trait OrgFreedesktopDBusIntrospectable {
    type Err;
    fn introspect(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusIntrospectable for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn introspect(&self) -> Result<String, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let xml: String = i.read()?;
        Ok(xml)
    }
}

pub trait OrgBluezGattDescriptor1 {
    type Err;
    fn read_value(&self, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<Vec<u8>, Self::Err>;
    fn write_value(&self, value: Vec<u8>, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), Self::Err>;
    fn get_uuid(&self) -> Result<String, Self::Err>;
    fn get_characteristic(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn get_value(&self) -> Result<Vec<u8>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezGattDescriptor1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn read_value(&self, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<Vec<u8>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.GattDescriptor1".into(), &"ReadValue".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(options);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let value: Vec<u8> = i.read()?;
        Ok(value)
    }

    fn write_value(&self, value: Vec<u8>, options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.GattDescriptor1".into(), &"WriteValue".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(value);
            i.append(options);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_uuid(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.GattDescriptor1", "UUID")
    }

    fn get_characteristic(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.GattDescriptor1", "Characteristic")
    }

    fn get_value(&self) -> Result<Vec<u8>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.GattDescriptor1", "Value")
    }
}

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get<R0: for<'b> arg::Get<'b>>(&self, interface: &str, name: &str) -> Result<arg::Variant<R0>, Self::Err>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, name: &str, value: arg::Variant<I2>) -> Result<(), Self::Err>;
    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgFreedesktopDBusProperties for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn get<R0: for<'b> arg::Get<'b>>(&self, interface: &str, name: &str) -> Result<arg::Variant<R0>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Get".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface);
            i.append(name);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let value: arg::Variant<R0> = i.read()?;
        Ok(value)
    }

    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, name: &str, value: arg::Variant<I2>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"Set".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface);
            i.append(name);
            i.append(value);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>> = i.read()?;
        Ok(properties)
    }
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl dbus::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface = i.read()?;
        self.changed_properties = i.read()?;
        self.invalidated_properties = i.read()?;
        Ok(())
    }
}
