use std::ptr;
use std::slice;

use libc::c_char;
use libc::size_t;

use crate::zend::string::{create_zend_string_from_bytes, ZendString};

mod impls;

/// Convert string to lowercase
///
/// # Safety
///
/// Caller must ensure input points to len valid bytes and output has space for len bytes
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_str_tolower(
    input: *const c_char,
    len: size_t,
    output: *mut c_char,
) -> size_t {
    if input.is_null() || output.is_null() || len == 0 {
        return 0;
    }

    let input_slice = slice::from_raw_parts(input as *const u8, len);
    let result = impls::to_lowercase(input_slice);

    ptr::copy_nonoverlapping(result.as_ptr(), output as *mut u8, result.len());

    result.len()
}

/// Convert string to uppercase
///
/// # Safety
///
/// Caller must ensure input points to len valid bytes and output has space for len bytes
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_str_toupper(
    input: *const c_char,
    len: size_t,
    output: *mut c_char,
) -> size_t {
    if input.is_null() || output.is_null() || len == 0 {
        return 0;
    }

    let input_slice = slice::from_raw_parts(input as *const u8, len);
    let result = impls::to_uppercase(input_slice);

    ptr::copy_nonoverlapping(result.as_ptr(), output as *mut u8, result.len());

    result.len()
}

/// Perform bitwise OR on two byte sequences
///
/// # Safety
///
/// Caller must ensure op1 points to len1 bytes, op2 to len2 bytes.
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_bitwise_or(
    op1: *const c_char,
    len1: size_t,
    op2: *const c_char,
    len2: size_t,
) -> *mut ZendString {
    if op1.is_null() || op2.is_null() {
        return ptr::null_mut();
    }

    if len1 == 0 && len2 == 0 {
        return create_zend_string_from_bytes(&[], false);
    }

    let op1_slice = slice::from_raw_parts(op1 as *const u8, len1);
    let op2_slice = slice::from_raw_parts(op2 as *const u8, len2);

    let result_vec = impls::bitwise_or(op1_slice, op2_slice);

    create_zend_string_from_bytes(&result_vec, false)
}

/// Perform bitwise AND on two byte sequences
///
/// # Safety
///
/// Caller must ensure op1 points to len1 bytes, op2 to len2 bytes.
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_bitwise_and(
    op1: *const c_char,
    len1: size_t,
    op2: *const c_char,
    len2: size_t,
) -> *mut ZendString {
    if op1.is_null() || op2.is_null() {
        return ptr::null_mut();
    }

    let op1_slice = slice::from_raw_parts(op1 as *const u8, len1);
    let op2_slice = slice::from_raw_parts(op2 as *const u8, len2);

    let result_vec = impls::bitwise_and(op1_slice, op2_slice);

    create_zend_string_from_bytes(&result_vec, false)
}

/// Perform bitwise XOR on two byte sequences
///
/// # Safety
///
/// Caller must ensure op1 points to len1 bytes, op2 to len2 bytes.
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_bitwise_xor(
    op1: *const c_char,
    len1: size_t,
    op2: *const c_char,
    len2: size_t,
) -> *mut ZendString {
    if op1.is_null() || op2.is_null() {
        return ptr::null_mut();
    }

    let op1_slice = slice::from_raw_parts(op1 as *const u8, len1);
    let op2_slice = slice::from_raw_parts(op2 as *const u8, len2);

    let result_vec = impls::bitwise_xor(op1_slice, op2_slice);

    create_zend_string_from_bytes(&result_vec, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_tolower() {
        unsafe {
            let input = CString::new("HELLO").unwrap();
            let mut output = vec![0u8; 5];
            let len =
                zend_oxidized_str_tolower(input.as_ptr(), 5, output.as_mut_ptr() as *mut c_char);
            assert_eq!(len, 5);
            assert_eq!(&output[..len], b"hello");
        }
    }

    #[test]
    fn test_ffi_toupper() {
        unsafe {
            let input = CString::new("hello").unwrap();
            let mut output = vec![0u8; 5];
            let len =
                zend_oxidized_str_toupper(input.as_ptr(), 5, output.as_mut_ptr() as *mut c_char);
            assert_eq!(len, 5);
            assert_eq!(&output[..len], b"HELLO");
        }
    }
}
