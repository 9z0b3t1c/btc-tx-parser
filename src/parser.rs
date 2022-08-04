pub use crate::transaction::BtcTx;
pub use crate::transaction::Input;
pub use crate::transaction::Output;
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::io::Read;

pub fn parse(raw_tx_hex: &String) -> BtcTx {
    let bytes = hex::decode(&raw_tx_hex).unwrap();
    let mut cursor = Cursor::new(&bytes);

    //TODO I think maybe move to a construct method?
    let mut btc_tx = BtcTx {
        txid: hex::encode(txid(&bytes)),
        ..Default::default()
    };

    btc_tx.version_number = version_number(&mut cursor);

    //inputs
    let num_inputs = get_varint(&mut cursor);
    let mut inputs: Vec<Input> = vec![];
    for _ in 0..num_inputs {
        inputs.push(Input {
            txid: input_txid(&mut cursor),
            vout: input_vout(&mut cursor),
            script_sig: input_script_sig(&mut cursor),
            sequence: input_sequence(&mut cursor),
        });
    }

    //outputs
    let num_outputs = get_varint(&mut cursor);
    let mut outputs: Vec<Output> = vec![];
    for _ in 0..num_outputs {
        outputs.push(Output {
            amount: output_amount(&mut cursor),
            script_pub_key: output_script_pub_key(&mut cursor),
        });
    }

    btc_tx.inputs = inputs;
    btc_tx.outputs = outputs;
    btc_tx.locktime = locktime(&mut cursor);
    btc_tx.size = bytes.len();
    btc_tx.vsize = btc_tx.size; // TODO segwit
    btc_tx.weight = btc_tx.size * 4; // TODO segwit
    btc_tx
}

// varint in the btc protocol is a variable length field, which indicates the
// length of the next field in the transaction hex.
// If the first byte of the varint is < 253, that's all you need to look at
// If it is 253 (fd), you need to grab the next 2 bytes
// If it is 254 (fe), you need to grab the next 4 bytes
// If it is 255 (ff), you need to grab the next 8 bytes
fn get_varint<T>(c: &mut Cursor<T>) -> u64
where
    T: AsRef<[u8]>,
{
    let mut first_byte = [0; 1];
    let _ = c.read(&mut first_byte);
    match first_byte {
        [255] => {
            let mut b = [0; 8];
            let _ = c.read(&mut b);
            u64::from_le_bytes(b)
        }
        [254] => {
            let mut b = [0; 4];
            let _ = c.read(&mut b);
            u32::from_le_bytes(b).into()
        }
        [253] => {
            let mut b = [0; 2];
            let _ = c.read(&mut b);
            u16::from_le_bytes(b).into()
        }
        _ => u8::from_be_bytes(first_byte).into(),
    }
}

fn version_number<T>(c: &mut Cursor<T>) -> u32
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 4];
    let _ = c.read(&mut buffer);
    u32::from_le_bytes(buffer)
}

fn input_txid<T>(c: &mut Cursor<T>) -> String
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 32];
    let _ = c.read(&mut buffer);
    buffer.reverse();
    hex::encode(&buffer)
}

fn input_script_sig<T>(c: &mut Cursor<T>) -> String
where
    T: AsRef<[u8]>,
{
    let size = get_varint(c) as usize;
    let mut buffer = Vec::with_capacity(size);
    let _ = c.take(size as u64).read_to_end(&mut buffer);
    hex::encode(&buffer)
}

fn input_vout<T>(c: &mut Cursor<T>) -> u32
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 4];
    let _ = c.read(&mut buffer);
    u32::from_le_bytes(buffer)
}

fn input_sequence<T>(c: &mut Cursor<T>) -> u32
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 4];
    let _ = c.read(&mut buffer);
    u32::from_le_bytes(buffer)
}

fn output_amount<T>(c: &mut Cursor<T>) -> u64
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 8];
    let _ = c.read(&mut buffer);
    u64::from_le_bytes(buffer)
}

fn output_script_pub_key<T>(c: &mut Cursor<T>) -> String
where
    T: AsRef<[u8]>,
{
    let size = get_varint(c) as usize;
    let mut buffer = Vec::with_capacity(size);
    let _ = c.take(size as u64).read_to_end(&mut buffer);
    hex::encode(&buffer)
}

fn locktime<T>(c: &mut Cursor<T>) -> u32
where
    T: AsRef<[u8]>,
{
    let mut buffer = [0; 4];
    let _ = c.read(&mut buffer);
    u32::from_le_bytes(buffer)
}

//The txid is a double sha256 of the raw transaction, in little endian
fn txid(bytes: &[u8]) -> [u8; 32] {
    let mut hash = Sha256::digest(Sha256::digest(bytes));
    hash.reverse();
    <[u8; 32]>::from(hash)
}

#[test]
fn test_txid() {
    let t = "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000";
    let r = parse(&t.to_string());
    assert_eq!(
        r.txid,
        "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16"
    );
}

#[test]
fn test_varint_1_byte() {
    let bytes = hex::decode("08").unwrap();
    let mut cursor = Cursor::new(&bytes);
    let v = get_varint(&mut cursor);
    assert_eq!(v, 8);
}

#[test]
fn test_varint_2_bytes() {
    let bytes = hex::decode("fd1234").unwrap();
    let mut cursor = Cursor::new(&bytes);
    let v = get_varint(&mut cursor);
    assert_eq!(v, 13330);
}

#[test]
fn test_varint_4_bytes() {
    let bytes = hex::decode("fe12345678").unwrap();
    let mut cursor = Cursor::new(&bytes);
    let v = get_varint(&mut cursor);
    assert_eq!(v, 2018915346);
}

#[test]
fn test_varint_8_bytes() {
    let bytes = hex::decode("ff1234567890abcdef").unwrap();
    let mut cursor = Cursor::new(&bytes);
    let v = get_varint(&mut cursor);
    assert_eq!(v, 17279655982273016850);
}
