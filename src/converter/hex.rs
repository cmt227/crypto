extern crate rustc_serialize;

use std::string::String;
use self::rustc_serialize::base64::*;

pub fn base64_of_hex(hex: String) -> String {
    let config = Config {
        char_set: Standard,
        newline: Newline::LF,
        pad: false,
        line_length: None,
    };

    let bytes = hex.bytes_of_hex();
    bytes.to_base64(config)
}

pub trait FromHex {
    fn bytes_of_hex(&self) -> Vec<u8>;
}

impl FromHex for String {
    fn bytes_of_hex(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.len() / 2);
        let mut buf: u8 = 0;
        let mut modulus = 0;
        // if there are an uneven number of characters,
        // increment modulus to simulate a leading zero.
        if self.len() % 2 != 0 {
            modulus = 1;
        }
        for byte in self.bytes() {
            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                _ => panic!("fuck"),
            }
            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                res.push(buf);
            }
            buf <<= 4;
        }
        res
    }
}

pub trait ToHex {
    fn hex_of_bytes(&self) -> String;
}

impl ToHex for Vec<u8> {
    fn hex_of_bytes(&self) -> String {
        let mut res = Vec::with_capacity(self.len() * 2);
        for byte in self.iter() {
            let upper: u8 = (byte & 240u8) >> 4;
            let lower: u8 = byte & 15u8;
            res.push(bits_to_hex_char(&upper));
            res.push(bits_to_hex_char(&lower));
        }
        if *res.first().unwrap() == b'0' {
            res.remove(0);
        }
        String::from_utf8(res).unwrap()
    }
}

fn bits_to_hex_char(bits: &u8) -> u8 {
    match *bits {
        0...9 => bits + b'0',
        10...15 => bits + b'a' - 10,
        _ => 16,
    }
}
