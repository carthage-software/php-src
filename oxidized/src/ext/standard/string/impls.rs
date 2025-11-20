pub fn str_repeat(input: &[u8], mult: usize) -> Vec<u8> {
    if input.is_empty() || mult == 0 {
        return Vec::new();
    }

    let result_len = input.len() * mult;

    if input.len() == 1 {
        return vec![input[0]; result_len];
    }

    let mut result = Vec::with_capacity(result_len);
    result.extend_from_slice(input);

    let mut copied = input.len();
    while copied < result_len {
        let to_copy = copied.min(result_len - copied);

        // Safety: We ensure that we do not write beyond the allocated capacity of `result`.
        unsafe {
            let src = result.as_ptr();
            let dst = result.as_mut_ptr().add(copied);
            std::ptr::copy_nonoverlapping(src, dst, to_copy);
            result.set_len(copied + to_copy);
        }

        copied += to_copy;
    }

    result
}

pub fn trim(input: &[u8], what: Option<&[u8]>, mode: i32) -> (usize, usize) {
    if input.is_empty() {
        return (0, 0);
    }

    let mut start = 0;
    let mut end = input.len();

    match what {
        Some(chars) if chars.len() == 1 => {
            let ch = chars[0];
            if mode & 1 != 0 {
                while start < end && input[start] == ch {
                    start += 1;
                }
            }
            if mode & 2 != 0 {
                while start < end && input[end - 1] == ch {
                    end -= 1;
                }
            }
        }
        Some(chars) => {
            let mask = build_char_mask(chars);
            if mode & 1 != 0 {
                while start < end && mask[input[start] as usize] {
                    start += 1;
                }
            }
            if mode & 2 != 0 {
                while start < end && mask[input[end - 1] as usize] {
                    end -= 1;
                }
            }
        }
        None => {
            if mode & 1 != 0 {
                while start < end {
                    let c = input[start];
                    if c <= b' '
                        && (c == b' '
                            || c == b'\n'
                            || c == b'\r'
                            || c == b'\t'
                            || c == b'\x0b'
                            || c == 0)
                    {
                        start += 1;
                    } else {
                        break;
                    }
                }
            }
            if mode & 2 != 0 {
                while start < end {
                    let c = input[end - 1];
                    if c <= b' '
                        && (c == b' '
                            || c == b'\n'
                            || c == b'\r'
                            || c == b'\t'
                            || c == b'\x0b'
                            || c == 0)
                    {
                        end -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    (start, end - start)
}

fn build_char_mask(chars: &[u8]) -> [bool; 256] {
    let mut mask = [false; 256];
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if i + 3 < chars.len() && chars[i + 1] == b'.' && chars[i + 2] == b'.' {
            let range_end = chars[i + 3];
            if range_end >= c {
                for ch in c..=range_end {
                    mask[ch as usize] = true;
                }
                i += 4;
                continue;
            }
        }

        mask[c as usize] = true;
        i += 1;
    }

    mask
}

const PAD_LEFT: i32 = 0;
const PAD_RIGHT: i32 = 1;
const PAD_BOTH: i32 = 2;

pub fn str_pad(input: &[u8], pad_length: usize, pad_str: &[u8], pad_type: i32) -> Vec<u8> {
    if pad_length <= input.len() || pad_str.is_empty() {
        return input.to_vec();
    }

    let num_pad_chars = pad_length - input.len();
    let mut result = Vec::with_capacity(pad_length);

    let (left_pad, right_pad) = match pad_type {
        PAD_LEFT => (num_pad_chars, 0),
        PAD_RIGHT => (0, num_pad_chars),
        PAD_BOTH => {
            let left = num_pad_chars / 2;
            (left, num_pad_chars - left)
        }
        _ => (0, num_pad_chars),
    };

    if left_pad > 0 {
        fill_padding(&mut result, left_pad, pad_str);
    }

    result.extend_from_slice(input);

    if right_pad > 0 {
        fill_padding(&mut result, right_pad, pad_str);
    }

    result
}

fn fill_padding(result: &mut Vec<u8>, pad_chars: usize, pad_str: &[u8]) {
    if pad_str.len() == 1 {
        result.resize(result.len() + pad_chars, pad_str[0]);
        return;
    }

    let full_repeats = pad_chars / pad_str.len();
    let remaining = pad_chars % pad_str.len();

    for _ in 0..full_repeats {
        result.extend_from_slice(pad_str);
    }

    if remaining > 0 {
        result.extend_from_slice(&pad_str[..remaining]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_repeat_single_byte() {
        let result = str_repeat(b"a", 5);
        assert_eq!(result, b"aaaaa");
    }

    #[test]
    fn test_str_repeat_multi_byte() {
        let result = str_repeat(b"ab", 3);
        assert_eq!(result, b"ababab");
    }

    #[test]
    fn test_str_repeat_empty() {
        let result = str_repeat(b"", 10);
        assert_eq!(result, b"");
    }

    #[test]
    fn test_str_repeat_zero_mult() {
        let result = str_repeat(b"test", 0);
        assert_eq!(result, b"");
    }

    #[test]
    fn test_str_repeat_one_mult() {
        let result = str_repeat(b"hello", 1);
        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_str_repeat_large() {
        let result = str_repeat(b"x", 1000);
        assert_eq!(result.len(), 1000);
        assert!(result.iter().all(|&b| b == b'x'));
    }

    #[test]
    fn test_str_repeat_doubling() {
        let result = str_repeat(b"abc", 10);
        assert_eq!(result.len(), 30);
        for chunk in result.chunks_exact(3) {
            assert_eq!(chunk, b"abc");
        }
    }

    #[test]
    fn test_trim_default_both() {
        let (start, len) = trim(b"  hello  ", None, 3);
        assert_eq!(start, 2);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_default_left() {
        let (start, len) = trim(b"  hello", None, 1);
        assert_eq!(start, 2);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_default_right() {
        let (start, len) = trim(b"hello  ", None, 2);
        assert_eq!(start, 0);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_custom_single_char() {
        let (start, len) = trim(b"xxxhelloxxx", Some(b"x"), 3);
        assert_eq!(start, 3);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_custom_multi_char() {
        let (start, len) = trim(b"xyzhelloyzx", Some(b"xyz"), 3);
        assert_eq!(start, 3);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_range() {
        let (start, len) = trim(b"abchellodef", Some(b"a..f"), 3);
        assert_eq!(start, 3);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_no_change() {
        let (start, len) = trim(b"hello", None, 3);
        assert_eq!(start, 0);
        assert_eq!(len, 5);
    }

    #[test]
    fn test_trim_empty() {
        let (start, len) = trim(b"", None, 3);
        assert_eq!(start, 0);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_trim_all() {
        let (start, len) = trim(b"   ", None, 3);
        assert_eq!(start, 3);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_build_char_mask_simple() {
        let mask = build_char_mask(b"abc");
        assert!(mask[b'a' as usize]);
        assert!(mask[b'b' as usize]);
        assert!(mask[b'c' as usize]);
        assert!(!mask[b'd' as usize]);
    }

    #[test]
    fn test_build_char_mask_range() {
        let mask = build_char_mask(b"a..z");
        assert!(mask[b'a' as usize]);
        assert!(mask[b'm' as usize]);
        assert!(mask[b'z' as usize]);
        assert!(!mask[b'A' as usize]);
    }

    #[test]
    fn test_str_pad_right() {
        let result = str_pad(b"hello", 10, b" ", PAD_RIGHT);
        assert_eq!(result, b"hello     ");
    }

    #[test]
    fn test_str_pad_left() {
        let result = str_pad(b"hello", 10, b" ", PAD_LEFT);
        assert_eq!(result, b"     hello");
    }

    #[test]
    fn test_str_pad_both() {
        let result = str_pad(b"hello", 11, b" ", PAD_BOTH);
        assert_eq!(result, b"   hello   ");
    }

    #[test]
    fn test_str_pad_both_odd() {
        let result = str_pad(b"hello", 10, b" ", PAD_BOTH);
        assert_eq!(result, b"  hello   ");
    }

    #[test]
    fn test_str_pad_multi_char() {
        let result = str_pad(b"hello", 10, b"*-", PAD_RIGHT);
        assert_eq!(result, b"hello*-*-*");
    }

    #[test]
    fn test_str_pad_multi_char_partial() {
        let result = str_pad(b"hello", 11, b"abc", PAD_RIGHT);
        assert_eq!(result, b"helloabcabc");
    }

    #[test]
    fn test_str_pad_no_padding_needed() {
        let result = str_pad(b"hello", 5, b" ", PAD_RIGHT);
        assert_eq!(result, b"hello");
    }

    #[test]
    fn test_str_pad_shorter_than_input() {
        let result = str_pad(b"hello", 3, b" ", PAD_RIGHT);
        assert_eq!(result, b"hello");
    }
}
