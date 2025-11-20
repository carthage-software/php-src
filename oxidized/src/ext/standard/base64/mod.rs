use std::ptr;

use libc::c_int;
use libc::c_uchar;
use libc::size_t;

use crate::zend::string::create_zend_string_from_bytes;
use crate::zend::string::ZendString;

mod impls;

const PHP_BASE64_NO_PADDING: c_int = 1;

/// Encode input bytes to base64 string
///
/// # Safety
///
/// Caller must ensure input points to length valid bytes is a valid pointer
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_base64_encode(
    input: *const c_uchar,
    length: size_t,
    flags: c_int,
) -> *mut ZendString {
    if input.is_null() || length == 0 {
        return ptr::null_mut();
    }

    let input_slice = std::slice::from_raw_parts(input, length);
    let no_padding = (flags & PHP_BASE64_NO_PADDING) != 0;
    let encoded = impls::encode(input_slice, no_padding);

    create_zend_string_from_bytes(&encoded, false)
}

/// Decode base64 string to bytes
///
/// # Safety
///
/// Caller must ensure input points to length valid bytes.
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_base64_decode(
    input: *const c_uchar,
    length: size_t,
    strict: c_int,
) -> *mut ZendString {
    if input.is_null() || length == 0 {
        return ptr::null_mut();
    }

    let input_slice = std::slice::from_raw_parts(input, length);
    let strict_mode = strict != 0;
    let decoded = match impls::decode(input_slice, strict_mode) {
        Some(data) => data,
        None => return ptr::null_mut(),
    };

    create_zend_string_from_bytes(&decoded, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_returns_zend_string() {
        unsafe {
            let input = b"SGVsbG8=";
            let result = php_oxidized_base64_decode(input.as_ptr(), input.len(), 0);
            assert!(!result.is_null());
        }
    }

    #[test]
    fn test_decode_invalid_returns_null() {
        unsafe {
            let input = b"!!!invalid!!!";
            let result = php_oxidized_base64_decode(input.as_ptr(), input.len(), 1);
            assert!(result.is_null());
        }
    }
}
