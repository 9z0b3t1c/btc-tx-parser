use std::io::Cursor;
use std::io::{Read, Seek, SeekFrom, Write};

pub use crate::transaction::BtcTx;
pub use crate::transaction::Input;
pub use crate::transaction::Output;
pub use crate::util;

// TODO need a better way than just ignoring the return value from write
pub fn build(btc_tx: &BtcTx) -> String {
    let mut cursor = Cursor::new(Vec::new());

    let v = version_number(btc_tx.version_number);
    let _ = cursor.write(&v).unwrap();

    //inputs
    let num_inputs = btc_tx.inputs.len() as u64;
    let _ = cursor.write(&varint(num_inputs));
    for input in &btc_tx.inputs {
        let _ = cursor.write(&input_txid(&input.txid));
        let _ = cursor.write(&input_vout(&input.vout));
        let _ = cursor.write(&script_len(&input.script_sig));
        let _ = cursor.write(&input_script_sig(&input.script_sig));
        let _ = cursor.write(&input_sequence(&input.sequence));
    }

    //outputs
    let num_outputs = btc_tx.outputs.len();
    let _ = cursor.write(&varint(num_outputs.try_into().unwrap()));
    for output in &btc_tx.outputs {
        let _ = cursor.write(&output.amount.to_le_bytes());
        let _ = cursor.write(&script_len(&output.script_pub_key));
        let _ = cursor.write(&output_script_sig(&output.script_pub_key));
    }

    let _ = cursor.write(&btc_tx.locktime.to_le_bytes());

    //write the buffer and return
    write_buffer(&mut cursor)
}

fn version_number(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

fn input_txid(input_txid: &String) -> [u8; 32] {
    let mut input_txid_bytes = hex::decode(input_txid).unwrap();
    input_txid_bytes.reverse();
    input_txid_bytes.try_into().unwrap()
}

fn input_vout(vout: &u32) -> [u8; 4] {
    vout.to_le_bytes()
}

// TODO I don't think returning an array  will work here for big script sigs
fn script_len(script: &String) -> [u8; 1] {
    let script_bytes = hex::decode(script).unwrap();
    let script_len = script_bytes.len() as u8;
    script_len.to_be_bytes()
}

fn input_script_sig(input_script_sig_string: &String) -> Vec<u8> {
    hex::decode(input_script_sig_string).unwrap()
}

fn output_script_sig(output_script_pub_key_string: &String) -> Vec<u8> {
    hex::decode(output_script_pub_key_string).unwrap()
}

fn input_sequence(i: &u32) -> [u8; 4] {
    i.to_le_bytes()
}

// varint in the btc protocol is a variable length field, which indicates the
// length of the next field in the transaction hex.
// If the first byte of the varint is < 253, that's all you need to look at
// If it is 253 (fd), you need to grab the next 2 bytes
// If it is 254 (fe), you need to grab the next 4 bytes
// If it is 255 (ff), you need to grab the next 8 bytes
fn varint(i: u64) -> Vec<u8> {
    if i < u8::MAX.into() {
        let r: u8 = i as u8;
        return r.to_be_bytes().to_vec();
    }
    if i > u8::MAX.into() && i < u16::MAX.into() {
        let mut v = vec![253];
        let r: u16 = i as u16;
        v.append(&mut r.to_le_bytes().to_vec());
        return v;
    }
    if i > u16::MAX.into() && i < u32::MAX.into() {
        let mut v = vec![254];
        let r: u32 = i as u32;
        v.append(&mut r.to_le_bytes().to_vec());
        return v;
    }
    if i > u32::MAX.into() && i < u64::MAX {
        let mut v = vec![255];
        v.append(&mut i.to_le_bytes().to_vec());
        return v;
    }
    vec![]
}

fn write_buffer<T>(c: &mut Cursor<T>) -> String
where
    T: AsRef<[u8]>,
{
    //write the cursor into a buffer
    let size = c.position() as usize;
    let mut buffer = Vec::with_capacity(size);
    c.seek(SeekFrom::Start(0)).unwrap();
    let _ = c.take(size as u64).read_to_end(&mut buffer);
    hex::encode(&buffer)
}

#[test]
fn test_varint_1_byte() {
    let candidate = 8;
    let r = varint(candidate);
    assert_eq!(hex::encode(r), "08");
}

#[test]
fn test_varint_2_bytes() {
    let candidate = 13330;
    let r = varint(candidate);
    assert_eq!(hex::encode(r), "fd1234");
}

#[test]
fn test_varint_4_bytes() {
    let candidate = 2018915346;
    let r = varint(candidate);
    assert_eq!(hex::encode(r), "fe12345678");
}

#[test]
fn test_varint_8_bytes() {
    let candidate = 17279655982273016850;
    let r = varint(candidate);
    assert_eq!(hex::encode(r), "ff1234567890abcdef");
}
