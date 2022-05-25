use std::{fs, path::Path};

pub fn shift_cipher(cipher: &str, k: Option<u8>) -> Vec<(String, u8)> {
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

fn main() {
    let cipher = "OVDTHUFWVZZPISLRLFZHYLAOLYL";
    println!(
        "The most possible plain text: {:?} with k: {:?}\n",
        shift_cipher(cipher, None)[0].0,
        shift_cipher(cipher, None)[0].1
    );
    ()
}
