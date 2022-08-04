use bitcoincore_rpc::{Auth, Client};

pub fn client() -> Client {
    let btc_cli_ip = std::env::var("BTC_CLI_IP").expect("BTC_CLI_IP env var not set.");
    let btc_cli_port = std::env::var("BTC_CLI_PORT").expect("BTC_CLI_PORT env var not set.");
    let btc_cli_username =
        std::env::var("BTC_CLI_USERNAME").expect("BTC_CLI_USERNAME env var not set");
    let btc_cli_password =
        std::env::var("BTC_CLI_PASSWORD").expect("BTC_CLI_PASSWORD env var not set");
    let btc_cli_url = format!("http://{}:{}", btc_cli_ip, btc_cli_port);
    Client::new(
        &btc_cli_url,
        Auth::UserPass(btc_cli_username, btc_cli_password),
    )
    .unwrap()
}
