extern crate crypto;

use crypto::converter::hex::*;
use crypto::crypto::set1::fixed_xor;
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
    let s1: String = args[1].clone();
    let s2: String = args[2].clone();
    let bytes1 = s1.bytes_of_hex();
    let bytes2 = s2.bytes_of_hex();
    let res = fixed_xor(bytes1, bytes2).hex_of_bytes();
    println!("Program successful!\nResult: {}", res);
}
