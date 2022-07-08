use bitcoincore_rpc::{Auth, Client};

pub fn client() -> Client {
    let btc_cli_ip = std::env::var("BTC_CLI_IP").expect("BTC_CLI_IP env var not set.");
    let btc_cli_port = std::env::var("BTC_CLI_PORT").expect("BTC_CLI_PORT env var not set.");
    let btc_cli_username = std::env::var("BTC_CLI_USERNAME").expect("BTC_CLI_USERNAME env var not set");
    let btc_cli_password = std::env::var("BTC_CLI_PASSWORD").expect("BTC_CLI_PASSWORD env var not set");
    let btc_cli_url = format!("http://{}:{}", btc_cli_ip, btc_cli_port);
    Client::new(&btc_cli_url, Auth::UserPass(btc_cli_username, btc_cli_password)).unwrap()
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

pub fn bytes_to_u64(bytes: &Vec<u8>) -> u64 {
    if bytes.len() > 8 {
        panic!("Input exceeds u64 size")
    }
    let mut varint = 0;
    for byte in bytes {
        varint <<= 8;
        varint |= *byte as u64;
    }
    return varint;
}
