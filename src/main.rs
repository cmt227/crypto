#![feature(convert)]

extern crate crypto;

use std::env;
use crypto::converter::hex::*;
use crypto::crypto::set1::*;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // all arguments should be hex encoded
    let args: Vec<String> = env::args().collect();
    // first arg is the program name
    if args.len() <= 1 {
        panic!("no argument to convert :(");
    }
    let ch = args[1].clone();
    println!("{}", ch);
    match ch.as_str() {
        // Challenge 1 -- one arg: a hex encoded string.
        "1.1" => {
            let s1 = args[2].clone();
            let res = base64_of_hex(&s1);
            println!("Program successful.\nBase64 encoding of input: {}", res);
        },
        // Challenge 2 -- two args: both hex encoded strings (of equal length!)
        "1.2" => {
            let s1 = args[2].clone();
            let bytes1 = s1.bytes_of_hex();
            let s2 = args[3].clone();
            let bytes2 = s2.bytes_of_hex();
            let res = fixed_xor(&bytes1, &bytes2).hex_of_bytes();
            println!("Program successful!\nHex encoding of fixed-xor of arguments: {}", res);
        },
        // Challenge 3 -- one argument: a single-byte xor cipher in hex to be decoded.
        "1.3" => {
            let s1 = args[2].clone();
            let bytes1 = s1.bytes_of_hex();
            let msg = String::from("ETAOIN SHRDLU");
            let msg_bytes = msg.into_bytes();
            println!("Attemping to decode single-byte xor cipher into English plaintext...");
            let res = single_byte_xor_decode(&bytes1);
            println!("Program successful!");
            println!("Decoded string: {}\nRank: {}\nByte: {}", res.2, res.0, res.1);
            let dec = xor_one_byte(&msg_bytes, &res.1);
            println!("msg decoded with byte yields: {}", String::from_utf8(dec).unwrap());
        },
        // Challenge 4
        "1.4" => {
            let mut sol = ( 0, 0u8, String::new() );

            let path = Path::new("inputs/1.4.txt");
            let file = File::open(path).unwrap();
            let reader: BufReader<File> = BufReader::new(file);
            for line in reader.lines() {
                let line: Vec<u8> = line.unwrap().bytes_of_hex();
                println!("{:?}", line);
                /*let (r, b, s) = single_byte_xor_decode(&line);
                if r > sol.0 {
                    sol = (r, b, s);
                }*/
            }
            println!("Highest ranking decoded string: {}", sol.2);
            println!("Rank: {}", sol.0);
            println!("Byte: {}", sol.1);
        },
        _ => println!("Haven't done that challenge yet!")
    }

    /* FizzBuzz!
    for i in 1..101 {
        let d3 = i.is_divisible_by(3);
        let d5 = i.is_divisible_by(5);
        if d3 && d5 {
            println!("FizzBuzz!");
        } else if d3 {
            println!("Fizz");
        } else if d5 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    */
}

trait Divisible {
    fn is_divisible_by(&self, d: i32) -> bool;
}

impl Divisible for i32 {
    fn is_divisible_by(&self, d: i32) -> bool {
        if self % d == 0 {
            true
        } else {
            false
        }
    }
}
