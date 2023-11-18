



pub fn xor_encrypt_chunk(chunk: &[u8], key: &[u8], offset: usize) -> Vec<u8> {
   // println!("{:?} ", &chunk);
    chunk.iter().enumerate().map(|(i, &b)| b ^ key[(offset + i) % key.len()]).collect()

   
}
