#[derive(Default, Debug)]
pub struct BtcTx {
    pub version_number: u64,
    pub txid: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub locktime: u64,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: String,
    pub vout: u64,
    pub script_sig: String,
    pub sequence: u64,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub amount: u64,
    pub script_pub_key: String,
}
