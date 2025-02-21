![License](https://img.shields.io/badge/license-MIT-green)  
![Rust](https://img.shields.io/badge/Rust-1.80.1-orange?style=flat-square&logo=rust)

English Version / [日本語版](https://github.com/Atamol/RustyEncryptor/blob/main/README.md)

---

# RustyEncryptor

Looking for a lightning-fast way to lock down your files and texts? RustyEncryptor is a super straightforward AES-256-GCM encryption tool that works from the command line, so you can encrypt and decrypt your stuff in a snap!

## Features
- Speedy AES-256-GCM encryption and decryption
- Simple command-line interface (no headaches!)
- Secure file and text encryption

## Dependencies

```toml
[dependencies]
clap = "4.1"
ring = "0.16"
base64 = "0.21"
```

## Encrypting Text

Got a message you want to hide? Just feed it into Rusty_Encryptor with your 32-byte (Base64-encoded) key:

```bash
./target/release/rusty_encryptor -e "Hello, world!" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

## Decrypting Text

Need to see what’s inside your encrypted text? Use the same key to get it back:

```bash
./target/release/rusty_encryptor -d "ENCRYPTED_TEXT" -k "YOUR_BASE64_ENCODED_32_BYTE_KEY"
```

## Note

- AES-256-GCM uses a 12-byte nonce. Right now, we’re using a fixed nonce in this example, but in a real-world setup, you should generate a random nonce and store it alongside your ciphertext.
- Make sure you keep your keys safe! Proper key management is critical.
