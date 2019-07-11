use std::error::Error;
use chrono::prelude::*;
use serde::Serialize;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

static SOURCE: &'static str = "hat-mibcs";

#[derive(Clone, Serialize)]
pub struct WeightData {
    pub source: &'static str,
    pub address: String,
    #[serde(with = "my_date_format", rename = "datetime")]
    pub created_at: DateTime<Local>,
    pub weight: Option<f32>,
    pub impedance: Option<u16>,
}

impl WeightData {
    pub fn decode(value: &Vec<u8>, btaddr: &str) -> Result<WeightData, Box<Error>> {
        let mut rdr = Cursor::new(value.clone());

        let _statusbit0 = rdr.read_u8()?;
        let statusbis1 = rdr.read_u8()?;
        let _year = rdr.read_u16::<LittleEndian>()?;
        let _month = rdr.read_u8()?;
        let _day = rdr.read_u8()?;
        let _hh = rdr.read_u8()?;
        let _mm = rdr.read_u8()?;
        let _ss = rdr.read_u8()?;
        let impedance = rdr.read_u16::<LittleEndian>()?;
        let weight = rdr.read_u16::<LittleEndian>()? as f32 * 0.01 / 2.0;

        let got_impedance: bool = statusbis1 & 0b00000010 != 0;
        let got_weight: bool = statusbis1 & 0b00000100 != 0;
        let weight_stabilized: bool = statusbis1 & 0b00100000 != 0;
        let impedance_stabilized: bool = statusbis1 & 0b10000000 != 0;

        let weight = if got_weight && weight_stabilized {
            Some(weight)
        } else {
            None
        };
        let impedance = if got_impedance && impedance_stabilized {
            Some(impedance)
        } else {
            None
        };

        let data = WeightData {
            source: SOURCE,
            address: btaddr.to_string(),
            created_at: Local::now(),
            weight,
            impedance,
        };

        // println!("{}", serde_json::to_string(&data)?);

        Ok(data)
    }

    pub fn done(&self) -> bool {
        return self.impedance.is_some();
    }

    pub fn dump(&self) -> Result<(), Box<Error>>  {
        println!("{}", serde_json::to_string(&self)?);

        Ok(())
    }
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

