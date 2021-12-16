extern crate aesstream;
extern crate base64;
extern crate crypto;

use aesstream::{AesReader, AesWriter};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use std::{
    fs::File,
    io::{Cursor, Read, Write},
};

const EARLY_TIMES: &str =
    "VEZoS1drdFdXVEZTTTFFd1RsWktiMU42U25sTGJHUm9aRlZLYlZFeFJYbGliVlV4Wlc1VmNsWkZXVDA9";

fn decode_key() -> Vec<u8> {
    let mut s = EARLY_TIMES.as_bytes().to_vec();
    loop {
        s = base64::decode(s.clone()).unwrap();
        if s[0] == 0x2d {
            return s;
        }
    }
}

pub fn crypt_str(target: &str) -> Option<Vec<u8>> {
    let key = decode_key();
    let encryptor = AesSafe256Encryptor::new(&key);
    let mut encrypted = Vec::new();
    {
        let mut writer = AesWriter::new(&mut encrypted, encryptor).unwrap();
        writer
            .write_all(base64::encode(target.as_bytes()).as_bytes())
            .unwrap();
    }
    return Some(encrypted);
}

pub fn decrypt_str(target: &Vec<u8>) -> Option<String> {
    let key = decode_key();
    let decryptor = AesSafe256Decryptor::new(&key);
    let mut reader = AesReader::new(Cursor::new(target), decryptor).unwrap();
    let mut decrypted = String::new();
    reader.read_to_string(&mut decrypted).unwrap();
    Some(String::from_utf8(base64::decode(decrypted).unwrap()).unwrap())
}

pub fn encrypt_game_binary() {
    let mut file = File::open("target/release/suzu.exe").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();
    let encoded = base64::encode(buf.as_slice());

    let encrypted = crypt_str(&encoded).unwrap();

    let mut file = std::fs::File::create("suzu.exe.encrypted").unwrap();
    file.write_all(encrypted.as_slice()).unwrap();
}

pub fn decrypt_game_binary(target: &Vec<u8>) {
    let decrypted = decrypt_str(target).unwrap();
    let binary = base64::decode(decrypted).unwrap();
    std::fs::rename("suzu.exe", "suzu_old.exe").unwrap();
    let mut file = std::fs::File::create("suzu.exe").unwrap();
    file.write_all(binary.as_slice()).unwrap();
}
