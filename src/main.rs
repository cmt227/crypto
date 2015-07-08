extern crate rustc_serialize;
use rustc_serialize::hex::FromHex;
use rustc_serialize::base64::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no argument to convert :(");
    }
    let s: String = args[1].clone();
    let res = base64_of_hex(s);
    println!("Program successful.\nResult: {}", res);
}

fn base64_of_hex(hex: String) -> String {
    let config = Config {
        char_set: Standard,
        newline: Newline::LF,
        pad: false,
        line_length: None,
    };

    let bytes = hex.from_hex().unwrap();
    bytes.to_base64(config)
}
/*
trait HexConverter {
    fn bytes_of_hex(&self) -> Vec<u8>;
}

impl HexConverter for str {
    fn bytes_of_hex(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.len() / 2);
        let mut modulus = 0;
        let mut buf: u8 = 0;
        for byte in self.bytes() {
            buf <<= 4;
            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' ' | b'\r' | b'\n' | b'\t' => {
                    buf >>= 4;
                    continue
                }
                _ => break,
            }
            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                res.push(buf);
            }
        }
    }
}
*/
