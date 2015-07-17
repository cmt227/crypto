// A crate for representing and converting common crypto values.

extern crate rustc_serialize;

use self::rustc_serialize::hex::{ToHex, FromHex};
use self::rustc_serialize::base64::*;

pub struct Hex {
    num: String,
}

pub enum HexErr {
    InvalidHexChar,
    InvalidLength,
}

impl Hex {
    fn is_hex(s: &String) -> bool {
        for c in s.as_bytes().iter() {
            match *c {
                  b'0'...b'9'
                | b'a'...b'z'
                | b'A'...b'Z' => {},
                _ => { return false },
            }
        }
        true
    }

    pub fn new() -> Hex {
        Hex { num: String::new() }
    }

    pub fn from_string(s: String) -> Result<Hex, HexErr> {
        if !Hex::is_hex(&s) {
            Err(HexErr::InvalidHexChar)
        } else if s.len() % 2 != 0 {
            Err(HexErr::InvalidLength)
        } else {
            Ok(Hex{ num: s })
        }
    }
    
    pub fn from_vec(v: Vec<u8>) -> Hex {
        Hex{ num: v.as_slice().to_hex() }
    }

    /* TODO
    pub fn from_base64(b: Base64) -> Hex {
    }
    */

    pub fn as_vec(&self) -> Vec<u8> {
        self.num.clone().from_hex().unwrap()
    }
    
    pub fn as_string(&self) -> String {
        self.num.clone()
    }
}

pub struct Base64 {
    num: String,
}

pub enum Base64Err {
    InvalidBase64Char,
    InvalidBase64Length,
}

impl Base64 {
    fn is_base64(s: &String) -> bool {
        for c in s.as_bytes().iter() {
            match *c {
                  b'a'...b'z'
                | b'A'...b'Z'
                | b'0'...b'9'
                | b'+'
                | b'/' => {},
                _ => { return false },
            }
        }
        true
    }

    pub fn new() -> Base64 {
        Base64 { num: String::new() }
    }

    pub fn from_string(s: String) -> Result<Base64, Base64Err> {
        if !Base64::is_base64(&s) {
            Err(Base64Err::InvalidBase64Char)
        } else if s.len() % 4 == 1 {
            Err(Base64Err::InvalidBase64Length)
        } else {
            // OK if length % 4 == 2, 3, or 0
            Ok(Base64 { num: s })
        }
    }

    pub fn from_vec(v: Vec<u8>) -> Base64 {
        let config = Config {
            char_set: Standard,
            newline: Newline::LF,
            pad: false,
            line_length: None,
        };
        Base64{ num: v.as_slice().to_base64(config) }
    }

    pub fn as_vec(&self) -> Vec<u8> {
        self.num.clone().from_base64().unwrap()
    }

    pub fn as_string(&self) -> String {
        self.num.clone()
    }
}


pub fn from_hex(hex: &String) -> Vec<u8> {
    match hex.from_hex() {
        Ok(v) => v,
        Err(e) => {
            println!("Error converting value from hex.. {}", e);
            println!("Returning empty vector..");
            Vec::new()
        },
    }
}

pub fn to_hex(s: &Vec<u8>) -> String {
    s.to_hex()
}

pub fn from_base64(b64: &String) -> Vec<u8> {
    match b64.from_base64() {
        Ok(v) => v,
        Err(e) => {
            println!("Error converting value from base64.. {}", e);
            println!("Returning empty vector..");
            Vec::new()
        },
    }
}

pub fn to_base64(s: &Vec<u8>) -> String {
    let config = Config {
        char_set: Standard,
        newline: Newline::LF,
        pad: false,
        line_length: None,
    };
    s.to_base64(config)
}

// trying to write using a borrowed string.
pub fn base64_of_hex(hex: &String) -> String {
    let b = from_hex(hex);
    to_base64(&b)
}

#[cfg(test)]
mod test {

    use super::Hex as Hex;

    #[test]
    fn test_hex() {
        let s: String = "1234567890abCDEf".to_string();
        assert!(Hex::is_hex(&s));
        let h: Hex = match Hex::from_string("1234567890abCDEf".to_string()) {
            Ok(res) => res,
            Err(e) => panic!("Error converting from String to Hex"),
        };
        assert_eq!(s, h.as_string());
    }
        

}
