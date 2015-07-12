#![feature(slice_extras)]
#![feature(vec_push_all)]

extern crate crypto;

use crypto::converter::hex::*;
use crypto::crypto::set1::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no argument to convert :(");
    }
    /* challenge 1 
    let s: String = args[1].clone();
    let res = base64_of_hex(s);
    println!("Program successful.\nResult: {}", res);
    */
    /* challenge 2
    let s1: String = args[1].clone();
    let s2: String = args[2].clone();
    let bytes1 = s1.bytes_of_hex();
    let bytes2 = s2.bytes_of_hex();
    let res = fixed_xor(bytes1, bytes2).hex_of_bytes();
    println!("Program successful!\nResult: {}", res);
    */
    // challenge 3
    // takes 1 arg, the single-byte xor encoded string.
    let mut s: String = args[1].clone();
    let s = base64_of_hex(s);
    println!("Input string in base64: {}", s);
    let mut rank = 0;
    let mut b: u8 = 0;
    let mut sol: String = String::new();

    for byte in 0u8..255u8 {
        let bytes1 = s.clone().into_bytes();
        let res = xor_one_byte(bytes1, byte);
        if rank_string(&res) > rank {
            rank = rank_string(&res);
            b = byte;
            sol = String::from_utf8(res).unwrap();
        }
    }
    println!("Program successful!");
    println!("Result: {}", sol);
    println!("Byte: {}", b);
    println!("Rank: {}", rank);
}

// Returns an integer representing the 'rank' of a string.
// attempt 1: rank == number of vowels in the string.
fn rank_string(s: &Vec<u8>) -> i32 {
    num_vowels(s)
}

// Returns the number of vowels in the byte vector s
fn num_vowels(s: &Vec<u8>) -> i32 {
    if s.len() == 0 {
        0
    } else {
        let mut v = Vec::new();
        v.push_all(s.tail());
        match *s.first().unwrap() {
            b'a'|b'e'|b'i'|b'o'|b'u'|
            b'A'|b'E'|b'I'|b'O'|b'U'  => {
                1 + num_vowels(&v)
            },
            _ => num_vowels(&v)
        }
    }
}
