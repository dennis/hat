#[macro_use]
extern crate log;

use std::time::{Duration, SystemTime};

use chrono::prelude::DateTime;
use chrono::Local;
use serde::Serialize;
use structopt::StructOpt;

use cmd_opts::CmdOpts;

use crate::dbus_bluez::BluezManager;
use crate::device::XIAOMI_MIFLORA_SERVICE_UUID;

mod cmd_opts;
mod dbus_bluez;
mod device;

#[derive(Serialize)]
struct ScanResultDevice {
    addr: String,
    name: String,
    alias: String,
    rssi: i16,
}

#[derive(Serialize)]
struct ScanResult {
    devices: Vec<ScanResultDevice>,
}

#[derive(Serialize)]
struct HistoryCountResult {
    total_records: u16,
}

fn scan(
    manager: &mut BluezManager,
    cmd_options: &CmdOpts,
    duration_sec: u8,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let devices = manager.scan(
        XIAOMI_MIFLORA_SERVICE_UUID,
        Some(duration_sec as u32 * 1000),
    )?;
    let mut scan_result: ScanResult = ScanResult {
        devices: Vec::new(),
    };

    for device in devices {
        let device = device::Miflora::new(device, manager)?;

        let name = device.get_name()?;
        let alias = device.get_alias()?;
        let addr = device.get_address()?;
        let rssi = device.get_rssi()?;

        scan_result
            .devices
            .push(ScanResultDevice { name, addr, alias, rssi });
    }

    if cmd_options.json {
        println!("{}", serde_json::to_string(&scan_result)?);
    } else {
        for device in scan_result.devices {
            println!("- addr:  {:}", device.addr);
            println!("  name:  {:}", device.name);
            println!("  alias: {:}", device.alias);
            println!("  rssi:  {:}", device.rssi);
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct ReadResult {
    #[serde(with = "date_format")]
    datetime: DateTime<Local>,
    address: String,
    battery_pct: u8,
    firmware_version: String,
    temperature: f32,
    lux: u32,
    moisture: u8,
    conductivity: u16,
}

#[derive(Serialize)]
struct HistoryRecordResult {
    #[serde(with = "date_format")]
    datetime: DateTime<Local>,
    record_number: u16,
    total_records: u16,
    address: String,
    temperature: f32,
    lux: u32,
    moisture: u8,
    conductivity: u16,
}

mod date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Serializer};

    pub(crate) const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

fn read(
    manager: &mut BluezManager,
    cmd_options: &CmdOpts,
    addr: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let device = manager.find_by_address(addr, Some(60000))?;

    debug!("find_by_address: {:?}", device);

    let mut device = device::Miflora::new(device, manager)?;

    device.connect()?;

    let firmware_version = device.get_firmware_version()?;
    let battery_pct = device.get_battery_pct()?;
    let address = device.get_address()?;
    let readings = device.get_realtime_reading()?;

    let result = ReadResult {
        datetime: Local::now(),
        address,
        battery_pct,
        firmware_version,
        temperature: readings.temperature,
        lux: readings.lux,
        moisture: readings.moisture,
        conductivity: readings.conductivity,
    };

    if cmd_options.json {
        println!("{}", serde_json::to_string(&result)?);
    } else {
        println!("- addr:         {:}", result.address);
        println!("  datetime:     {:}", result.datetime.format(date_format::FORMAT));
        println!("  battery:      {:} %", result.battery_pct);
        println!("  alias:        {:}", result.firmware_version);
        println!("  temperature:  {:} °C", result.temperature);
        println!("  lux:          {:}", result.lux);
        println!("  moisture:     {:} %", result.moisture);
        println!("  conductivity: {:} µS/cm", result.conductivity);
    }

    Ok(())
}

fn blink(
    manager: &mut BluezManager,
    _cmd_options: &CmdOpts,
    addr: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let device = manager.find_by_address(addr, Some(60000))?;

    debug!("find_by_address: {:?}", device);

    let mut device = device::Miflora::new(device, manager)?;

    device.connect()?;

    debug!("blinking");

    device.blink()?;

    Ok(())
}

fn clear_history(
    manager: &mut BluezManager,
    _cmd_options: &CmdOpts,
    addr: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let device = manager.find_by_address(addr, Some(60000))?;

    debug!("find_by_address: {:?}", device);

    let mut device = device::Miflora::new(device, manager)?;

    device.connect()?;

    debug!("clear history");

    device.clear_history()?;

    Ok(())
}

fn history(
    manager: &mut BluezManager,
    cmd_options: &CmdOpts,
    addr: &str,
    from_requested: Option<u16>,
    to_requested: Option<u16>,
    page: Option<u16>,
    clear: bool,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    const DEFAULT_PAGE_SIZE: u16 = 10;
    let device = manager.find_by_address(addr, Some(60000))?;

    debug!("find_by_address: {:?}", device);

    let mut device = device::Miflora::new(device, manager)?;

    device.connect()?;

    let address = device.get_address()?;

    let total_records = device.get_history_record_count()?;

    if total_records == 0 {
        return Ok(());
    }
    
    // NOTE: Re-running this will give different timestamp - but within 3-5 seconds. I wonder
    // if this is a bug or due to the slow communication with Miflora
    let device_epoch = device.get_device_time()?;
    debug!("device epoch: {:?}", device_epoch);
    let device_boot_time = SystemTime::now() - Duration::from_secs(device_epoch as u64);
    debug!(
        "device boot time: {:?}",
        DateTime::<Local>::from(device_boot_time)
    );

    let mut from: u16 = 0;
    let mut to: u16;

    if let Some(from_requested) = from_requested {
        from = from_requested;
    }

    if let Some(to_requested) = to_requested {
        to = to_requested;
    }
    else {
        to = total_records;
    }

    if from > to {
        let t = from;
        from = to;
        to = t;
    }

    if to > total_records {
        to = total_records;
    }

    let page_size = if let Some(p) = page {
        if p > 1 {
            p
        }
        else {
            DEFAULT_PAGE_SIZE
        }
    }
    else {
        DEFAULT_PAGE_SIZE
    };

    let clear = clear && from == 0 && to == total_records;

    debug!("reading history records {:} to {:} with page size {:}", from, to, page_size);

    if !cmd_options.json && !cmd_options.no_headers {
        println!(
            "{datetime:19} {address:16} {temperature:5}    {lux:4}     {moisture:4}   {conductivity:4}      ",
            datetime = "datetime",
            address = "address",
            temperature = "temp",
            lux = "lux",
            moisture = "moist",
            conductivity = "cond"
        );
    }

    let mut page_from = from;

    loop {
        let page_to = if page_from + page_size > to {
            to
        } else {
            page_from + page_size
        };

        debug!("reading page records {:?}-{:?} of {:?}", page_from, page_to, to);

        // device.reconnect()?;

        let records = device.get_history_records(page_from, page_to)?;

        page_from = page_to;

        debug!("got {:?} records", records.len());

        for reading in records {
            let record_time = DateTime::<Local>::from(
                device_boot_time - Duration::from_secs(reading.epoch as u64),
            );

            let result = HistoryRecordResult {
                datetime: record_time,
                record_number: reading.record_number,
                total_records,
                address: address.clone(),
                temperature: reading.temperature,
                lux: reading.lux,
                moisture: reading.moisture,
                conductivity: reading.conductivity,
            };

            if cmd_options.json {
                println!("{}", serde_json::to_string(&result)?);
            } else {
                println!("{datetime:19} {address:16} {temperature:4.1} °C {lux:5} lux {moisture:4} % {conductivity:4} µS/cm", datetime=result.datetime.format(date_format::FORMAT), address=result.address, temperature=result.temperature, lux=result.lux, moisture=result.moisture, conductivity=result.conductivity);
            }
        }

        if page_to == to {
            debug!("Got everything, stopping");
            break;
        }
    }

    if clear {
        debug!("Clearing history");
        device.clear_history()?;
    }

    Ok(())
}

fn history_count(
    manager: &mut BluezManager,
    cmd_options: &CmdOpts,
    addr: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let device = manager.find_by_address(addr, Some(60000))?;

    debug!("find_by_address: {:?}", device);

    let mut device = device::Miflora::new(device, manager)?;

    device.connect()?;

    let result = HistoryCountResult { total_records: device.get_history_record_count()? };

    if cmd_options.json {
        println!("{}", serde_json::to_string(&result)?);
    }
    else {
        println!("Total history records: {:}", result.total_records);
    }

    Ok(())
}

fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut manager = dbus_bluez::BluezManager::new()?;
    let cmd_opts = CmdOpts::from_args();

    manager.start_discovery(Some(4000))?;

    match cmd_opts.cmd {
        cmd_opts::Command::Scan { duration_sec } => scan(&mut manager, &cmd_opts, duration_sec)?,
        cmd_opts::Command::Read { ref addr } => read(&mut manager, &cmd_opts, addr)?,
        cmd_opts::Command::Blink { ref addr } => blink(&mut manager, &cmd_opts, addr)?,
        cmd_opts::Command::History { ref addr, from, to, page, clear } => history(&mut manager, &cmd_opts, addr, from, to, page, clear)?,
        cmd_opts::Command::HistoryCount { ref addr } => history_count(&mut manager, &cmd_opts, addr)?,
        cmd_opts::Command::HistoryClear { ref addr } => clear_history(&mut manager, &cmd_opts, addr)?,
    }

    Ok(())
}

fn main() {
    env_logger::init();

    info!("starting up");

    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            error!("{:?}", e);
            1
        }
    })
}
