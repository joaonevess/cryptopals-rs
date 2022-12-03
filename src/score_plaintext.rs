use std::collections::HashMap;

// https://www.cl.cam.ac.uk/~mgk25/lee-essays.pdf
fn get_english_frequencies() -> HashMap<u8, f32> {
    let mut expected_frequencies: HashMap<u8, f32> = HashMap::new();

    expected_frequencies.insert(b'.', 0.0657); // Other
    expected_frequencies.insert(b' ', 0.1217);
    expected_frequencies.insert(b'a', 0.0609);
    expected_frequencies.insert(b'b', 0.0105);
    expected_frequencies.insert(b'c', 0.0284);
    expected_frequencies.insert(b'd', 0.0292);
    expected_frequencies.insert(b'e', 0.1136);
    expected_frequencies.insert(b'f', 0.0179);
    expected_frequencies.insert(b'g', 0.0138);
    expected_frequencies.insert(b'h', 0.0341);
    expected_frequencies.insert(b'i', 0.0544);
    expected_frequencies.insert(b'j', 0.0024);
    expected_frequencies.insert(b'k', 0.0041);
    expected_frequencies.insert(b'l', 0.0292);
    expected_frequencies.insert(b'm', 0.0276);
    expected_frequencies.insert(b'n', 0.0544);
    expected_frequencies.insert(b'o', 0.0600);
    expected_frequencies.insert(b'p', 0.0195);
    expected_frequencies.insert(b'q', 0.0024);
    expected_frequencies.insert(b'r', 0.0495);
    expected_frequencies.insert(b's', 0.0568);
    expected_frequencies.insert(b't', 0.0803);
    expected_frequencies.insert(b'u', 0.0243);
    expected_frequencies.insert(b'v', 0.0097);
    expected_frequencies.insert(b'w', 0.0138);
    expected_frequencies.insert(b'x', 0.0024);
    expected_frequencies.insert(b'y', 0.0130);
    expected_frequencies.insert(b'z', 0.0003);

    expected_frequencies
}

fn is_control(c: u8) -> bool {
    c < 0x20 || c == 0x7f
}

fn is_alphabetic(c: u8) -> bool {
    (c >= 0x41 && c <= 0x5A) || (c >= 0x61 && c <= 0x7A)
}

pub fn get_char_count(input: &[u8]) -> HashMap<u8, f32> {
    let mut counts: HashMap<u8, f32> = HashMap::new();

    for &c in input {
        if is_control(c) {
            continue;
        }

        let key = if is_alphabetic(c) {
            c.to_ascii_lowercase()
        } else if c == b' ' || c == b'\t' {
            b' '
        } else {
            b'.'
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

    let expected_frequencies = get_english_frequencies();
    let counts = get_char_count(bytes);

    // Calculate the score of the input by summing the squared difference between the
    // expected and actual frequency of each character.
    let mut score = 0.0;
    for (c, count) in counts {
        let expected = expected_frequencies.get(&c).unwrap_or(&0.0);
        let actual = count / bytes.len() as f32;
        score += (expected - actual).powi(2);
    }

    score
}
