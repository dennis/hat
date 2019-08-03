static ROOT_SERVICE_UUID: &'static str = "0000fe95-0000-1000-8000-00805f9b34fb";

use std::error::Error;

use std::thread;
use std::time::Duration;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::bluetooth_session::BluetoothSession as Session;

use crate::Cli;
use crate::Miflora;

pub struct Scanner<'a> {
    cli : &'a Cli,
}

impl<'a> Scanner<'a> {
    pub fn new(cli : &'a Cli) -> Scanner {
        Scanner { cli }
    }

    pub fn find_mifloras(&self, bt_session : &Session) -> Result<Vec<Miflora>, Box<Error>> {
        let adapter: Adapter = Adapter::init(bt_session)?;
        let session = DiscoverySession::create_session(
            &bt_session,
            adapter.get_id()
        )?;

        session.start_discovery()?;
        thread::sleep(Duration::from_millis(1000));
        session.stop_discovery()?;

        let devices = adapter.get_device_list()?;

        let mifloras : Vec<Miflora> = devices
            .iter()
            .filter( |&device| self.is_miflora(bt_session, device) )
            .map( |device| Miflora::new(device.clone()) )
            .collect();

        if self.cli.debug { eprintln!("found mifloras: {:?}", mifloras) }

        Ok(mifloras)
    }

    fn is_miflora(&self, bt_session : &Session, device_path : &String) -> bool {
        if let Ok(result) = self.look_for_root_device(bt_session, device_path) {
            if self.cli.debug { eprintln!("is_miflora: {:?} = {:?}", device_path, if result { "yes" } else { "no" }); }
            return result;
        }
        else {
            if self.cli.debug { eprintln!("is_miflora: {:?} = error getting data", device_path); }
            return false;
        }
    }

    fn look_for_root_device(&self, bt_session : &Session, device_path : &String) -> Result<bool, Box<Error>> {
        let device = Device::new(bt_session, device_path.clone());
        let uuids = device.get_uuids()?;
        'uuid_loop: for uuid in uuids {
            if uuid == ROOT_SERVICE_UUID {
                return Ok(true)
            }
        }

        Ok(false)
    }
}
