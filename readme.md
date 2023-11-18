# ANCrypt

ANCrypt is a powerful file encryption and decryption tool, built with Rust for performance and reliability. 

** binary only supports macos as of now

## Features

🔐 **Strong Encryption**: Secure your files using XOR encryption.

🔓 **Easy Decryption**: Quickly decrypt your files with the same ease.

📂 **Directory Support**: Encrypt and decrypt entire directories.

## Installation

### Download the Binary

For macOS and Linux users:

1. Download the latest binary from the [Releases page](https://github.com/AkashskyX/ancrypt/blob/main/release/ancrypt).
2. Make the binary executable: `chmod +x /path/to/ancrypt`
3. Move it to a directory in your PATH, e.g., `mv /path/to/ancrypt /usr/local/bin/`

### Build from Source

Alternatively, you can build ANCrypt from source:

```bash
git clone https://github.com/akashSkyX/ancrypt.git
cd ancrypt
cargo build --release
```



## Usage

- `-m d`: Mode set to decryption.
- `-k your_key`: Your decryption key.
- `-s /path/to/encrypted_source`: Source directory containing files to decrypt.
- `-t /path/to/decrypted_target`: (Optional) Target directory to store decrypted files. Defaults to `decrypted_files` in the source directory.


### Encrypting Files

```bash
ancrypt -m e -k your_key -s /path/to/source -t /path/to/target
```

This command encrypts all files in /path/to/source using your_key and stores the encrypted files in /path/to/target.


### Decrypting Files 

```bash
ancrypt -m d -k your_key -s /path/to/encrypted_source -t /path/to/decrypted_target]
```



This command decrypts all files in /path/to/encrypted_source using your_key and stores the decrypted files in /path/to/decrypted_target

### Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.


### License
Distributed under the MIT License. See LICENSE for more information.

