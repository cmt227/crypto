#![feature(convert)]

extern crate crypto;
use std::env;
use crypto::converter::*;
use crypto::set1::*;
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
    println!("Running challenge {}", ch);
    match ch.as_str() {
        // Challenge 1 -- one arg: a hex encoded string.
        "1.1" => {
            let s1: Hex = match Hex::from_string(args[2].clone()) {
                Ok(res) => res,
                Err(_) => panic!("Error!"),
            };
            let s2: Base64 = match Base64::from_string(args[3].clone()) {
                Ok(res) => res,
                Err(_) => panic!("ERROR"),
            };
            println!("Input1 (Hex): {}\nInput2 (Base64): {}", s1.as_string(), s2.as_string());
            let b1 = s1.as_vec();
            let b2 = s2.as_vec();
            println!("Input1 (ascii): {}\nInput2 (ascii): {}", String::from_utf8(b1).unwrap(), String::from_utf8(b2).unwrap());
            let b1 = s1.as_vec();
            let b2 = s2.as_vec();
            println!("Input1 as Base64: {}", Base64::from_vec(b1).as_string());
            println!("Input2 as Hex: {}", Hex::from_vec(b2).as_string());
        },
        // Challenge 2 -- two args: both hex encoded strings (of equal length!)
        "1.2" => {
            let s1 = match Hex::from_string(args[2].clone()) {
                Ok(res) => res,
                Err(_) => panic!("Error on input 1"),
            };
            let s2 = match Hex::from_string(args[3].clone()) {
                Ok(res) => res,
                Err(_) => panic!("Error in input 2"),
            };
            let b1 = s1.as_vec();
            let b2 = s2.as_vec();
            let res = Hex::from_vec( fixed_xor(&b1, &b2) );
            println!("Program successful!\nHex encoding of fixed-xor of arguments: {}", res.as_string());
        },
        // Challenge 3 -- one argument: a single-byte xor cipher in hex to be decoded.
        "1.3" => {
            let s1 = match Hex::from_string(args[2].clone()) {
                Ok(res) => res,
                Err(_) => panic!("Error on input"),
            };
            let b1 = s1.as_vec();
            let msg = String::from("ETAOIN SHRDLU");
            let msg_bytes = msg.into_bytes();
            println!("Attemping to decode single-byte xor cipher into English plaintext...");
            let res = single_byte_xor_decode(&b1);
            println!("Program successful!");
            println!("Decoded string: {}\nRank: {}\nByte: {}", res.2, res.0, res.1);
            let dec = xor_one_byte(&msg_bytes, &res.1);
            println!("msg decoded with byte yields: {}", String::from_utf8(dec).unwrap());
        },
        // Challenge 4
        "1.4" => {
            let path = Path::new("inputs/1.4.txt");
            let file = File::open(path).unwrap();
            let reader: BufReader<File> = BufReader::new(file);
            let mut i = 0;
            let mut rank = 0;
            for l in reader.lines() {
                i += 1;
                let line: Hex = match Hex::from_string(l.unwrap()) {
                    Ok(res) => res,
                    Err(_) => panic!("Error reading line {}", i),
                };
                let res: Vec<u8> = line.as_vec();
                let (r, b, s) = single_byte_xor_decode(&res);
                if r > rank {
                    rank = r;
                    println!("New winner! Line {} decoded: {}", i, s);
                    println!("Rank: {}, Byte: {}\n", r, b);
                }
            }
        },
        // Challenge 5
        "1.5" => {
            let mut bytes1 = Vec::new();
            let path = Path::new("inputs/1.5.txt");
            let file = File::open(path).unwrap();
            let mut reader: BufReader<File> = BufReader::new(file);
            let _ = reader.read_to_end(&mut bytes1);
            // two args are eng plaintext.
            let key = String::from("ICE");;
            let key_bytes = key.into_bytes();
            let enc: Vec<u8> = repeating_xor_encrypt(&bytes1, &key_bytes);
            let res = Hex::from_vec(enc);
            println!("Result: {}", res.as_string());
        }
        // Challenge 6
        "1.6" => {
            let bytes1 = String::from("this is a test").into_bytes();
            let bytes2 = String::from("wokka wokka!!!").into_bytes();
            let dist = hamming_dist(&bytes1, &bytes2);
            println!("Hamming distance between test strings is: {}", dist);
        }
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
