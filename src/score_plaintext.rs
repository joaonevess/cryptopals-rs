pub fn english_score(bytes: &[u8]) -> f64 {
    // Create a fixed-size array to count the frequency of each byte
    // can be faster than using a HashMap for small inputs
    let mut byte_counts = [0; 256];
    for byte in bytes {
        byte_counts[*byte as usize] += 1;
    }

    // Use the English letter frequencies found on Wikipedia
    // https://en.wikipedia.org/wiki/Letter_frequency#Relative_frequencies_of_letters_in_the_English_language
    let mut score = 0.0;
    for (i, count) in byte_counts.iter().enumerate() {
        // We'll only consider ASCII letters, which have values between 65 and 122 (inclusive)
        if i >= 65 && i <= 122 {
            let letter = i as u8 as char;
            let frequency = match letter {
                'a' | 'A' => 8.167,
                'b' | 'B' => 1.492,
                'c' | 'C' => 2.782,
                'd' | 'D' => 4.253,
                'e' | 'E' => 12.702,
                'f' | 'F' => 2.228,
                'g' | 'G' => 2.015,
                'h' | 'H' => 6.094,
                'i' | 'I' => 6.966,
                'j' | 'J' => 0.153,
                'k' | 'K' => 0.772,
                'l' | 'L' => 4.025,
                'm' | 'M' => 2.406,
                'n' | 'N' => 6.749,
                'o' | 'O' => 7.507,
                'p' | 'P' => 1.929,
                'q' | 'Q' => 0.095,
                'r' | 'R' => 5.987,
                's' | 'S' => 6.327,
                't' | 'T' => 9.056,
                'u' | 'U' => 2.758,
                'v' | 'V' => 0.978,
                'w' | 'W' => 2.360,
                'x' | 'X' => 0.150,
                'y' | 'Y' => 1.974,
                'z' | 'Z' => 0.074,
                _ => 0.0,
            };

            score += frequency * (*count as f64);
        }
    }

    // Return the score normalized by the number of bytes in the input slice
    score / bytes.len() as f64
}
