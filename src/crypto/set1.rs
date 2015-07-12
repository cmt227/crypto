// Precondition: bytes1 and bytes2 are of equal lengths.
pub fn fixed_xor(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Vec<u8> {
    if bytes1.len() != bytes2.len() {
        panic!("Unequal length arguments!");
    }
    let mut res = Vec::with_capacity(bytes1.len());
    for i in 0..(bytes1.len()) {
        res.push(bytes1[i] ^ bytes2[i]);
    }
    res
}

// Produce the vector resulting from the xor of 
// each byte in vec with the single byte b.
pub fn xor_one_byte(vec: Vec<u8>, b: u8) -> Vec<u8> {
    let mut v: Vec<u8> = vec.clone();
    for i in 0..v.len() {
        v[i] = b;
    }

    fixed_xor(vec, v)
}
