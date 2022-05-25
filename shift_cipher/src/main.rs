#![allow(non_upper_case_globals)]

use argparse::{ArgumentParser, Store, StoreTrue};
use std::{fs, path::Path, process::exit};

static mut plain_text: String = String::new();
static mut cipher_text: String = String::new();
static mut key: u128 = u128::MAX;
static mut encrypt: bool = false;
static mut decrypt: bool = false;
static mut output_upper_case: bool = false;
static mut output_count: u8 = 1;

/// decrypt shift cipher -> return (Vec<(plaintext, key)>)
pub fn shift_cipher_decrypt(cipher: &str, k: Option<u8>) -> Vec<(String, u8)> {
    let numbers = cipher
        .as_bytes()
        .iter()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| c - b'a')
        .collect::<Vec<u8>>();

    if let Some(k) = k {
        let decipher = numbers
            .clone()
            .into_iter()
            .map(|c| (c + k) % 26)
            .map(|c| c + b'a')
            .collect::<Vec<u8>>();
        let decipher_str = std::str::from_utf8(&decipher).unwrap().to_owned();
        return vec![(decipher_str, k)];
    }

    let mut v = Vec::<(String, u32, u8)>::new();
    for k in 1..=25 {
        let decipher = numbers
            .clone()
            .into_iter()
            .map(|c| (c + k) % 26)
            .map(|c| c + b'a')
            .collect::<Vec<u8>>();
        v.push((std::str::from_utf8(&decipher).unwrap().to_owned(), 0, k));
    }

    // calculate prop
    let items = fs::read_to_string(Path::new("shift_cipher\\src\\fc_words_prefix_suffix.txt"))
        .expect("Something went wrong reading the file");

    let items = items.split("\r\n").collect::<Vec<&str>>();
    for plain_text_t in &mut v {
        let score = &mut plain_text_t.1;
        for item in items.clone() {
            if plain_text_t.0.contains(item) {
                // println!("caught: {:?}, {:?}", plain_text_t.0, item);
                *score += item.len() as u32;
            }
        }
    }

    v.sort_by_key(|k| k.1);
    v.reverse();
    let v = v
        .into_iter()
        .map(|k| (k.0, k.2))
        .collect::<Vec<(String, u8)>>();

    v
}

/// encrypt shift cipher -> return (Vec<(cipher, key)>)
pub fn shift_cipher_encrypt(plain: &str, k: Option<u8>) -> Vec<(String, u8)> {
    let numbers = plain
        .as_bytes()
        .iter()
        .map(|p| p.to_ascii_lowercase())
        .map(|p| p - b'a')
        .collect::<Vec<u8>>();

    if let Some(k) = k {
        let encipher = numbers
            .clone()
            .into_iter()
            .map(|p| (p + k) % 26)
            .map(|p| p + b'a')
            .collect::<Vec<u8>>();
        let encipher_str = std::str::from_utf8(&encipher).unwrap().to_owned();
        return vec![(encipher_str, k)];
    }

    let mut v = Vec::<(String, u8)>::new();
    for k in 1..=25 {
        let encipher = numbers
            .clone()
            .into_iter()
            .map(|p| (p + k) % 26)
            .map(|p| p + b'a')
            .collect::<Vec<u8>>();
        v.push((std::str::from_utf8(&encipher).unwrap().to_owned(), k));
    }

    v
}

fn main() {
    // handle args
    unsafe {
        let mut ap = ArgumentParser::new();
        ap.set_description("SHIFT CIPHER TOOL");
        ap.refer(&mut encrypt)
            .add_option(&["-e", "--encrypt"], StoreTrue, "Encryption mode");
        ap.refer(&mut decrypt)
            .add_option(&["-d", "--decrypt"], StoreTrue, "Decryption mode");
        ap.refer(&mut plain_text)
            .add_option(&["-p", "--plain"], Store, "Plain text.");
        ap.refer(&mut cipher_text)
            .add_option(&["-c", "--cipher"], Store, "Cipher text.");
        ap.refer(&mut key)
            .add_option(&["-k", "--key"], Store, "Key");

        ap.refer(&mut output_upper_case).add_option(
            &["-o", "--output_upper_case"],
            StoreTrue,
            "Output should be the upper case.",
        );

        ap.refer(&mut output_count).add_option(
            &["-n", "--output_count"],
            Store,
            "Output count of possible decrypted message.",
        );

        ap.parse_args_or_exit();
    }

    // sanity check on arguments and wrap unsafe variable to safe variable
    unsafe {
        if (!encrypt && !decrypt) || (encrypt && decrypt) {
            println!("Do you want to do encryption or decryption?\n");
            exit(0);
        }

        if encrypt && plain_text == "" {
            println!("Please enter the key and the plain text for encryption!\n");
            exit(0);
        }

        if decrypt && cipher_text == "" {
            println!("Please enter the cipher text for decryption!\n");
            exit(0);
        }

        if key != u128::MAX && output_count > 1 {
            println!("If specify the key, then the output count should only be 1!\n");
            exit(0);
        }
    };

    let k = unsafe {
        if key != u128::MAX {
            Some((key % 26) as u8)
        } else {
            None
        }
    };

    if unsafe { encrypt } {
        let v = shift_cipher_encrypt(unsafe { &plain_text }, k);

        for i in 0..unsafe {
            if output_count > 25 {
                25
            } else {
                output_count
            }
        } as usize
        {
            println!(
                "The encrypted cipher is: {:?} with k: {:?}\n",
                if unsafe { output_upper_case } {
                    v[i].0.to_ascii_uppercase()
                } else {
                    v[i].0.clone()
                },
                v[i].1
            );
        }
    } else {
        let v = shift_cipher_decrypt(unsafe { &cipher_text }, k);

        for i in 0..unsafe {
            if output_count > 25 {
                25
            } else {
                output_count
            }
        } as usize
        {
            println!(
                "The top {:?} possible plain text is: {:?} with k: {:?}\n",
                i + 1,
                if unsafe { output_upper_case } {
                    v[i].0.to_ascii_uppercase()
                } else {
                    v[i].0.clone()
                },
                v[i].1
            );
        }
    }
}
