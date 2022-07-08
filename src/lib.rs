use sha2::{Digest, Sha256};
mod transaction;
pub mod util;
pub use crate::transaction::BtcTx;
pub use crate::transaction::Input;
pub use crate::transaction::Output;


#[derive(Debug)]
pub struct BtcTxParser {
    tx_hex: String,
    index: usize,
}

impl BtcTxParser {
    pub fn parse(raw_tx_hex: &String) -> BtcTx {
        let mut parser = BtcTxParser {
            tx_hex: raw_tx_hex.to_string(),
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
        btc_tx.size = hex::decode(&raw_tx_hex).unwrap().len();
        btc_tx.weight = btc_tx.size * 4;
        btc_tx
    }

    //le
    fn txid(&mut self) -> String {
        let hash = Sha256::digest(Sha256::digest(hex::decode(&self.tx_hex).unwrap()));
        let txid = util::convert_endian(&hex::encode(hash));
        txid
    }

    //le
    fn version_number(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        util::bytes_to_u64(&b)
    }

    //TODO this has to handle variable length inputs - the 'compact size' shenanigans
    fn input_count(&mut self) -> u8 {
        self.get_bytes(1, false)[0]
    }

    //TODO this has to handle variable length inputs - the 'compact size' shenanigans
    fn output_count(&mut self) -> u8 {
        self.get_bytes(1, false)[0]
    }

    fn input_txid(&mut self) -> String {
        let b = self.get_bytes(32, true);
        hex::encode(b)
    }

    //le
    fn vout(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        util::bytes_to_u64(&b)
    }

    //TODO figure out the correct compact size behaviour
    fn script_sig(&mut self) -> String {
        let script_sig_len = self.get_varint() as usize;
        hex::encode(self.get_bytes(script_sig_len, false))
    }

    fn sequence(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        util::bytes_to_u64(&b)
    }

    fn amount(&mut self) -> u64 {
        let b = self.get_bytes(8, true);
        util::bytes_to_u64(&b)
    }

    fn script_pub_key(&mut self) -> String {
        let script_pub_key_size = self.get_varint() as usize;
        hex::encode(self.get_bytes(script_pub_key_size, false))
    }

    fn locktime(&mut self) -> u64 {
        let b = self.get_bytes(4, true);
        util::bytes_to_u64(&b)
    }

    // Gets 'n' bytes as a string
    // Flips the string if endianness requires
    // Advances the index
    // Transforms the string into a vec of hex
    fn get_bytes(&mut self, n: usize, convert_endian: bool) -> Vec<u8> {
        let (start, end) = (self.index, self.index + n * 2);
        self.index += n * 2;
        let mut string = self.tx_hex[start..end].to_string();
        if convert_endian {
            string = util::convert_endian(&string)
        }
        hex::decode(string).unwrap()
    }

    fn get_varint(&mut self) -> u64 {
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
        util::bytes_to_u64(&byte_vector)
    }
}
