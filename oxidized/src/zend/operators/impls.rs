/// Perform bitwise OR on two byte slices
///
/// Returns a new Vec containing the result. If slices have different lengths,
/// the remaining bytes from the longer slice are appended unchanged.
pub fn bitwise_or(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    if op1.is_empty() {
        return op2.to_vec();
    }

    if op2.is_empty() {
        return op1.to_vec();
    }

    let min_len = op1.len().min(op2.len());
    let max_len = op1.len().max(op2.len());

    if max_len == 0 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(max_len);

    result.extend(
        op1[..min_len]
            .iter()
            .zip(&op2[..min_len])
            .map(|(a, b)| a | b),
    );

    if op1.len() > min_len {
        result.extend_from_slice(&op1[min_len..]);
    } else if op2.len() > min_len {
        result.extend_from_slice(&op2[min_len..]);
    }

    result
}

/// Perform bitwise AND on two byte slices
///
/// Returns a new Vec containing the result. The result length is the minimum
/// of the two input lengths.
pub fn bitwise_and(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    if op1.is_empty() || op2.is_empty() {
        return Vec::new();
    }

    let min_len = op1.len().min(op2.len());

    if min_len == 0 {
        return Vec::new();
    }

    op1[..min_len]
        .iter()
        .zip(&op2[..min_len])
        .map(|(a, b)| a & b)
        .collect()
}

/// Perform bitwise XOR on two byte slices
///
/// Returns a new Vec containing the result. The result length is the minimum
/// of the two input lengths (matching PHP's behavior).
pub fn bitwise_xor(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    let min_len = op1.len().min(op2.len());

    if min_len == 0 {
        return Vec::new();
    }

    op1[..min_len]
        .iter()
        .zip(&op2[..min_len])
        .map(|(a, b)| a ^ b)
        .collect()
}

/// Convert bytes to lowercase (ASCII only)
///
/// # Arguments
///
/// * `input` - The bytes to convert
///
/// # Returns
///
/// A new Vec with lowercase ASCII characters
pub fn to_lowercase(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .map(|&b| if b.is_ascii_uppercase() { b + 32 } else { b })
        .collect()
}

/// Convert bytes to uppercase (ASCII only)
///
/// # Arguments
///
/// * `input` - The bytes to convert
///
/// # Returns
///
/// A new Vec with uppercase ASCII characters
pub fn to_uppercase(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .map(|&b| if b.is_ascii_lowercase() { b - 32 } else { b })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_or() {
        assert_eq!(bitwise_or(b"\x0F", b"\xF0"), b"\xFF");
        assert_eq!(bitwise_or(b"abc", b"def"), b"egg");
        assert_eq!(
            bitwise_or(b"short", b"muchlonger"),
            b"\x7F\x7D\x6F\x7A\x7Conger"
        );
    }

    #[test]
    fn test_bitwise_and() {
        assert_eq!(bitwise_and(b"\xFF", b"\x0F"), b"\x0F");
        assert_eq!(bitwise_and(b"abc", b"def"), b"``b");
    }

    #[test]
    fn test_bitwise_xor() {
        assert_eq!(bitwise_xor(b"\xFF", b"\x0F"), b"\xF0");
        assert_eq!(bitwise_xor(b"abc", b"def"), b"\x05\x07\x05");
        assert_eq!(
            bitwise_xor(b"short", b"muchlonger"),
            b"\x1E\x1D\x0C\x1A\x18"
        );
    }

    #[test]
    fn test_bitwise_large() {
        let op1 = vec![0xAAu8; 10000];
        let op2 = vec![0x55u8; 10000];

        let or_result = bitwise_or(&op1, &op2);
        assert_eq!(or_result.len(), 10000);
        assert!(or_result.iter().all(|&b| b == 0xFF));

        let and_result = bitwise_and(&op1, &op2);
        assert_eq!(and_result.len(), 10000);
        assert!(and_result.iter().all(|&b| b == 0x00));

        let xor_result = bitwise_xor(&op1, &op2);
        assert_eq!(xor_result.len(), 10000);
        assert!(xor_result.iter().all(|&b| b == 0xFF));
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(bitwise_or(b"", b""), Vec::<u8>::new());
        assert_eq!(bitwise_and(b"", b""), Vec::<u8>::new());
        assert_eq!(bitwise_xor(b"", b""), Vec::<u8>::new());

        assert_eq!(bitwise_or(b"", b"test"), b"test");
        assert_eq!(bitwise_and(b"", b"test"), b"");
        assert_eq!(bitwise_xor(b"", b"test"), b"");

        assert_eq!(bitwise_or(b"test", b""), b"test");
        assert_eq!(bitwise_and(b"test", b""), b"");
        assert_eq!(bitwise_xor(b"test", b""), b"");
    }
}
