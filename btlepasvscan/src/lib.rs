extern crate btlepasvscan_sys;

use std::ffi::CStr;
use std::slice;

pub enum Error {
    Context,
    Recv,
    BadData,
    SetSockOpt,
    Bind,
    HciLeSetScanParameters,
    HciLeSetScanEnable,
    HciLeSetScanDisable,
    Open,
}

impl ToString for Error {
    fn to_string(&self) -> std::string::String {
        match self {
            Error::Context => {
                "bluetooth context ".to_string() + &std::io::Error::last_os_error().to_string()
            }
            Error::Recv => "recv() ".to_string() + &std::io::Error::last_os_error().to_string(),
            Error::SetSockOpt => {
                "setsockopt() ".to_string() + &std::io::Error::last_os_error().to_string()
            }
            Error::Bind => "bind() ".to_string() + &std::io::Error::last_os_error().to_string(),
            Error::HciLeSetScanParameters => {
                "hci_le_set_scan_parameters() ".to_string()
                    + &std::io::Error::last_os_error().to_string()
            }
            Error::HciLeSetScanEnable => {
                "hci_le_set_scan_enable() ".to_string()
                    + &std::io::Error::last_os_error().to_string()
            }
            Error::HciLeSetScanDisable => {
                "hci_le_set_scan_disable() ".to_string()
                    + &std::io::Error::last_os_error().to_string()
            }
            Error::BadData => "Received unexpected BTLE data ".to_string(),
            Error::Open => "open() ".to_string(),
        }
    }
}

pub struct BtlePasvScan {
    context: *mut btlepasvscan_sys::btlepasvscan_ctx,
}

pub struct Data<'t> {
    pub buffer: &'t [u8],
    pub address: &'t CStr,
}

impl Drop for BtlePasvScan {
    fn drop(&mut self) {
        unsafe {
            btlepasvscan_sys::btlepasvscan_close(self.context);
        }
    }
}

impl BtlePasvScan {
    pub fn new() -> Result<BtlePasvScan, Error> {
        let context = unsafe { btlepasvscan_sys::btlepasvscan_open() };

        if context.is_null() {
            Err(Error::Context)
        } else {
            match unsafe { (*context).error } {
                0 => Ok(BtlePasvScan { context }),
                1 => Err(Error::Recv),
                2 => Err(Error::BadData),
                3 => Err(Error::SetSockOpt),
                4 => Err(Error::Bind),
                5 => Err(Error::HciLeSetScanParameters),
                6 => Err(Error::HciLeSetScanEnable),
                7 => Err(Error::HciLeSetScanDisable),
                8 => Err(Error::Open),
                _ => Err(Error::Context)
            }
        }
    }

    pub fn read(&mut self) -> Result<Data, ()> {
        let rc = unsafe { btlepasvscan_sys::btlepasvscan_read(self.context) };

        if rc == 0 {
            Err(())
        } else {
            let buffer: &[u8] = unsafe {
                slice::from_raw_parts((*self.context).data, (*self.context).length as usize)
            };
            let address: &CStr = unsafe { CStr::from_ptr((*self.context).address.as_mut_ptr()) };

            Ok(Data { buffer, address })
        }
    }
}
