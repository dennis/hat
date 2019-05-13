use crate::bluetooth;
use chrono::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct WeightDataAnnouncement {
    pub source: String,
    pub address: String,
    #[serde(with = "my_date_format")]
    pub datetime: DateTime<Local>,
    pub weight: Option<f32>,
    pub impedance: Option<u32>,
}

#[derive(Clone, Serialize)]
pub struct WeightData {
    pub address: String,
    #[serde(with = "my_date_format", rename = "datetime")]
    pub created_at: DateTime<Local>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Local>,
    #[serde(skip_serializing)]
    pub status_updated_at: DateTime<Local>,
    #[serde(skip_serializing)]
    pub announced: bool,
    #[serde(skip_serializing)]
    pub announcable: bool,
    pub weight: f32,
    pub impedance: u32,
    #[serde(skip_serializing)]
    pub statusbits0: u8,
    #[serde(skip_serializing)]
    pub statusbits1: u8,
    #[serde(skip_serializing)]
    pub got_impedance: bool, // statusbits1, bit 1
    #[serde(skip_serializing)]
    pub got_weight: bool, // statusbits1, bit 2
    #[serde(skip_serializing)]
    pub weight_stabilized: bool, // statusbits1, bit 5
    #[serde(skip_serializing)]
    pub impedance_stabilized: bool, // statusbits1, bit 7
}

mod my_date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

impl WeightData {
    pub fn parse(
        address: &str,
        ads: &[bluetooth::advertising_data::AdvertisingData],
    ) -> Option<WeightData> {
        if ads.len() != 3 {
            return None;
        }

        if ads[0].adtype != bluetooth::adtype::Adtype::Flags {
            return None;
        }
        if ads[1].adtype != bluetooth::adtype::Adtype::IncompleteListOf16bitServiceClassUUIDs {
            return None;
        }
        if ads[2].adtype != bluetooth::adtype::Adtype::ServiceData {
            return None;
        }
        if ads[2].data[0] != 0x1b || ads[2].data[1] != 0x18 {
            return None;
        }

        let weight_low = ads[2].data[13];
        let weight_high = ads[2].data[14];

        let weight: f32 = (((weight_high as u32) << 8) + weight_low as u32) as f32 * 0.01 / 2.0;

        let impedance_low = ads[2].data[11];
        let impedance_high = ads[2].data[12];
        let impedance: u32 = ((impedance_high as u32) << 8) + impedance_low as u32;

        let statusbits0 = ads[2].data[2];
        let statusbits1 = ads[2].data[3];

        let got_impedance: bool = statusbits1 & 0b00000010 != 0;
        let got_weight: bool = statusbits1 & 0b00000100 != 0;
        let weight_stabilized: bool = statusbits1 & 0b00100000 != 0;
        let impedance_stabilized: bool = statusbits1 & 0b10000000 != 0;

        Some(WeightData {
            address: address.to_string(),
            created_at: Local::now(),
            updated_at: Local::now(),
            status_updated_at: Local::now(),
            announced: false,
            announcable: false,
            weight,
            impedance,
            statusbits0,
            statusbits1,
            got_impedance,
            got_weight,
            weight_stabilized,
            impedance_stabilized,
        })
    }

    pub fn update(&mut self, weight_data: &WeightData, debug: bool) {
        // This implementation assumes only one Mi Scale weight
        assert!(self.address == weight_data.address);

        // If we haven't received data from it for the last 5 seconds, it must
        // be a new measurement
        let elapsed = self
            .updated_at
            .signed_duration_since(Local::now())
            .num_seconds();

        if elapsed > 5 {
            if debug {
                println!("elapsed = {:?} > 5 - resetting", elapsed);
            }

            self.announcable = false;
            self.announced = false;
        }

        // if the stabilized flags are turned off, then its are new measurement
        if self.weight_stabilized && !weight_data.weight_stabilized
            || self.impedance_stabilized && !weight_data.impedance_stabilized
        {
            if debug {
                println!("something became unstabilized, reset");
            }

            self.announcable = false;
            self.announced = false;
        }

        // uptime timestamps
        self.updated_at = Local::now();

        if self.statusbits0 != weight_data.statusbits0
            || self.statusbits1 != weight_data.statusbits1
        {
            self.status_updated_at = self.updated_at;
        }

        // anything to announce?
        if !self.announced {
            // We got both weight_stabilized and impedance_stabilized - then announce it
            self.announcable = weight_data.weight_stabilized && weight_data.impedance_stabilized;
        } else if !self.announced && self.weight_stabilized {
            // We got weight_stabilized - but more than 5 seconds since last stausbits update
            let elapsed = self
                .status_updated_at
                .signed_duration_since(Local::now())
                .num_seconds();
            if debug {
                println!("time since last status update: {:?}", elapsed);
            }

            self.announcable = elapsed > 5;
        }

        self.weight = weight_data.weight;
        self.impedance = weight_data.impedance;
        self.statusbits0 = weight_data.statusbits0;
        self.statusbits1 = weight_data.statusbits1;
        self.got_impedance = weight_data.got_impedance;
        self.got_weight = weight_data.got_weight;
        self.weight_stabilized = weight_data.weight_stabilized;
        self.impedance_stabilized = weight_data.impedance_stabilized;
    }

    pub fn announcement(&mut self) -> Result<String, serde_json::error::Error> {
        let data = WeightDataAnnouncement {
            source: "hat-mibcs".to_string(),
            address: self.address.clone(),
            datetime: self.created_at,
            weight: if self.got_weight {
                Some(self.weight)
            } else {
                None
            },
            impedance: if self.got_impedance {
                Some(self.impedance)
            } else {
                None
            },
        };

        self.announced = true;
        Ok(serde_json::to_string(&data)?)
    }

    pub fn dump(&self) {
        println!("{:?} created_at={:?}, updated_at={:?}, weight={:?}, impedance={:?}, got_impedance={:?}, got_weight={:?}, impedance_stabilized={:?}, weight_stabilized={:?}, statusbits={:#010b} #{:#010b}", self.address, self.created_at, self.updated_at, self.weight, self.impedance, self.got_impedance, self.got_weight, self.impedance_stabilized, self.weight_stabilized, self.statusbits0, self.statusbits1);
    }
}
