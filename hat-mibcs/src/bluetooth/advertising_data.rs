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

pub fn parse_ad(data: &[u8]) -> IntoIter<AdvertisingData> {
    // decode Advertising Data.
    // 1st byte = length of the element, excluding the length byte itself
    // 2nd AD type = specifies what data is included in the element
    // 3rd+ byte = data itself
    let mut idx: usize = 0;
    let mut vec = Vec::new();

    while idx < data.len() {
        let ad_length = data[idx] as usize;
        idx += 1;
        let ad_type = data[idx];
        idx += 1;
        let ad_data = &data[idx..idx + ad_length - 1];
        idx += ad_length - 1;

        let ad = AdvertisingData {
            length: ad_length,
            adtype: adtype::Adtype::from_u8(ad_type).unwrap(),
            data: ad_data,
        };

        vec.push(ad);
    }

    vec.into_iter()
}
