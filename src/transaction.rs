use bitcoin::hashes::hex::FromHex;
use bitcoin::Txid;
use bitcoincore_rpc::{bitcoin, RpcApi};

use crate::parser;
use crate::util;

#[derive(Default, Debug)]
pub struct BtcTx {
    pub version_number: u32, // 4 byte le
    pub txid: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub locktime: u32, // 4 byte le
    pub transaction_fee: u64,
    pub size: usize,
    pub vsize: usize,
    pub weight: usize,
    //TODO derive the following fields:
    //blockhash
    //confirmations
    //time
    //blocktime
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: String,
    //vout is the index number of this input in it's previous transaction's vector of outputs
    pub vout: u32, // 4 byte le
    // The script to unlock this transaction
    pub script_sig: String,
    pub sequence: u32, // 4 byte le
}

#[derive(Debug, Clone)]
pub struct Output {
    pub amount: u64, // 8 byte le
    //The script to lock the output
    pub script_pub_key: String,
}

impl BtcTx {
    pub fn get_transaction_fee(&mut self) -> u64 {
        let output_sum: u64 = self
            .outputs
            .iter()
            .map(|o| o.amount)
            .collect::<Vec<u64>>()
            .iter()
            .sum();
        let mut input_sum = 0;
        for i in &self.inputs {
            let it = Txid::from_hex(&i.txid).unwrap();
            let ir = util::client().get_raw_transaction_hex(&it, None).unwrap();
            let ip = parser::parse(&ir);
            let relevant_amount = ip.outputs[i.vout as usize].amount;
            input_sum += relevant_amount;
        }
        let diff = input_sum - output_sum;
        self.transaction_fee = diff;
        diff
    }

    pub fn sats_per_vbyte(&mut self) -> f64 {
        let r: f64 = self.transaction_fee as f64 / self.vsize as f64;
        r
    }
}
