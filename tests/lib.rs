extern crate crypto;

use crypto::converter::hex::*;
use std::string::String;

#[test]
fn hex_convertions() {
    let pairs = [
        ("12", &vec![18]),
        ("1a2", &vec![1, 162]),
        ("f562c7ab", &vec![245, 98, 199, 171]),
        ("abcdef12345", &vec![10, 188, 222, 241, 35, 69])
    ];
    for &(h, v) in pairs.iter() {
        let hex = String::from(h);
        let bytes: Vec<u8> = hex.bytes_of_hex();
        let s: String = v.hex_of_bytes();
        assert_eq!(bytes, *v);
        assert_eq!(s, hex);
    }
}

#[test]
fn test() {
    assert_eq!(0123, 123);
}
