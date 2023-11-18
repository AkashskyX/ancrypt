// encrypt.rs

pub fn xor_encrypt(contents: &[u8], key: &[u8]) -> Vec<u8> {
    contents.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect()
}


