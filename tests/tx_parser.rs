use btc_tx_parser::builder;
use btc_tx_parser::parser;
use btc_tx_parser::util;
use btc_tx_parser::BtcTx;
use btc_tx_parser::Input;
use btc_tx_parser::Output;

use bitcoin::hashes::hex::FromHex;
use bitcoin::Txid;
use bitcoincore_rpc::{bitcoin, RpcApi};

//https://blockstream.info/tx/b5c8d30d5bae5b0abc44ee38816dc8c6bcb7de63ecfda3ba6099d30b4e650f46
fn random_tx() -> BtcTx {
    BtcTx {
        version_number: 1,
        txid: "b5c8d30d5bae5b0abc44ee38816dc8c6bcb7de63ecfda3ba6099d30b4e650f46".to_string(),
        transaction_fee: 17952,
        locktime: 0,
        size: 338,
        vsize: 338,
        weight: 1352,
        inputs: [
            Input {
                txid: "234c704755dbf83653c157b86b3346651e5537692376fcd3ae6fffd93f141222".to_string(),
                vout: 0,
                script_sig: "473044022064263b2af280ffddbefbb136b47b4c40b92befbff0211167b47cd89df0ab894d02206b1c7bad71a2e4feefa2c0f062de7537a23ebb23f95edcbbc514347b356701b9012102cefe6dd8b42642d6a4e9b2d9cbbd9a32aea7476f5452d0b9df8b84deff9d4188".to_string(),
                sequence: 4294967295,
            }, Input {
                txid: "8595a2e02ff061cbbc44bd428ce1c40668d5bf1518cb01f660f5cc2f58974519".to_string(),
                vout: 0,
                script_sig: "47304402202d7ff142e30cd88b43ca7b51b7a8581e1f861827d996385554ced66d5bdbca270220320d3d460bdbd1d22c09b221e11db575603355aedac332ac3d558154d654b1f7012103bc6bf57983c31cdd5f3ac9912361460a2a96b3a667c958a150eb05fefab118b1".to_string(),
                sequence: 4294967295,
            },
        ].to_vec(),
        outputs: [Output {
            amount: 9657247,
            script_pub_key: "76a914fd2d6215f93e43c5e19caf454ba7f4f943b9cf5288ac".to_string(),
        },].to_vec(),
    }
}

//https://blockstream.info/tx/f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16
fn first_ever_tx() -> BtcTx {
    BtcTx {
        version_number: 1,
        txid: "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16".to_string(),
        transaction_fee: 0,
        locktime: 0,
        size: 275,
        vsize: 275,
        weight: 1100,
        inputs: [
            Input {
                txid: "0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9".to_string(),
                vout: 0,
                script_sig: "47304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901".to_string(),
                sequence: 4294967295,
            },
        ].to_vec(),
        outputs: [Output {
            amount: 1000000000,
            script_pub_key: "4104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac".to_string(),
        }, Output {
            amount: 4000000000,
            script_pub_key: "410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac".to_string(),
        }
        ].to_vec(),
    }
}

//https://blockstream.info/tx/ea44e97271691990157559d0bdd9959e02790c34db6c006d779e82fa5aee708e
fn second_tx() -> BtcTx {
    BtcTx {
        version_number: 1,
        txid: "ea44e97271691990157559d0bdd9959e02790c34db6c006d779e82fa5aee708e".to_string(),
        transaction_fee: 0,
        locktime: 0,
        size: 157,
        vsize: 157,
        weight: 628,
        inputs: [
            Input {
                txid: "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16".to_string(),
                vout: 0,
                script_sig: "4730440220576497b7e6f9b553c0aba0d8929432550e092db9c130aae37b84b545e7f4a36c022066cb982ed80608372c139d7bb9af335423d5280350fe3e06bd510e695480914f01".to_string(),
                sequence: 4294967295,
            },
        ].to_vec(),
        outputs: [Output {
            amount: 1000000000,
            script_pub_key: "76a914340cfcffe029e6935f4e4e5839a2ff5f29c7a57188ac".to_string(),
        },
        ].to_vec(),
    }
}

//https://blockstream.info/tx/92a78def188053081187b847b267f0bfabf28368e9a7a642780ce46a78f551ba
fn more_secret_tx() -> BtcTx {
    BtcTx {
        version_number: 1,
        txid: "92a78def188053081187b847b267f0bfabf28368e9a7a642780ce46a78f551ba".to_string(),
        transaction_fee: 103000,
        locktime: 0,
        size: 653,
        vsize: 653,
        weight: 2612,
        inputs: [
            Input {
                txid: "bc7530978073c78fbb0e020a503748130f5e10690a752eb794f6d87dd096988b".to_string(),
                vout: 0,
                script_sig: "47304402206213230eddf32c60167e654e3934602c0e46308932ea344a0e242699c1818f51022044895b0fc7adef9e551777d0de789d508fb56785ca80fbbfeec01b9d07b4fb7901410450128ec8ff327d0cd782702a32f51b14149d8a19b89075a56ead462363fa29ae9b35ca4f71ae8d5cd78803d835d05451ebb3ee861c80746f0e4fd5316ec306a7".to_string(),
                sequence: 4294967295,
            }, Input {
                txid: "461af0f9c71cefe13db48b3dc396834cc19b0624b08aee7420a5f356e91c4992".to_string(),
                vout: 0,
                script_sig: "48304502207fec947609bd275a32cfd058c76678fe868c12b681c9ab0c31f716a92ba5fed0022100cd95a9ff2036a7ee0babe268ac64b425b4490be36609452ec01c11b8eaf14665014104b5a08389cbbf01178c5451f9e1c09265e73ef7bc4a1bc6761143593134e5c6460ab31ae2d5f09140a5e95a58538fd4651cb966a86de41c1a6a79b6045ecc0312".to_string(),
                sequence: 4294967295,
            }, Input {
                txid: "a1d13badbaa7ea88a1ff5a347d7b715131dcde7616ce7025876e91e75d84a33c".to_string(),
                vout: 0,
                script_sig: "483045022100a53211eed0e857dfab414f106190780c3791797b81aaf5a8a8f997681f6ea660022030a00ef0733bafa5f05026e027ac6f230c3929f9c766ef31edeabf2bcaed81740121036ec01e60571b5050bafb2d77063061a487228da342e996003e35ac7b5519e2e7".to_string(),
                sequence: 4294967295,
            },
        ].to_vec(),
        outputs: [ Output {
            amount: 18230926,
            script_pub_key: "76a9142b18e0074aad70661b6fecf742cbefab4a145d1188ac".to_string(),
        }, Output {
            amount: 1000000,
            script_pub_key: "76a914a229e570ef0c11b6a20451d65047b0fbe2c96a2f88ac".to_string(),
        }, Output {
            amount: 1000000,
            script_pub_key: "76a91408536923b85945c704b47bb2657294757bc417dc88ac".to_string(),
        }, Output {
            amount: 1000000,
            script_pub_key: "76a91415c307a88533528de8414fc2fc96b4e67ac0e0ef88ac".to_string(),
        },
        ].to_vec(),
    }
}

#[test]
fn test_parse_first_ever_transaction() {
    let t = Txid::from_hex(&"f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16")
        .unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    let mut r = parser::parse(&rth);
    let actual = first_ever_tx();
    assert_eq!(r.version_number, actual.version_number);
    assert_eq!(r.txid, actual.txid);
    assert_eq!(r.locktime, actual.locktime);
    assert_eq!(r.size, actual.size);
    assert_eq!(r.weight, actual.weight);
    assert_eq!(r.get_transaction_fee(), actual.transaction_fee);
    for (index, input) in r.inputs.iter().enumerate() {
        let actual_input = &actual.inputs[index];
        assert_eq!(input.txid, actual_input.txid);
        assert_eq!(input.vout, actual_input.vout);
        assert_eq!(input.script_sig, actual_input.script_sig);
        assert_eq!(input.sequence, actual_input.sequence);
    }
    for (index, output) in r.outputs.iter().enumerate() {
        let actual_output = &actual.outputs[index];
        assert_eq!(output.amount, actual_output.amount);
        assert_eq!(output.script_pub_key, actual_output.script_pub_key);
    }
}

#[test]
fn test_parse_second_tx() {
    let t = Txid::from_hex(&"ea44e97271691990157559d0bdd9959e02790c34db6c006d779e82fa5aee708e")
        .unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    let mut r = parser::parse(&rth);
    let actual = second_tx();
    assert_eq!(r.version_number, actual.version_number);
    assert_eq!(r.txid, actual.txid);
    assert_eq!(r.locktime, actual.locktime);
    assert_eq!(r.size, actual.size);
    assert_eq!(r.weight, actual.weight);
    assert_eq!(r.get_transaction_fee(), actual.transaction_fee);
    for (index, input) in r.inputs.iter().enumerate() {
        let actual_input = &actual.inputs[index];
        assert_eq!(input.txid, actual_input.txid);
        assert_eq!(input.vout, actual_input.vout);
        assert_eq!(input.script_sig, actual_input.script_sig);
        assert_eq!(input.sequence, actual_input.sequence);
    }
    for (index, output) in r.outputs.iter().enumerate() {
        let actual_output = &actual.outputs[index];
        assert_eq!(output.amount, actual_output.amount);
        assert_eq!(output.script_pub_key, actual_output.script_pub_key);
    }
}

#[test]
fn test_parse_more_secret_tx() {
    let t = Txid::from_hex(&"92a78def188053081187b847b267f0bfabf28368e9a7a642780ce46a78f551ba")
        .unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    let mut r = parser::parse(&rth);
    let actual = more_secret_tx();
    assert_eq!(r.version_number, actual.version_number);
    assert_eq!(r.txid, actual.txid);
    assert_eq!(r.locktime, actual.locktime);
    assert_eq!(r.size, actual.size);
    assert_eq!(r.weight, actual.weight);
    assert_eq!(r.get_transaction_fee(), actual.transaction_fee);
    for (index, input) in r.inputs.iter().enumerate() {
        let actual_input = &actual.inputs[index];
        assert_eq!(input.txid, actual_input.txid);
        assert_eq!(input.vout, actual_input.vout);
        assert_eq!(input.script_sig, actual_input.script_sig);
        assert_eq!(input.sequence, actual_input.sequence);
    }
    for (index, output) in r.outputs.iter().enumerate() {
        let actual_output = &actual.outputs[index];
        assert_eq!(output.amount, actual_output.amount);
        assert_eq!(output.script_pub_key, actual_output.script_pub_key);
    }
}

#[test]
fn test_parse_random_class_transaction() {
    let t = Txid::from_hex(&"b5c8d30d5bae5b0abc44ee38816dc8c6bcb7de63ecfda3ba6099d30b4e650f46")
        .unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    let r = parser::parse(&rth);
    let actual = random_tx();
    assert_eq!(r.version_number, actual.version_number);
    assert_eq!(r.txid, actual.txid);
    assert_eq!(r.locktime, actual.locktime);
    assert_eq!(r.size, actual.size);
    assert_eq!(r.weight, actual.weight);
    //TODO one of the inputs was a segwit transaction
    // assert_eq!(r.get_transaction_fee(), actual.transaction_fee);
    for (index, input) in r.inputs.iter().enumerate() {
        let actual_input = &actual.inputs[index];
        assert_eq!(input.txid, actual_input.txid);
        assert_eq!(input.vout, actual_input.vout);
        assert_eq!(input.script_sig, actual_input.script_sig);
        assert_eq!(input.sequence, actual_input.sequence);
    }
    for (index, output) in r.outputs.iter().enumerate() {
        let actual_output = &actual.outputs[index];
        assert_eq!(output.amount, actual_output.amount);
        assert_eq!(output.script_pub_key, actual_output.script_pub_key);
    }
}

#[test]
fn test_build_first_ever_transaction() {
    let candidate = first_ever_tx();
    let r = builder::build(&candidate);
    let t = Txid::from_hex(&candidate.txid).unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    assert_eq!(r, rth);
}

#[test]
fn test_build_second_tx() {
    let candidate = second_tx();
    let r = builder::build(&candidate);
    let t = Txid::from_hex(&candidate.txid).unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    assert_eq!(r, rth);
}

#[test]
fn test_build_random_tx() {
    let candidate = random_tx();
    let r = builder::build(&candidate);
    let t = Txid::from_hex(&candidate.txid).unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    assert_eq!(r, rth);
}

#[test]
fn test_build_more_secret_tx() {
    let candidate = more_secret_tx();
    let r = builder::build(&candidate);
    let t = Txid::from_hex(&candidate.txid).unwrap();
    let rth = util::client().get_raw_transaction_hex(&t, None).unwrap();
    assert_eq!(r, rth);
}
