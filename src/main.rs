// main.rs
use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
mod encrypt;
mod chunk_encrypt;
mod dir_walk;
use dir_walk::{walk_dir_encrypt, walk_dir_decrypt};
use std::time::Instant;

fn main() -> io::Result<()> {
    let matches = App::new("🔐 ANCrypt")
        .version("1.0.1")
        .author("Sky. <akash@collectivai.com>")
        .about("Encrypts and Decrypts your files with style! ✨ V2 with multithreading 🔥 ")
        .arg(Arg::new("mode")
             .short('m')
             .long("mode")
             .value_name("MODE")
             .help("Operation mode: 'e' for encrypt, 'd' for decrypt")
             .takes_value(true)
             .required(true))
        .arg(Arg::new("key")
             .short('k')
             .long("key")
             .value_name("KEY")
             .help("Encryption/Decryption key")
             .takes_value(true)
             .required(true))
        .arg(Arg::new("source")
             .short('s')
             .long("source")
             .value_name("SOURCE")
             .help("Source directory for files")
             .takes_value(true)
             .required(true))
        .arg(Arg::new("target")
             .short('t')
             .long("target")
             .value_name("TARGET")
             .help("Target directory for encrypted/decrypted files")
             .takes_value(true)
             .required(false))
        .get_matches();

    // Parsing command line arguments
    let mode = matches.value_of("mode").unwrap();
    let key = matches.value_of("key").unwrap().as_bytes();
    let source = matches.value_of("source").unwrap();
    let mut target = matches.value_of("target").unwrap_or("encrypted_files");

    // Ensure target directory exists
    if !Path::new(target).exists() {
        fs::create_dir(target)?;
    }

    // Perform encryption or decryption
    match mode {
        "e" => {
            let start = Instant::now();
            println!("🔒 Encrypting files in '{}'...", source);
            walk_dir_encrypt(source, target, key)?;
            println!("time taken {:?}" , start.elapsed());
            println!("✅ Encryption complete! Files saved in '{}'", target);
        },
        "d" => {
            let start = Instant::now();
            println!("🔓 Decrypting files in '{}'...", source);
            target = "decrypted_file";
            walk_dir_decrypt(source, target, key)?;
            println!("time taken {:?}" , start.elapsed());
            println!("✅ Decryption complete! Files saved in '{}'", target);
        },
        _ => {
            writeln!(io::stderr(), "❌ Invalid mode! Use 'e' for encrypt or 'd' for decrypt.")?;
            std::process::exit(1);
        }
    }

    Ok(())
}
