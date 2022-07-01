use btc_tx_parser::BtcTxParser;

#[test]
fn test_version_number() {
    //working with transaction b5c8d30d5bae5b0abc44ee38816dc8c6bcb7de63ecfda3ba6099d30b4e650f46
    let raw_tx_hex = "01000000022212143fd9ff6faed3fc76236937551e6546336bb857c15336f8db5547704c23000000006a473044022064263b2af280ffddbefbb136b47b4c40b92befbff0211167b47cd89df0ab894d02206b1c7bad71a2e4feefa2c0f062de7537a23ebb23f95edcbbc514347b356701b9012102cefe6dd8b42642d6a4e9b2d9cbbd9a32aea7476f5452d0b9df8b84deff9d4188ffffffff194597582fccf560f601cb1815bfd56806c4e18c42bd44bccb61f02fe0a29585000000006a47304402202d7ff142e30cd88b43ca7b51b7a8581e1f861827d996385554ced66d5bdbca270220320d3d460bdbd1d22c09b221e11db575603355aedac332ac3d558154d654b1f7012103bc6bf57983c31cdd5f3ac9912361460a2a96b3a667c958a150eb05fefab118b1ffffffff019f5b9300000000001976a914fd2d6215f93e43c5e19caf454ba7f4f943b9cf5288ac00000000";
    let t = BtcTxParser::new(raw_tx_hex);
    println!("working with {:?}", t);
    assert_eq!(t.version_number, 1);
}
