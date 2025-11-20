const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PAD: u8 = b'=';

// Maps ASCII => Base64 value (0-63), PAD (254), or INVALID (255)
const DEC_TABLE: [u8; 256] = {
    let mut t = [255u8; 256];
    let mut i = 0;
    while i < 64 {
        t[TABLE[i] as usize] = i as u8;
        i += 1;
    }
    t[b'=' as usize] = 254; // Marker for padding
    t
};

/// Encode bytes to base64
///
/// # Arguments
///
/// * `input` - The bytes to encode
/// * `no_padding` - Whether to omit padding characters
///
/// # Returns
///
/// A vector containing the base64-encoded bytes
pub fn encode(input: &[u8], no_padding: bool) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }

    let output_len = if no_padding {
        (input.len() * 4).div_ceil(3)
    } else {
        input.len().div_ceil(3) * 4
    };

    let mut output = Vec::with_capacity(output_len);
    let mut chunks = input.chunks_exact(3);

    for chunk in &mut chunks {
        let group = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);

        output.extend_from_slice(&[
            TABLE[((group >> 18) & 0x3F) as usize],
            TABLE[((group >> 12) & 0x3F) as usize],
            TABLE[((group >> 6) & 0x3F) as usize],
            TABLE[(group & 0x3F) as usize],
        ]);
    }

    let rem = chunks.remainder();
    if !rem.is_empty() {
        let b0 = rem[0] as u32;
        output.push(TABLE[((b0 >> 2) & 0x3F) as usize]);

        if rem.len() == 1 {
            output.push(TABLE[((b0 << 4) & 0x3F) as usize]);
            if !no_padding {
                output.push(PAD);
                output.push(PAD);
            }
        } else {
            let b1 = rem[1] as u32;
            output.push(TABLE[(((b0 << 4) | (b1 >> 4)) & 0x3F) as usize]);
            output.push(TABLE[((b1 << 2) & 0x3F) as usize]);
            if !no_padding {
                output.push(PAD);
            }
        }
    }

    output
}

/// Decode base64 to bytes
///
/// # Arguments
///
/// * `input` - The base64-encoded bytes
/// * `strict` - Whether to enforce strict validation
///
/// # Returns
///
/// `Some(Vec<u8>)` if decoding succeeds, `None` if input is invalid
pub fn decode(input: &[u8], strict: bool) -> Option<Vec<u8>> {
    if input.is_empty() {
        return Some(Vec::new());
    }

    let mut output = Vec::with_capacity((input.len() * 3) / 4);

    if strict {
        let mut chunks = input.chunks_exact(4);

        for chunk in &mut chunks {
            let b0 = DEC_TABLE[chunk[0] as usize];
            let b1 = DEC_TABLE[chunk[1] as usize];
            let b2 = DEC_TABLE[chunk[2] as usize];
            let b3 = DEC_TABLE[chunk[3] as usize];

            if (b0 | b1 | b2 | b3) >= 64 {
                if b0 == 255 || b1 == 255 || b2 == 255 || b3 == 255 {
                    return None;
                }

                let mut acc = (b0 as u32) << 18 | (b1 as u32) << 12;

                if b0 >= 64 || b1 >= 64 {
                    return None;
                }

                if b2 == 254 {
                    if b3 != 254 {
                        return None;
                    }

                    output.push((acc >> 16) as u8);
                    if chunks.remainder().is_empty() {
                        return Some(output);
                    } else {
                        return None;
                    }
                } else if b3 == 254 {
                    acc |= (b2 as u32) << 6;
                    output.push((acc >> 16) as u8);
                    output.push((acc >> 8) as u8);
                    if chunks.remainder().is_empty() {
                        return Some(output);
                    } else {
                        return None;
                    }
                }

                return None;
            }

            let group = (b0 as u32) << 18 | (b1 as u32) << 12 | (b2 as u32) << 6 | (b3 as u32);
            output.extend_from_slice(&[(group >> 16) as u8, (group >> 8) as u8, group as u8]);
        }

        if !chunks.remainder().is_empty() {
            return None;
        }
    } else {
        let mut acc: u32 = 0;
        let mut bits = 0;

        for &ch in input {
            let val = DEC_TABLE[ch as usize];
            if val == 255 {
                continue;
            }
            if val == 254 {
                break;
            }

            acc = (acc << 6) | (val as u32);
            bits += 6;

            if bits >= 8 {
                bits -= 8;
                output.push((acc >> bits) as u8);
            }
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        assert_eq!(encode(b"Hello", false), b"SGVsbG8=");
    }

    #[test]
    fn test_encode_no_padding() {
        assert_eq!(encode(b"Hello", true), b"SGVsbG8");
    }

    #[test]
    fn test_decode_basic() {
        assert_eq!(decode(b"SGVsbG8=", false), Some(b"Hello".to_vec()));
    }

    #[test]
    fn test_decode_strict_invalid() {
        assert_eq!(decode(b"SGVs!!!bG8=", true), None);
    }

    #[test]
    fn test_decode_strict_wrong_padding() {
        assert_eq!(decode(b"VV=", true), None);
    }

    #[test]
    fn test_decode_strict_correct_padding() {
        assert_eq!(decode(b"VV==", true), Some(b"U".to_vec()));
    }

    #[test]
    fn test_round_trip() {
        let input = b"The quick brown fox jumps over the lazy dog";
        let encoded = encode(input, false);
        let decoded = decode(&encoded, true).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_large() {
        let input = vec![0u8; 10000];
        let encoded = encode(&input, false);
        let decoded = decode(&encoded, false).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_edge_cases() {
        assert_eq!(encode(b"", false), b"");
        assert_eq!(encode(b"f", false), b"Zg==");
        assert_eq!(encode(b"fo", false), b"Zm8=");
        assert_eq!(encode(b"foo", false), b"Zm9v");
        assert_eq!(encode(b"foob", false), b"Zm9vYg==");
        assert_eq!(encode(b"fooba", false), b"Zm9vYmE=");
        assert_eq!(encode(b"foobar", false), b"Zm9vYmFy");
    }
}
