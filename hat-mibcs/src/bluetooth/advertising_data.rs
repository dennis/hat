use enum_primitive::FromPrimitive;
use std::vec::IntoIter;

use super::adtype;

pub struct AdvertisingData<'t> {
    pub length: usize,
    pub adtype: adtype::Adtype,
    pub data: &'t [u8],
}

impl<'t> AdvertisingData<'t> {
    pub fn dump(&self) {
        println!("length: {:?} (data + type byte)", self.length);
        println!("  type: {:?}", self.adtype.to_string());
        println!("  data: {:?}", self.data);
        println!("");
    }
}

pub fn parse_ad(data: &[u8]) -> Option<IntoIter<AdvertisingData>> {
    // decode Advertising Data.
    // 1st byte = length of the element, excluding the length byte itself
    // 2nd AD type = specifies what data is included in the element
    // 3rd+ byte = data itself
    let mut idx: usize = 0;
    let mut vec = Vec::new();

    while idx < data.len() {
        if idx >= data.len() {
            return None
        }
        let ad_length = data[idx] as usize;
        idx += 1;
        if idx >= data.len() {
            return None
        }
        if ad_length == 0 {
            return None
        }
        let ad_type = data[idx];
        idx += 1;
        if idx > data.len() || idx+ad_length-1 > data.len() {
            return None
        }
        let ad_data = &data[idx..idx + ad_length - 1];
        idx += ad_length - 1;

        let adtype = adtype::Adtype::from_u8(ad_type)?;

        let ad = AdvertisingData {
            length: ad_length,
            adtype: adtype,
            data: ad_data,
        };

        vec.push(ad);
    }

    Some(vec.into_iter())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn good_data_test() {
        let data = [
            0x03, 0x03, 0x9f, 0xfe, 0x17, 0x16, 0x9f, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];

        parse_ad(&data);
    }

    #[test]
    fn bad_data_test() {
        // This is just bad data, so we can't parse it properly
        let data = [
            0x02, 0x01, 0x06, 0x03, 0x02, 0xf0, 0x18, 0x11, 0x06, 0xf2, 0xc3, 0xf0, 0xae, 0xa9,
            0xfa, 0x15, 0x8c, 0x9d, 0x49, 0xae, 0x73, 0x71, 0x0a, 0x81, 0xe7, 0x02, 0x0a, 0x04,
            0x00, 0x00, 0x00
        ];

        assert!(parse_ad(&data).is_none())
    }
}
