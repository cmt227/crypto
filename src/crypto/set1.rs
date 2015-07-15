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
        println!("{:?}", xor);
        let r = rank_string(&xor);
        if r > res.0 {
            let s = match String::from_utf8(xor) {
                Err(e) => {
                    println!("oops! error: {}", e);
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
