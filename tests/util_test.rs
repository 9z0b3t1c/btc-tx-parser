use btc_tx_parser::util;

#[test]
fn test_max_bytes_to_u64() {
    let v = vec![255,255,255,255,255,255,255,255];
    let r = util::bytes_to_u64(&v);
    assert_eq!(r, u64::MAX);
}

#[test]
fn test_min_bytes_to_u64() {
    let v = vec![0];
    let r = util::bytes_to_u64(&v);
    assert_eq!(r, u64::MIN);
}

