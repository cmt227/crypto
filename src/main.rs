#![feature(slice_extras)]
#![feature(vec_push_all)]

extern crate crypto;

use crypto::converter::hex::*;
use crypto::crypto::set1::*;
use std::env;

fn main() {
    // all arguments should be hex encoded
    let args: Vec<String> = env::args().collect();
    // first arg is the program name
    if args.len() <= 1 {
        panic!("no argument to convert :(");
    }/* else {
        // There is at least one argument 
        s1 = args[1].clone();
        bytes1 = s1.bytes_of_hex();
    }
    // collect some more arguments if they exist
    if args.len() > 3 {
        s2 = args[2].clone();
        bytes2 = s2.bytes_of_hex();
    }
    if args.len() > 4 {
        s3 = args[3].clone();
        bytes3 = s3.bytes_of_hex();
    }*/
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
    // Think im starting to get a handle on the ownership & borrowing stuff!

    let s1: String = args[1].clone();
    let s1_64: String = base64_of_hex(&s1);
    let bytes1: Vec<u8> = s1.bytes_of_hex();
    assert_eq!(bytes1, bytes_of_base64(&s1_64));
    assert_eq!(s1_64, base64_of_bytes(&bytes1));
    println!("Input string in base64: {}", s1_64);
/*    
    let s1: String = args[1].clone();
    let bytes1 = bytes_of_base64(&s1);
    let res = xor_one_byte(&bytes1, &88u8);
    println!("decoded string: {}", base64_of_bytes(&res));

    let s2: String = args[2].clone();
    let s2_64 = base64_of_hex(&s2);
    let bytes2 = s2.bytes_of_hex();
    println!("other string in base64: {}", s2_64);
 
    let res = fixed_xor(&bytes1, &bytes2);
    println!("fixed_xor of the two byte strings(printed in hex): {}", res.hex_of_bytes());
*/
    let mut rank = 0;
    for byte in 0..255 {
        let res = xor_one_byte(&bytes1, &byte);
        if rank_string(&res) > rank {
            rank = rank_string(&res);
            println!("Found new winner: rank: {}, byte: {}, string: {}", rank, byte, base64_of_bytes(&res));
        }
    }
    println!("Program successful!");
    //println!("Result: {}", sol);
    //println!("Byte: {}", b);
    //println!("Rank: {}", rank);
}

// Returns an integer representing the 'rank' of a string.
// attempt 1: rank == number of vowels in the string.
fn rank_string(s: &Vec<u8>) -> i32 {
    num_eng_chars(s)
}


// Returns the number of vowels in the byte vector s
fn num_eng_chars(s: &Vec<u8>) -> i32 {
    if s.len() == 0 {
        0
    } else {
        let mut v = Vec::new();
        v.push_all(s.tail());
        match *s.first().unwrap() {
            b'a'...b'z' | b'A'...b'Z' | b' ' => {
                1 + num_eng_chars(&v)
            },
            _ => num_eng_chars(&v)
        }
    }
}
