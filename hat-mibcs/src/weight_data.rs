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
    pub fn decode(value: &Vec<u8>, btaddr: &str, debug : bool) -> Result<WeightData, Box<Error>> {
        let mut rdr = Cursor::new(value.clone());

        let statusbit0 = rdr.read_u8()?;
        let statusbit1 = rdr.read_u8()?;
        let year = rdr.read_u16::<LittleEndian>()?;
        let month = rdr.read_u8()?;
        let day = rdr.read_u8()?;
        let hh = rdr.read_u8()?;
        let mm = rdr.read_u8()?;
        let ss = rdr.read_u8()?;
        let impedance = rdr.read_u16::<LittleEndian>()?;
        let weight = rdr.read_u16::<LittleEndian>()? as f32 * 0.01 / 2.0;

        let got_impedance: bool = statusbit1 & 0b00000010 != 0;
        let got_weight: bool = statusbit1 & 0b00000100 != 0;
        let weight_stabilized: bool = statusbit1 & 0b00100000 != 0;
        let impedance_stabilized: bool = statusbit1 & 0b10000000 != 0;

        if debug {
            eprintln!("  decoded weight data:");
            eprintln!("    statusbit0  {:010b}", statusbit0);
            eprintln!("    statusbit1  {:010b}", statusbit1);
            eprintln!("    yymmdd      {:?}{:?}{:?}", year, month, day);
            eprintln!("    hhmmss      {:?}{:?}{:?}", hh, mm, ss);
            eprintln!("    impedance   {:?}", impedance);
            eprintln!("    weight      {:?}", weight);
            eprintln!("      impedance            {:?}", got_impedance);
            eprintln!("      impedance_stabilized {:?}", impedance_stabilized);
            eprintln!("      weight               {:?}", got_weight);
            eprintln!("      weight_stabilized    {:?}", weight_stabilized);
        }


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

