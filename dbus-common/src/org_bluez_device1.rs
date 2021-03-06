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

pub trait OrgBluezDevice1 {
    type Err;
    fn disconnect(&self) -> Result<(), Self::Err>;
    fn connect(&self) -> Result<(), Self::Err>;
    fn connect_profile(&self, uuid: &str) -> Result<(), Self::Err>;
    fn disconnect_profile(&self, uuid: &str) -> Result<(), Self::Err>;
    fn pair(&self) -> Result<(), Self::Err>;
    fn cancel_pairing(&self) -> Result<(), Self::Err>;
    fn get_address(&self) -> Result<String, Self::Err>;
    fn get_address_type(&self) -> Result<String, Self::Err>;
    fn get_name(&self) -> Result<String, Self::Err>;
    fn get_alias(&self) -> Result<String, Self::Err>;
    fn set_alias(&self, value: String) -> Result<(), Self::Err>;
    fn get_class(&self) -> Result<u32, Self::Err>;
    fn get_appearance(&self) -> Result<u16, Self::Err>;
    fn get_icon(&self) -> Result<String, Self::Err>;
    fn get_paired(&self) -> Result<bool, Self::Err>;
    fn get_trusted(&self) -> Result<bool, Self::Err>;
    fn set_trusted(&self, value: bool) -> Result<(), Self::Err>;
    fn get_blocked(&self) -> Result<bool, Self::Err>;
    fn set_blocked(&self, value: bool) -> Result<(), Self::Err>;
    fn get_legacy_pairing(&self) -> Result<bool, Self::Err>;
    fn get_rssi(&self) -> Result<i16, Self::Err>;
    fn get_connected(&self) -> Result<bool, Self::Err>;
    fn get_uuids(&self) -> Result<Vec<String>, Self::Err>;
    fn get_modalias(&self) -> Result<String, Self::Err>;
    fn get_adapter(&self) -> Result<dbus::Path<'static>, Self::Err>;
    fn get_manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err>;
    fn get_service_data(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err>;
    fn get_tx_power(&self) -> Result<i16, Self::Err>;
    fn get_services_resolved(&self) -> Result<bool, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezDevice1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn disconnect(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"Disconnect".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn connect(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"Connect".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn connect_profile(&self, uuid: &str) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"ConnectProfile".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(uuid);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn disconnect_profile(&self, uuid: &str) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"DisconnectProfile".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(uuid);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn pair(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"Pair".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn cancel_pairing(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Device1".into(), &"CancelPairing".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_address(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Address")
    }

    fn get_address_type(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "AddressType")
    }

    fn get_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Name")
    }

    fn get_alias(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Alias")
    }

    fn get_class(&self) -> Result<u32, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Class")
    }

    fn get_appearance(&self) -> Result<u16, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Appearance")
    }

    fn get_icon(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Icon")
    }

    fn get_paired(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Paired")
    }

    fn get_trusted(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Trusted")
    }

    fn get_blocked(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Blocked")
    }

    fn get_legacy_pairing(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "LegacyPairing")
    }

    fn get_rssi(&self) -> Result<i16, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "RSSI")
    }

    fn get_connected(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Connected")
    }

    fn get_uuids(&self) -> Result<Vec<String>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "UUIDs")
    }

    fn get_modalias(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Modalias")
    }

    fn get_adapter(&self) -> Result<dbus::Path<'static>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Adapter")
    }

    fn get_manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ManufacturerData")
    }

    fn get_service_data(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServiceData")
    }

    fn get_tx_power(&self) -> Result<i16, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "TxPower")
    }

    fn get_services_resolved(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServicesResolved")
    }

    fn set_alias(&self, value: String) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Alias", value)
    }

    fn set_trusted(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Trusted", value)
    }

    fn set_blocked(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Blocked", value)
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
