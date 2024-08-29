use clap::{Arg, Command};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, aead, BoundKey};
use ring::rand::{SystemRandom, SecureRandom};
use base64::{encode, decode};

fn main() {
    let matches = Command::new("Rusty Encryptor")
        .version("1.0")
        .author("Your Name")
        .about("Encrypts and decrypts text or files using AES-256-GCM")
        .arg(
            Arg::new("encrypt")
                .short('e')
                .long("encrypt")
                .takes_value(true)
                .about("Encrypts the provided text"),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .takes_value(true)
                .about("Decrypts the provided text"),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .takes_value(true)
                .about("Encryption/Decryption key (32 bytes in base64)"),
        )
        .get_matches();

    let key = matches.value_of("key").expect("Key is required").as_bytes();
    if key.len() != 32 {
        eprintln!("Key must be 32 bytes long (AES-256 requires a 32-byte key).");
        return;
    }

    if let Some(text) = matches.value_of("encrypt") {
        let encrypted = encrypt_text(text.as_bytes(), key);
        match encrypted {
            Ok(result) => println!("Encrypted: {}", result),
            Err(e) => eprintln!("Error encrypting: {:?}", e),
        }
    }

    if let Some(text) = matches.value_of("decrypt") {
        let decrypted = decrypt_text(text.as_bytes(), key);
        match decrypted {
            Ok(result) => println!("Decrypted: {}", String::from_utf8_lossy(&result)),
            Err(e) => eprintln!("Error decrypting: {:?}", e),
        }
    }
}

fn encrypt_text(plaintext: &[u8], key: &[u8]) -> Result<String, aead::Error> {
    let mut sealing_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap());
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let mut in_out = plaintext.to_vec();
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)?;
    Ok(encode(&in_out))
}

fn decrypt_text(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, aead::Error> {
    let mut opening_key = LessSafeKey::new(UnboundKey::new(&AES_256_GCM, key).unwrap());
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let mut in_out = decode(ciphertext).unwrap();
    opening_key.open_in_place(Aad::empty(), &mut in_out)?;
    Ok(in_out)
}
