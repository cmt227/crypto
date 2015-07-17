// Precondition: bytes1 and bytes2 are of equal lengths.
pub fn fixed_xor(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> Vec<u8> {
    if bytes1.len() != bytes2.len() {
        panic!("Unequal length arguments!");
    }
    let mut res = Vec::with_capacity(bytes1.len());
    for i in 0..bytes1.len() {
        res.push(bytes1[i] ^ bytes2[i]);
    }
    res
}

// Produce the vector resulting from the xor of 
// each byte in vec with the single byte b.
pub fn xor_one_byte(vec: &Vec<u8>, b: &u8) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::with_capacity(vec.len());
    for _ in 0..v.capacity() {
         v.push(*b);
    }
    fixed_xor(vec, &v)
}

// Attempts to decode a byte array that has been encoded by
// xor with a single byte.
// Assumes English plaintext as the decoded medium.
// returns tuple of (rank: i32, byte: u8, message: String)
pub fn single_byte_xor_decode(bytes: &Vec<u8>) -> (i32, u8, String) {
    let mut res = (0, 0, String::new());
    for byte in 0..255 {
        let xor = xor_one_byte(bytes, &byte);
        let r = rank_string(&xor);
        if r > res.0 {
            let s = match String::from_utf8(xor) {
                Err(_) => {
                    // Occurs if the single-byte xor
                    //doesn't produce valid utf-8.. skip it!
                    continue;
                },
                Ok(res) => res
            };
            res = ( r, byte, s );
        }
    }
    res
}

// Returns an integer representing the 'rank' of a string.
fn rank_string(s: &Vec<u8>) -> i32 {
    num_eng_chars(s)
}

// Returns the number of uppercase and lowercase characters,
// and spaces in the byte vector s.
fn num_eng_chars(s: &Vec<u8>) -> i32 {
    if s.len() == 0 {
        0
    } else {
        let mut v = Vec::new();
        v.push_all(s.tail());
        match *s.first().unwrap() {
            b' ' | b'a'...b'z' | b'A'...b'Z' => {
                1 + num_eng_chars(&v)
            },
            _ => num_eng_chars(&v)
        }
    }
}

// Encrypt b1 via repeating-key-xor with key
pub fn repeating_xor_encrypt(b1: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    // init the resulting vector (same length as input)
    let key_width = key.len();
    let mut res = Vec::with_capacity(b1.len());
    let mut k: Vec<u8>;
    let mut b = Vec::with_capacity(key_width);
    for chunk in b1.chunks(key_width) {
        b.push_all(chunk);
        k = key.clone();
        if chunk.len() < key_width {
            k.split_off(chunk.len());
        }
        res.append(&mut fixed_xor(&b, &k));
        b.clear();
    }
    res
    // TODO impiment with recursion
    /*
    match b1.len() {
        0 => {
            Vec::new()
        },
        _ => {
            let mut hd = b1.clone();
            let tl = res.split_off(key_width);

        },
    }*/
}

pub fn hamming_dist(v1: &Vec<u8>, v2: &Vec<u8>) -> i32 {
    let xor = fixed_xor(v1, v2); // enforces that v1.len() == v2.len()
    let mut sum = 0;
    for i in xor.iter() {
        let diff = ones_in_byte(i);
        sum += diff;
        println!("now sum is {}", sum);
    }
    sum
}

fn ones_in_byte(n1: &u8) -> i32 {
    ones(n1, 128)
}

fn ones(n1: &u8, div: u8) -> i32 {
    // we're considering bytes, so start at 2^7
    if n1 == &0 {
        0
    } else if n1 / div == 0 {
        ones(n1, div/2)
    } else {
        let rem = n1 % div;
        1 + ones(&rem, div/2)
    }
}

#[test]
fn ones_test() {
    assert_eq!(1, ones_in_byte(&128));
    assert_eq!(4, ones_in_byte(&15));
    assert_eq!(8, ones_in_byte(&255));
    assert_eq!(5, ones_in_byte(&87));
}
