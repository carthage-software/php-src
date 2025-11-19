use std::str;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput,
    Overflow,
    Underflow,
}

pub fn parse_float(input: &str) -> Result<(f64, usize), ParseError> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(ParseError::InvalidInput);
    }

    let mut pos = 0;

    while pos < bytes.len() && (bytes[pos] == b' ' || bytes[pos] == b'\t') {
        pos += 1;
    }

    if pos >= bytes.len() {
        return Err(ParseError::InvalidInput);
    }

    let start_pos = pos;
    let is_negative = if bytes[pos] == b'-' {
        pos += 1;
        true
    } else if bytes[pos] == b'+' {
        pos += 1;
        false
    } else {
        false
    };

    if pos >= bytes.len() {
        return Err(ParseError::InvalidInput);
    }

    let mut has_digits = false;
    let mut has_dot = false;
    let mut has_exp = false;

    while pos < bytes.len() {
        match bytes[pos] {
            b'0'..=b'9' => {
                has_digits = true;
                pos += 1;
            }
            b'.' if !has_dot && !has_exp => {
                has_dot = true;
                pos += 1;
            }
            b'e' | b'E' if has_digits && !has_exp => {
                has_exp = true;
                pos += 1;
                if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                    pos += 1;
                }
                if pos >= bytes.len() || !bytes[pos].is_ascii_digit() {
                    break;
                }
            }
            _ => break,
        }
    }

    if !has_digits {
        return Err(ParseError::InvalidInput);
    }

    let float_str = match str::from_utf8(&bytes[start_pos..pos]) {
        Ok(s) => s,
        Err(_) => return Err(ParseError::InvalidInput),
    };

    match float_str.parse::<f64>() {
        Ok(value) => {
            if value.is_infinite() {
                if is_negative {
                    Err(ParseError::Underflow)
                } else {
                    Err(ParseError::Overflow)
                }
            } else {
                Ok((value, pos))
            }
        }
        Err(_) => Err(ParseError::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_integers() {
        assert_eq!(parse_float("123").unwrap(), (123.0, 3));
        assert_eq!(parse_float("-456").unwrap(), (-456.0, 4));
        assert_eq!(parse_float("+789").unwrap(), (789.0, 4));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_decimals() {
        assert_eq!(parse_float("3.14").unwrap(), (3.14, 4));
        assert_eq!(parse_float("-2.718").unwrap(), (-2.718, 6));
        assert_eq!(parse_float(".5").unwrap(), (0.5, 2));
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(parse_float("1e10").unwrap(), (1e10, 4));
        assert_eq!(parse_float("1.5e-10").unwrap(), (1.5e-10, 7));
        assert_eq!(parse_float("2.5E+3").unwrap(), (2500.0, 6));
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(parse_float("  123").unwrap(), (123.0, 5));
        assert_eq!(parse_float("\t\t456").unwrap(), (456.0, 5));
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_partial_parse() {
        assert_eq!(parse_float("123abc").unwrap(), (123.0, 3));
        assert_eq!(parse_float("3.14 remaining").unwrap(), (3.14, 4));
    }

    #[test]
    fn test_invalid_input() {
        assert!(parse_float("").is_err());
        assert!(parse_float("abc").is_err());
        assert!(parse_float("   ").is_err());
    }

    #[test]
    fn test_special_cases() {
        assert_eq!(parse_float("0").unwrap(), (0.0, 1));
        assert_eq!(parse_float("-0").unwrap(), (-0.0, 2));
        assert_eq!(parse_float("0.0").unwrap(), (0.0, 3));
    }
}
