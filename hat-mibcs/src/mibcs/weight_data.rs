use crate::bluetooth;

pub struct WeightData {
    pub address: String,
    pub weight: f32,
    pub impedance: u32,
    pub statusbits0: u8,
    pub statusbits1: u8,
    pub got_impedance: bool,        // statusbits1, bit 1
    pub got_weight: bool,           // statusbits1, bit 2
    pub weight_stabilized: bool,    // statusbits1, bit 5
    pub impedance_stabilized: bool, // statusbits1, bit 7
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

    pub fn dump(&self) {
        println!("{:?} weight={:?}, impedance={:?}, got_impedance={:?}, got_weight={:?}, impedance_stabilized={:?}, weight_stabilized={:?}, statusbits={:#010b} #{:#010b}", self.address, self.weight, self.impedance, self.got_impedance, self.got_weight, self.impedance_stabilized, self.weight_stabilized, self.statusbits0, self.statusbits1);
    }
}
