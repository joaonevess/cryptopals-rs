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

pub fn break_single_byte_xor(ciphertext: &[u8]) -> u8 {
    // Create a frequency array of the ciphertext
    let mut freq = [0; 256];
    for &b in ciphertext {
        freq[b as usize] += 1;
    }

    // Find the byte with the highest frequency in the ciphertext
    let mut max_b = 0;
    let mut max_count = 0;
    for (b, &count) in freq.iter().enumerate() {
        if count > max_count {
            max_b = b as u8;
            max_count = count;
        }
    }

    // The most common byte in English text is the space character
    // We can use this to find the key byte
    max_b ^ b' '
}

fn repeating_key_xor(input: &[u8], key: &[u8]) -> Result<Vec<u8>, &'static str> {
    if key.is_empty() {
        return Err("key cannot be empty");
    }

    Ok(input
        .iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| x ^ y)
        .collect())
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
    fn test_xor_2() {
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
        let key = break_single_byte_xor(&ciphertext);
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_repeating_key_xor() {
        // Test repeating-key XOR
        let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected_output =
            hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        let output = repeating_key_xor(input, key).unwrap();
        assert_eq!(output, expected_output);
    }
}
