use bitcoincore_rpc::{bitcoin, RpcApi};
use bitcoin::hashes::hex::FromHex;
use bitcoin::Txid;

use crate::util;
use crate::BtcTxParser;


#[derive(Default, Debug)]
pub struct BtcTx {
    pub version_number: u64, // 4 byte le
    pub txid: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub locktime: u64, // 4 byte le
    pub transaction_fee: u64,
    //TODO derive the following fields:
    //size
    //vsize
    //weight
    //blockhash
    //confirmations
    //time
    //blocktime
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: String,
    pub vout: u64, // 4 byte le
    pub script_sig: String,
    pub sequence: u64, // 4 byte le
}

#[derive(Debug, Clone)]
pub struct Output {
    pub amount: u64, // 8 byte le
    pub script_pub_key: String,
}

impl BtcTx {
    pub fn get_transaction_fee(&mut self) -> u64 {
        let output_sum: u64 = self.outputs.iter().map(|o|
                                                      o.amount).collect::<Vec<u64>>().iter().sum();
        let mut input_sum = 0;
        for i in &self.inputs {
            let it = Txid::from_hex(&i.txid).unwrap();
            let ir = util::client().get_raw_transaction_hex(&it, None).unwrap();
            let ip = BtcTxParser::parse(ir);
            let relevant_amount = ip.outputs[i.vout as usize].amount;
            input_sum += relevant_amount;
        }
        let diff = input_sum - output_sum;
        self.transaction_fee = diff;
        diff
    }
}
