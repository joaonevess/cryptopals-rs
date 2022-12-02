#[cfg(test)]
mod tests {
    use cryptopals_rs::*;

    #[test]
    fn set_1_challenge_1() {
        let raw_bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let base64 = base64::encode(&raw_bytes);
        assert_eq!(
            base64,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn set_1_challenge_2() {
        let raw_bytes1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let raw_bytes2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let xor_bytes = xor::xor(&raw_bytes1, &raw_bytes2).unwrap();
        let xor_str = hex::encode(&xor_bytes);
        assert_eq!(xor_str, "746865206b696420646f6e277420706c6179");
    }

    // write a test that breaks single XOR cipher
    #[test]
    fn set_1_challenge_3() {
        let raw_bytes =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();

        let key = xor::break_single_byte_xor(&raw_bytes);
        let key_bytes = vec![key; raw_bytes.len()];
        let xor_bytes = xor::xor(&raw_bytes, &key_bytes).unwrap();
        let xor_str = String::from_utf8(xor_bytes).unwrap();
        assert_eq!(xor_str, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn set_1_challenge_4() {
        let file = std::fs::File::open("tests/challengedata/4.txt").unwrap();
        let reader = std::io::BufReader::new(file);
        let mut best_score = 0.0;
        let mut best_line = String::new();

        for line in std::io::BufRead::lines(reader) {
            let line = line.unwrap();
            let raw_bytes = hex::decode(&line).unwrap();

            let key = xor::break_single_byte_xor(&raw_bytes);
            let key_bytes = vec![key; raw_bytes.len()];

            let xor_bytes = xor::xor(&raw_bytes, &key_bytes).unwrap();
            let score = score_plaintext::english_score(&xor_bytes);

            if score > best_score {
                best_score = score;
                best_line = match std::str::from_utf8(&xor_bytes) {
                    Ok(v) => v.to_string(),
                    Err(e) => format!("Invalid UTF-8 sequence: {}", e),
                };
            }
        }
        assert_eq!(best_line, "Now that the party is jumping\n");
    }

    #[test]
    fn set_1_challenge_5() {
        let raw_bytes =
            b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expanded_key = xor::expand_key(key, raw_bytes.len());

        let xor_bytes = xor::xor(raw_bytes, &expanded_key).unwrap();

        let xor_str = hex::encode(&xor_bytes);
        assert_eq!(xor_str, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }
}
