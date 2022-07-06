use sha2::{Digest, Sha256};
#[path = "./transaction.rs"]
mod transaction;
pub use crate::transaction::BtcTx;
pub use crate::transaction::Input;
pub use crate::transaction::Output;

#[derive(Debug)]
pub struct BtcTxParser {
    tx_hex: String,
    index: usize,
}

impl BtcTxParser {
    pub fn parse(raw_tx_hex: String) -> BtcTx {
        let mut parser = BtcTxParser {
            tx_hex: raw_tx_hex,
            index: 0,
        };
        let mut btc_tx = BtcTx {
            txid: parser.txid(),
            version_number: parser.version_number(),
            ..Default::default()
        };

        let input_count = parser.input_count();
        let mut inputs: Vec<Input> = vec![];
        for _ in 0..input_count {
            inputs.push(Input {
                txid: parser.input_txid(),
                vout: parser.vout(),
                script_sig: parser.script_sig(),
                sequence: parser.sequence(),
            });
        }

        let output_count = parser.output_count();
        let mut outputs: Vec<Output> = vec![];
        for _ in 0..output_count {
            outputs.push(Output {
                amount: parser.amount(),
                script_pub_key: parser.script_pub_key(),
            });
        }

        btc_tx.inputs = inputs;
        btc_tx.outputs = outputs;
        btc_tx.locktime = parser.locktime();
        btc_tx
    }

    fn txid(&mut self) -> String {
        let hash = Sha256::digest(Sha256::digest(hex::decode(&self.tx_hex).unwrap()));
        let txid = BtcTxParser::convert_endian(&hex::encode(hash));
        txid
    }

    fn version_number(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        BtcTxParser::bytes_to_u64(&b)
    }

    //TODO this has to handle variable length inputs
    fn input_count(&mut self) -> u8 {
        self.get_bytes(1, false)[0]
    }

    fn output_count(&mut self) -> u8 {
        self.get_bytes(1, false)[0]
    }

    fn input_txid(&mut self) -> String {
        let b = self.get_bytes(32, true);
        hex::encode(b)
    }

    fn vout(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        BtcTxParser::bytes_to_u64(&b)
    }

    fn script_sig(&mut self) -> String {
        let script_sig_len = self.get_varint() as usize;
        hex::encode(self.get_bytes(script_sig_len, false))
    }

    fn sequence(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        BtcTxParser::bytes_to_u64(&b)
    }

    fn amount(&mut self) -> u64 {
        let b = self.get_bytes(8, true);
        BtcTxParser::bytes_to_u64(&b)
    }

    fn script_pub_key(&mut self) -> String {
        let script_pub_key_size = self.get_varint() as usize;
        hex::encode(self.get_bytes(script_pub_key_size, false))
    }

    fn locktime(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        BtcTxParser::bytes_to_u64(&b)
    }

    fn get_bytes(&mut self, n: usize, convert_endian: bool) -> Vec<u8> {
        let (start, end) = (self.index, self.index + n * 2);
        self.index += n * 2;
        let mut string = self.tx_hex[start..end].to_string();
        if convert_endian {
            string = BtcTxParser::convert_endian(&string)
        }
        hex::decode(string).unwrap()
    }

    pub fn convert_endian(string: &str) -> String {
        let mut new_string = String::new();
        let mut prev_char = ' ';
        for (i, curr_char) in string.chars().rev().enumerate() {
            if i % 2 == 0 {
                prev_char = curr_char;
                continue;
            }
            new_string.push(curr_char);
            new_string.push(prev_char);
        }
        return new_string;
    }

    pub fn bytes_to_u64(byte_vector: &Vec<u8>) -> u64 {
        if byte_vector.len() > 8 {
            panic!("Input exceeds u64 size")
        }
        let mut varint = 0;
        for byte in byte_vector {
            varint <<= 8;
            varint |= *byte as u64;
        }
        return varint;
    }

    pub fn get_varint(&mut self) -> u64 {
        let integer = self.get_bytes(1, false)[0];
        let i = u8::MAX - integer;
        if i > 2 {
            return integer as u64;
        }
        let bytes = if i == 0 {
            8
        } else if i == 1 {
            4
        } else {
            2
        };
        let byte_vector = self.get_bytes(bytes, true);
        return BtcTxParser::bytes_to_u64(&byte_vector);
    }
}
