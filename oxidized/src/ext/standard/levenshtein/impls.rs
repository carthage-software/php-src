pub fn levenshtein(s1: &[u8], s2: &[u8], cost_ins: i32, cost_rep: i32, cost_del: i32) -> i32 {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return (len2 as i32) * cost_ins;
    }
    if len2 == 0 {
        return (len1 as i32) * cost_del;
    }

    let mut p1 = vec![0i32; len2 + 1];
    let mut p2 = vec![0i32; len2 + 1];

    for (i, p1_elem) in p1.iter_mut().enumerate() {
        *p1_elem = (i as i32) * cost_ins;
    }

    for (i, &s1_byte) in s1.iter().enumerate() {
        p2[0] = ((i + 1) as i32) * cost_del;

        for j in 0..len2 {
            let c0 = p1[j] + if s1_byte == s2[j] { 0 } else { cost_rep };
            let c1 = p1[j + 1] + cost_del;
            let c2 = p2[j] + cost_ins;

            p2[j + 1] = c0.min(c1).min(c2);
        }

        std::mem::swap(&mut p1, &mut p2);
    }

    p1[len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_strings() {
        assert_eq!(levenshtein(b"", b"", 1, 1, 1), 0);
        assert_eq!(levenshtein(b"hello", b"", 1, 1, 1), 5);
        assert_eq!(levenshtein(b"", b"world", 1, 1, 1), 5);
    }

    #[test]
    fn test_equal_strings() {
        assert_eq!(levenshtein(b"hello", b"hello", 1, 1, 1), 0);
    }

    #[test]
    fn test_simple_distance() {
        assert_eq!(levenshtein(b"kitten", b"sitting", 1, 1, 1), 3);
        assert_eq!(levenshtein(b"saturday", b"sunday", 1, 1, 1), 3);
    }

    #[test]
    fn test_custom_costs() {
        assert_eq!(levenshtein(b"abc", b"def", 1, 2, 1), 6);
        assert_eq!(levenshtein(b"abc", b"def", 2, 1, 1), 3);
    }
}
