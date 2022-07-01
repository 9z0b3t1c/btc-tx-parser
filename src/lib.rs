use hex::FromHex;
use std::io::Read;

#[derive(Default, Debug)]
pub struct BtcTxParser {
    pub txid: String,
    pub version_number: u32,
}

impl BtcTxParser {
    pub fn new(raw_hex: &str) -> BtcTxParser {
        let mut btp = BtcTxParser {
            ..Default::default()
        };
        let bytes = Vec::<u8>::from_hex(raw_hex).unwrap();
        let bytes_slice = &bytes[..];
        let vn = version_number(bytes_slice);
        println!("version number: {}", vn);
        btp.version_number = vn;
        return btp;
    }
}

// 4-bytes; little endian
fn version_number(mut bytes: &[u8]) -> u32 {
    let mut buffer = [0; 4];
    bytes.read(&mut buffer).ok();
    u32::from_le_bytes(buffer)
}
