const BLOCK_SIZE: usize = 16;

pub fn to_lowercase(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());

    let (blocks, remainder) = input.split_at(input.len() - (input.len() % BLOCK_SIZE));

    for chunk in blocks.chunks_exact(BLOCK_SIZE) {
        for &byte in chunk {
            output.push(to_lower_byte(byte));
        }
    }

    for &byte in remainder {
        output.push(to_lower_byte(byte));
    }

    output
}

pub fn to_uppercase(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());

    let (blocks, remainder) = input.split_at(input.len() - (input.len() % BLOCK_SIZE));

    for chunk in blocks.chunks_exact(BLOCK_SIZE) {
        for &byte in chunk {
            output.push(to_upper_byte(byte));
        }
    }

    for &byte in remainder {
        output.push(to_upper_byte(byte));
    }

    output
}

#[inline(always)]
const fn to_lower_byte(b: u8) -> u8 {
    if b >= b'A' && b <= b'Z' {
        b + 32
    } else {
        b
    }
}

#[inline(always)]
const fn to_upper_byte(b: u8) -> u8 {
    if b >= b'a' && b <= b'z' {
        b - 32
    } else {
        b
    }
}

pub fn bitwise_or(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    let min_len = op1.len().min(op2.len());
    let mut result = Vec::with_capacity(op1.len().max(op2.len()));

    for i in 0..min_len {
        result.push(op1[i] | op2[i]);
    }

    if op1.len() > min_len {
        result.extend_from_slice(&op1[min_len..]);
    } else if op2.len() > min_len {
        result.extend_from_slice(&op2[min_len..]);
    }

    result
}

pub fn bitwise_and(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    let min_len = op1.len().min(op2.len());
    let mut result = Vec::with_capacity(min_len);

    for i in 0..min_len {
        result.push(op1[i] & op2[i]);
    }

    result
}

pub fn bitwise_xor(op1: &[u8], op2: &[u8]) -> Vec<u8> {
    let min_len = op1.len().min(op2.len());
    let mut result = Vec::with_capacity(op1.len().max(op2.len()));

    for i in 0..min_len {
        result.push(op1[i] ^ op2[i]);
    }

    if op1.len() > min_len {
        result.extend_from_slice(&op1[min_len..]);
    } else if op2.len() > min_len {
        result.extend_from_slice(&op2[min_len..]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        assert_eq!(to_lowercase(b"HELLO"), b"hello");
        assert_eq!(to_lowercase(b"Hello World"), b"hello world");
        assert_eq!(to_lowercase(b"123ABC"), b"123abc");
        assert_eq!(to_lowercase(b""), b"");
    }

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase(b"hello"), b"HELLO");
        assert_eq!(to_uppercase(b"Hello World"), b"HELLO WORLD");
        assert_eq!(to_uppercase(b"123abc"), b"123ABC");
        assert_eq!(to_uppercase(b""), b"");
    }

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
            b"\x1E\x1D\x0C\x1A\x18onger"
        );
    }
}
