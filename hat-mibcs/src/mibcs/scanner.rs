use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use btlepasvscan::Error as BtleError;

use super::weight_data::WeightData;
use crate::bluetooth;
use crate::Cli;

pub struct Error {
    pub message: String,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            message: error.to_string(),
        }
    }
}

impl std::convert::From<()> for Error {
    fn from(_error: ()) -> Error {
        let last_err = std::io::Error::last_os_error();

        Error {
            message: last_err.to_string(),
        }
    }
}

impl std::convert::From<BtleError> for Error {
    fn from(error: BtleError) -> Error {
        Error {
            message: error.to_string(),
        }
    }
}

impl std::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error {
            message: error.to_string(),
        }
    }
}

impl std::convert::From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Error {
        Error {
            message: error.to_string(),
        }
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(_error: serde_json::error::Error) -> Error {
        Error {
            message: "cannot generate json".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct MIBCSScanner<'t> {
    cli: &'t Cli,
    last_weight_data: Option<WeightData>,
    announced: bool,
}

impl<'t> MIBCSScanner<'t> {
    pub fn new(cli: &Cli) -> MIBCSScanner {
        MIBCSScanner {
            cli,
            last_weight_data: None,
            announced: false,
        }
    }

    pub fn scan(&mut self) -> Result<(), Error> {
        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&term))?;

        let mut scanner = btlepasvscan::BtlePasvScan::new()?;
        let now = SystemTime::now();

        loop {
            match scanner.read() {
                Ok(data) => {
                    if self.handle_data(&data)? && self.cli.until_data {
                        break;
                    }
                }
                Err(_) => break,
            }

            if term.load(Ordering::Relaxed) {
                break;
            }


            if self.cli.duration > 0 {
                let elapsed = now.elapsed()?;

                if elapsed.as_secs() > self.cli.duration {
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_data(&mut self, data: &btlepasvscan::Data) -> Result<bool, Error> {
        if self.cli.debug {
            print!("{:?} ", data.address);
            for b in data.buffer {
                print!("0x{:02x?} ", *b);
            }
            println!("");
        }

        let ads = bluetooth::advertising_data::parse_ad(&data.buffer);

        if ads.is_none() {
            return Ok(false);
        }

        let ads = ads.unwrap();

        if self.cli.debug {
            for ad in ads.as_slice() {
                ad.dump();
            }
        }

        let wiscale_data = match WeightData::parse(data.address.to_str()?, ads.as_slice()) {
            Some(scale_data) => match self.last_weight_data.as_mut() {
                Some(previous) => {
                    let mut result = false;

                    if self.cli.debug {
                        scale_data.dump();
                    }

                    previous.update(&scale_data, self.cli.debug);

                    if previous.announcable && !previous.announced {
                        println!("{}", previous.announcement()?);
                        result = true;
                    }

                    result
                }
                None => {
                    self.last_weight_data = Some(scale_data.clone());
                    false
                }
            },
            None => false,
        };

        Ok(wiscale_data)
    }
}
