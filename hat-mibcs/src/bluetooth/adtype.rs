use enum_primitive::FromPrimitive;

enum_from_primitive! {

#[derive(Debug, PartialEq)]
pub enum Adtype {
    Flags = 0x01,
    IncompleteListOf16bitServiceClassUUIDs = 0x02,
    CompleteListof16BitServiceClassUUIDs = 0x03,
    IncompleteListof32BitServiceClassUUIDs = 0x04,
    CompleteListof32BitServiceClassUUIDs = 0x05,
    IncompleteListof128BitServiceClassUUIDs = 0x06,
    CompleteListof128BitServiceClassUUIDs = 0x07,
    ShortenedLocalName = 0x08,
    CompleteLocalName = 0x09,
    TxPowerLevel = 0x0a,
    ClassofDevice = 0x0d,
    SimplePairingHashC = 0x0e,
    SimplePairingRandomizerR = 0x0f,
    DeviceID = 0x10,
    SecurityManagerOutofBandFlags = 0x11,
    SlaveConnectionIntervalRange = 0x12,
    Listof16BitServiceSolicitationUUIDs = 0x14,
    Listof128BitServiceSolicitationUUIDs=0x15,
    ServiceData = 0x16,
    PublicTargetAddress = 0x17,
    RandomTargetAddress = 0x18,
    Appearance = 0x19,
    AdvertisingInterval = 0x1a,
    LEBluetoothDeviceAddress = 0x1b,
    LERole = 0x1c,
    SimplePairingHashC256 = 0x1d,
    SimplePairingRandomizerR256 = 0x1e,
    ListOf32BitServiceSolicitationUUIDs = 0x1f,
    ServiceData32BitUUID = 0x20,
    ServiceData128BitUUID = 0x21,
    LESecureConnectionsConfirmationValue = 0x22,
    LESecureConnectionsRandomValue = 0x23,
    URI = 0x24,
    IndoorPositioning = 0x25,
    TransportDiscoveryData = 0x26,
    LESupportedFeatures = 0x27,
    ChannelMapUpdateIndication = 0x28,
    PBADV = 0x29,
    MeshMessage = 0x2a,
    MeshBeacon = 0x2b,
    ThreeDInformationData = 0x3d,
    ManufacturerSpecificData = 0xff,
}
}

impl std::string::ToString for Adtype {
    fn to_string(&self) -> String {
        match self {
            Adtype::Flags => "Flags",
            Adtype::IncompleteListOf16bitServiceClassUUIDs => {
                "Incomplete List of 16-bit Service Class UUIDs"
            }
            Adtype::CompleteListof16BitServiceClassUUIDs => {
                "Complete List of 16-bit Service Class UUIDs"
            }
            Adtype::IncompleteListof32BitServiceClassUUIDs => {
                "Incomplete List of 32-bit Service Class UUIDs"
            }
            Adtype::CompleteListof32BitServiceClassUUIDs => {
                "Complete List of 32-bit Service Class UUIDs"
            }
            Adtype::IncompleteListof128BitServiceClassUUIDs => {
                "Incomplete List of 128-bit Service Class UUIDs"
            }
            Adtype::CompleteListof128BitServiceClassUUIDs => {
                "Complete List of 128-bit Service Class UUIDs"
            }
            Adtype::ShortenedLocalName => "Shortened Local Name",
            Adtype::CompleteLocalName => "Complete Local Name",
            Adtype::TxPowerLevel => "Tx Power Level",
            Adtype::ClassofDevice => "Class of Device",
            Adtype::SimplePairingHashC => "Simple Pairing Hash C/Simple Pairing Hash C-192",
            Adtype::SimplePairingRandomizerR => {
                "Simple Pairing Randomizer R/Simple Pairing Randomizer R-192"
            }
            Adtype::DeviceID => "Device ID/Security Manager TK Value",
            Adtype::SecurityManagerOutofBandFlags => "Security Manager Out of Band Flags",
            Adtype::SlaveConnectionIntervalRange => "Slave Connection Interval Range",
            Adtype::Listof16BitServiceSolicitationUUIDs => {
                "List of 16-bit Service Solicitation UUIDs"
            }
            Adtype::Listof128BitServiceSolicitationUUIDs => {
                "List of 128-bit Service Solicitation UUIDs"
            }
            Adtype::ServiceData => "Service Data/Service Data - 16-bit UUID",
            Adtype::PublicTargetAddress => "Public Target Address",
            Adtype::RandomTargetAddress => "Random Target Address",
            Adtype::Appearance => "Appearance",
            Adtype::AdvertisingInterval => "Advertising Interval",
            Adtype::LEBluetoothDeviceAddress => "LE Bluetooth Device Address",
            Adtype::LERole => "LE Role",
            Adtype::SimplePairingHashC256 => "Simple Pairing Hash C-256",
            Adtype::SimplePairingRandomizerR256 => "Simple Pairing Randomizer R-256",
            Adtype::ListOf32BitServiceSolicitationUUIDs => {
                "List of 32-bit Service Solicitation UUIDs"
            }
            Adtype::ServiceData32BitUUID => "Service Data - 32-bit UUID",
            Adtype::ServiceData128BitUUID => "Service Data - 128-bit UUID",
            Adtype::LESecureConnectionsConfirmationValue => {
                "LE Secure Connections Confirmation Value"
            }
            Adtype::LESecureConnectionsRandomValue => "LE Secure Connections Random Value",
            Adtype::URI => "URI",
            Adtype::IndoorPositioning => "Indoor Positioning",
            Adtype::TransportDiscoveryData => "Transport Discovery Data",
            Adtype::LESupportedFeatures => "LE Supported Features",
            Adtype::ChannelMapUpdateIndication => "Channel Map Update Indication",
            Adtype::PBADV => "PB-ADV",
            Adtype::MeshMessage => "Mesh Message",
            Adtype::MeshBeacon => "Mesh Beacon",
            Adtype::ThreeDInformationData => "3D Information Data",
            Adtype::ManufacturerSpecificData => "Manufacturer Specific Data",
        }
        .to_string()
    }
}

impl From<u8> for Adtype {
    fn from(val: u8) -> Adtype {
        Adtype::from_u8(val).expect("passed Value does not match an enum value!")
    }
}
