use crate::score_plaintext::english_score;

pub fn expand_key(key: &[u8], len: usize) -> Vec<u8> {
    key.iter().cycle().take(len).copied().collect()
}

pub fn xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, &'static str> {
    // Make sure the slices are the same length
    let length = a.len();
    if length != b.len() {
        return Err("slices have different lengths");
    }

    // Zip the slices together and xor the elements
    let result = a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect();
    Ok(result)
}

pub fn break_single_byte_xor(ciphertext: &[u8]) -> (u8, f32, String) {
    let mut best_key = 0;
    let mut best_score = f32::MAX;
    let mut best_plaintext = String::new();

    let mut expanded_key = vec![0; ciphertext.len()];
    for key in 0..=255 as u8 {
        expanded_key = expanded_key.iter().map(|_| key).collect();
        let decrypted = match xor(ciphertext, &expanded_key) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let score = english_score(&decrypted);

        if score < best_score {
            best_key = key;
            best_score = score;
            best_plaintext = String::from_utf8_lossy(&decrypted).to_string();
        }
    }

    (best_key, best_score, best_plaintext)
}

pub fn xor_repeating_key(input: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if key.is_empty() {
        return Err("key cannot be empty");
    }
    Ok(xor(input, &expand_key(key, input.len()))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        // Test XORing two slices of bytes
        let raw_bytes1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let raw_bytes2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let expected_xor = hex::decode("746865206b696420646f6e277420706c6179").unwrap();
        let xor_bytes = xor(&raw_bytes1, &raw_bytes2).unwrap();
        assert_eq!(xor_bytes, expected_xor);
    }

    #[test]
    fn test_xor_err() {
        // Test XORing two slices of bytes with different lenghts
        let raw_bytes1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let raw_bytes2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let xor_bytes = xor(&raw_bytes1, &raw_bytes2[0..10]);
        assert!(xor_bytes.is_err());
    }

    #[test]
    fn test_break_single_byte_xor() {
        // Test breaking a single-byte XOR cipher
        let ciphertext =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let expected_key = 'X' as u8;
        let (key, _, _) = break_single_byte_xor(&ciphertext);
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_repeating_key_xor() {
        // Test repeating-key XOR
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected_output =
            hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let output = xor_repeating_key(input, key).unwrap();
        assert_eq!(output, expected_output);
    }
}
