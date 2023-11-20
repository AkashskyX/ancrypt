// dir_walk.rs

use threadpool::ThreadPool;
use std::sync::mpsc::{ self};
use std::fs::{File, self};
use std::io::{self , Read , Write};
use std::path::Path;
use walkdir::WalkDir;


use super::encrypt::xor_encrypt;
use super::chunk_encrypt::xor_encrypt_chunk;



pub fn walk_dir_decrypt(source: &str, target: &str, key: &[u8]) -> io::Result<()> {
    let pool = ThreadPool::new(10);  // Adjust the number of threads as necessary

    if !Path::new(target).exists() {
        fs::create_dir_all(target)?;
    }

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            println!("Decrypting file: {}", entry_path.display());
            let mut file = File::open(entry_path)?;
            let chunk_size = 1024000; // Adjust the chunk size as necessary
            let mut buffer = vec![0; chunk_size];
            let mut index = 0;

            let (tx, rx) = mpsc::channel::<(usize, Vec<u8>)>();
            let relative_path = entry_path.strip_prefix(source)
                              .unwrap_or_else(|_| Path::new(""))
                              .to_path_buf();
            let new_path = Path::new(target).join(&relative_path);

            while let Ok(bytes_read) = file.read(&mut buffer) {
                if bytes_read == 0 { break; }
                let chunk = buffer[..bytes_read].to_vec();
                let tx_clone = tx.clone();
                let key_clone = key.to_vec();
                let current_index = index;
                let current_offset = current_index * chunk_size;

                pool.execute(move || {
                    let processed_chunk = xor_encrypt_chunk(&chunk, &key_clone, current_offset);
                    tx_clone.send((current_index, processed_chunk)).expect("Channel send error");
                });

                index += 1;
            }

            drop(tx); // Close the channel for this file

            let mut results: Vec<(usize, Vec<u8>)> = rx.iter().collect();
            results.sort_by_key(|&(index, _)| index);

            let mut output_file = File::create(&new_path)?;

            for (_, chunk) in results {
                output_file.write_all(&chunk)?;
            }
        }
    }

    pool.join(); // Wait for all threads to finish

    Ok(())
}













pub fn walk_dir_encrypt(source: &str, target: &str, key: &[u8]) -> io::Result<()> {
    let pool = ThreadPool::new(10);  // Adjust the number of threads as necessary

    if !Path::new(target).exists() {
        fs::create_dir_all(target)?;
    }

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            println!("encrypting file: {}", entry_path.display());
            let mut file = File::open(entry_path)?;
            let chunk_size = 1024000; // Adjust the chunk size as necessary
            let mut buffer = vec![0; chunk_size];
            let mut index = 0;

            let (tx, rx) = mpsc::channel::<(usize, Vec<u8>)>();
            let relative_path = entry_path.strip_prefix(source)
                              .unwrap_or_else(|_| Path::new(""))
                              .to_path_buf();
            let new_path = Path::new(target).join(&relative_path);

            while let Ok(bytes_read) = file.read(&mut buffer) {
                if bytes_read == 0 { break; }
                let chunk = buffer[..bytes_read].to_vec();
                let tx_clone = tx.clone();
                let key_clone = key.to_vec();
                let current_index = index;
                let current_offset = current_index * chunk_size;

                pool.execute(move || {
                    let processed_chunk = xor_encrypt_chunk(&chunk, &key_clone, current_offset);
                    tx_clone.send((current_index, processed_chunk)).expect("Channel send error");
                });

                index += 1;
            }

            drop(tx); // Close the channel for this file

            let mut results: Vec<(usize, Vec<u8>)> = rx.iter().collect();
            results.sort_by_key(|&(index, _)| index);

            let mut output_file = File::create(&new_path)?;

            for (_, chunk) in results {
                output_file.write_all(&chunk)?;
            }
        }
    }

    pool.join(); // Wait for all threads to finish

    Ok(())
}

