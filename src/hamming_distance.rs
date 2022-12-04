pub fn hamming_distance(a: &[u8], b: &[u8]) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("inputs must be of equal length");
    }
    Ok(a.iter()
        .zip(b)
        .map(|(x, y)| (x ^ y).count_ones() as usize)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance(b"this is a test", b"wokka wokka!!!"),
            Ok(37)
        );
        assert_eq!(
            hamming_distance(b"this is a test", b"this is a test"),
            Ok(0)
        );
    }
    #[test]
    fn test_hamming_distance_diff_len() {
        assert_eq!(
            hamming_distance(b"this is a test", b"this is a test!"),
            Err("inputs must be of equal length")
        );
    }
}
