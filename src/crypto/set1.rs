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
