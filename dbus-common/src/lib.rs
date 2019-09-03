pub mod org_bluez_adapter1;
pub mod org_bluez_device1;
pub mod org_bluez_gatt_service1;
pub mod org_bluez_gatt_characteristic1;
pub mod utils;
pub mod dbus_processor;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
