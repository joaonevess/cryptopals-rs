#[cfg(test)]
mod tests {
    use cryptopals_rs::*;

    #[test]
    fn set_1_challenge_1() {
        let raw_bytes = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        assert_eq!(
            base64::encode(&raw_bytes),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn set_1_challenge_2() {
        let raw_bytes1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let raw_bytes2 = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let xor_bytes = bitxor::xor_slices(&raw_bytes1, &raw_bytes2).unwrap();

        assert_eq!(
            hex::encode(&xor_bytes),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn set_1_challenge_3() {
        let ciphertext =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();

        let (_, _, plaintext) = bitxor::crack_single_byte_xor(&ciphertext);
        assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
    }

    #[test]
    fn set_1_challenge_4() {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open("tests/challengedata/4.txt").unwrap();
        let reader = BufReader::new(file);
        let mut best_score = f32::MAX;
        let mut best_line = String::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let ciphertext = hex::decode(&line).unwrap();
            let (_, score, potential_plaintext) = bitxor::crack_single_byte_xor(&ciphertext);

            if score < best_score {
                best_score = score;
                best_line = potential_plaintext;
            }
        }
        assert_eq!(best_line, "Now that the party is jumping\n");
    }

    #[test]
    fn set_1_challenge_5() {
        let raw_bytes =
            b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

        let ciphertext = bitxor::xor_repeating_key(raw_bytes, &b"ICE".to_vec()).unwrap();

        assert_eq!(
            hex::encode(&ciphertext),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
