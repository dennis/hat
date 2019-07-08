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
        let mut m = (self.method_call_with_args(&"org.freedesktop.DBus.Introspectable".into(), &"Introspect".into(), |_| {
        })?);
        (m.as_result())?;
        let mut i = m.iter_init();
        let xml: String = (i.read())?;
        Ok(xml)
    }
}

pub trait OrgBluezAdapter1 {
    type Err;
    fn start_discovery(&self) -> Result<(), Self::Err>;
    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn stop_discovery(&self) -> Result<(), Self::Err>;
    fn remove_device(&self, device: dbus::Path) -> Result<(), Self::Err>;
    fn get_discovery_filters(&self) -> Result<Vec<String>, Self::Err>;
    fn get_address(&self) -> Result<String, Self::Err>;
    fn get_address_type(&self) -> Result<String, Self::Err>;
    fn get_name(&self) -> Result<String, Self::Err>;
    fn get_alias(&self) -> Result<String, Self::Err>;
    fn set_alias(&self, value: String) -> Result<(), Self::Err>;
    fn get_class(&self) -> Result<u32, Self::Err>;
    fn get_powered(&self) -> Result<bool, Self::Err>;
    fn set_powered(&self, value: bool) -> Result<(), Self::Err>;
    fn get_discoverable(&self) -> Result<bool, Self::Err>;
    fn set_discoverable(&self, value: bool) -> Result<(), Self::Err>;
    fn get_discoverable_timeout(&self) -> Result<u32, Self::Err>;
    fn set_discoverable_timeout(&self, value: u32) -> Result<(), Self::Err>;
    fn get_pairable(&self) -> Result<bool, Self::Err>;
    fn set_pairable(&self, value: bool) -> Result<(), Self::Err>;
    fn get_pairable_timeout(&self) -> Result<u32, Self::Err>;
    fn set_pairable_timeout(&self, value: u32) -> Result<(), Self::Err>;
    fn get_discovering(&self) -> Result<bool, Self::Err>;
    fn get_uuids(&self) -> Result<Vec<String>, Self::Err>;
    fn get_modalias(&self) -> Result<String, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezAdapter1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn start_discovery(&self) -> Result<(), Self::Err> {
        let mut m = (self.method_call_with_args(&"org.bluez.Adapter1".into(), &"StartDiscovery".into(), |_| {
        }))?;
        (m.as_result())?;
        Ok(())
    }

    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = (self.method_call_with_args(&"org.bluez.Adapter1".into(), &"SetDiscoveryFilter".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(properties);
        }))?;
        m.as_result()?;
        Ok(())
    }

    fn stop_discovery(&self) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Adapter1".into(), &"StopDiscovery".into(), |_| {
        })?;
        m.as_result()?;
        Ok(())
    }

    fn remove_device(&self, device: dbus::Path) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Adapter1".into(), &"RemoveDevice".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(device);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_discovery_filters(&self) -> Result<Vec<String>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Adapter1".into(), &"GetDiscoveryFilters".into(), |_| {
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let filters: Vec<String> = i.read()?;
        Ok(filters)
    }

    fn get_address(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Address")
    }

    fn get_address_type(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "AddressType")
    }

    fn get_name(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Name")
    }

    fn get_alias(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Alias")
    }

    fn get_class(&self) -> Result<u32, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Class")
    }

    fn get_powered(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Powered")
    }

    fn get_discoverable(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discoverable")
    }

    fn get_discoverable_timeout(&self) -> Result<u32, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "DiscoverableTimeout")
    }

    fn get_pairable(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Pairable")
    }

    fn get_pairable_timeout(&self) -> Result<u32, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "PairableTimeout")
    }

    fn get_discovering(&self) -> Result<bool, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discovering")
    }

    fn get_uuids(&self) -> Result<Vec<String>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "UUIDs")
    }

    fn get_modalias(&self) -> Result<String, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Modalias")
    }

    fn set_alias(&self, value: String) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Alias", value)
    }

    fn set_powered(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Powered", value)
    }

    fn set_discoverable(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Discoverable", value)
    }

    fn set_discoverable_timeout(&self, value: u32) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "DiscoverableTimeout", value)
    }

    fn set_pairable(&self, value: bool) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Pairable", value)
    }

    fn set_pairable_timeout(&self, value: u32) -> Result<(), Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "PairableTimeout", value)
    }
}

pub trait OrgFreedesktopDBusProperties {
    type Err;
    fn get<R0: for<'b> arg::Get<'b>>(&self, interface: &str, name: &str) -> Result<arg::Variant<R0>, Self::Err>;
    fn set<I2: arg::Arg + arg::Append>(&self, interface: &str, name: &str, value: arg::Variant<I2>) -> Result<(), Self::Err>;
    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err>;
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

    fn get_all(&self, interface: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>, Self::Err> {
        let mut m = self.method_call_with_args(&"org.freedesktop.DBus.Properties".into(), &"GetAll".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(interface);
        })?;
        m.as_result()?;
        let mut i = m.iter_init();
        let properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>> = i.read()?;
        Ok(properties)
    }
}

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg>>>,
    pub invalidated_properties: Vec<String>,
}

impl dbus::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        (&self.interface as &arg::RefArg).append(i);
        (&self.changed_properties as &arg::RefArg).append(i);
        (&self.invalidated_properties as &arg::RefArg).append(i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface = i.read()?;
        self.changed_properties = i.read()?;
        self.invalidated_properties = i.read()?;
        Ok(())
    }
}

pub trait OrgBluezGattManager1 {
    type Err;
    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn unregister_application(&self, application: dbus::Path) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezGattManager1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn register_application(&self, application: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.GattManager1".into(), &"RegisterApplication".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(application);
            i.append(options);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn unregister_application(&self, application: dbus::Path) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.GattManager1".into(), &"UnregisterApplication".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(application);
        })?;
        m.as_result()?;
        Ok(())
    }
}

pub trait OrgBluezLEAdvertisingManager1 {
    type Err;
    fn register_advertisement(&self, advertisement: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), Self::Err>;
    fn get_active_instances(&self) -> Result<u8, Self::Err>;
    fn get_supported_instances(&self) -> Result<u8, Self::Err>;
    fn get_supported_includes(&self) -> Result<Vec<String>, Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezLEAdvertisingManager1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn register_advertisement(&self, advertisement: dbus::Path, options: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.LEAdvertisingManager1".into(), &"RegisterAdvertisement".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(advertisement);
            i.append(options);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn unregister_advertisement(&self, service: dbus::Path) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.LEAdvertisingManager1".into(), &"UnregisterAdvertisement".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(service);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn get_active_instances(&self) -> Result<u8, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "ActiveInstances")
    }

    fn get_supported_instances(&self) -> Result<u8, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "SupportedInstances")
    }

    fn get_supported_includes(&self) -> Result<Vec<String>, Self::Err> {
        <Self as dbus::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.LEAdvertisingManager1", "SupportedIncludes")
    }
}

pub trait OrgBluezMedia1 {
    type Err;
    fn register_endpoint(&self, endpoint: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), Self::Err>;
    fn register_player(&self, player: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err>;
    fn unregister_player(&self, player: dbus::Path) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezMedia1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn register_endpoint(&self, endpoint: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Media1".into(), &"RegisterEndpoint".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(endpoint);
            i.append(properties);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn unregister_endpoint(&self, endpoint: dbus::Path) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Media1".into(), &"UnregisterEndpoint".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(endpoint);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn register_player(&self, player: dbus::Path, properties: ::std::collections::HashMap<&str, arg::Variant<Box<arg::RefArg>>>) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Media1".into(), &"RegisterPlayer".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(player);
            i.append(properties);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn unregister_player(&self, player: dbus::Path) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.Media1".into(), &"UnregisterPlayer".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(player);
        })?;
        m.as_result()?;
        Ok(())
    }
}

pub trait OrgBluezNetworkServer1 {
    type Err;
    fn register(&self, uuid: &str, bridge: &str) -> Result<(), Self::Err>;
    fn unregister(&self, uuid: &str) -> Result<(), Self::Err>;
}

impl<'a, C: ::std::ops::Deref<Target=dbus::Connection>> OrgBluezNetworkServer1 for dbus::ConnPath<'a, C> {
    type Err = dbus::Error;

    fn register(&self, uuid: &str, bridge: &str) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.NetworkServer1".into(), &"Register".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(uuid);
            i.append(bridge);
        })?;
        m.as_result()?;
        Ok(())
    }

    fn unregister(&self, uuid: &str) -> Result<(), Self::Err> {
        let mut m = self.method_call_with_args(&"org.bluez.NetworkServer1".into(), &"Unregister".into(), |msg| {
            let mut i = arg::IterAppend::new(msg);
            i.append(uuid);
        })?;
        m.as_result()?;
        Ok(())
    }
}
