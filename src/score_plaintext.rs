use lazy_static::lazy_static;
use std::collections::HashMap;

// https://www.cl.cam.ac.uk/~mgk25/lee-essays.pdf
lazy_static! {
    static ref ENGLISH_FREQUENCIES: HashMap<u8, f32> = HashMap::from_iter(vec![
        (b'.', 0.0657), // Other
        (b' ', 0.1217),
        (b'a', 0.0609),
        (b'b', 0.0105),
        (b'c', 0.0284),
        (b'd', 0.0292),
        (b'e', 0.1136),
        (b'f', 0.0179),
        (b'g', 0.0138),
        (b'h', 0.0341),
        (b'i', 0.0544),
        (b'j', 0.0024),
        (b'k', 0.0041),
        (b'l', 0.0292),
        (b'm', 0.0276),
        (b'n', 0.0544),
        (b'o', 0.0600),
        (b'p', 0.0195),
        (b'q', 0.0024),
        (b'r', 0.0495),
        (b's', 0.0568),
        (b't', 0.0803),
        (b'u', 0.0243),
        (b'v', 0.0097),
        (b'w', 0.0138),
        (b'x', 0.0024),
        (b'y', 0.0130),
        (b'z', 0.0003),
    ]);

}

fn is_control(c: u8) -> bool {
    c < 0x20 || c == 0x7f
}

fn is_alphabetic(c: u8) -> bool {
    (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
}

pub fn get_byte_frequencies(bytes: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();

    for &c in bytes {
        if is_control(c) {
            continue;
        }

        let key = match c {
            c if is_alphabetic(c) => c.to_ascii_lowercase(),
            b' ' | b'\t' => b' ',
            _ => b'.',
        };

        *counts.entry(key).or_insert(0.0) += 1.0;
    }

    counts
}

pub fn english_score(bytes: &[u8]) -> f32 {
    if !bytes.is_ascii() {
        return f32::MAX;
    }
    if bytes.iter().any(|&c| is_control(c) && c != b'\n') {
        return f32::MAX;
    }

    get_byte_frequencies(bytes)
        .iter()
        .map(|(c, count)| {
            let expected = ENGLISH_FREQUENCIES.get(c).unwrap_or(&0.0);
            let actual = count / bytes.len() as f32;
            (expected - actual).powi(2)
        })
        .sum()
}
