mod impls;

use libc::{c_char, c_int, size_t};
use std::slice;

/// Compute the Levenshtein distance between two strings with given costs
///
/// # Safety
///
/// Caller must ensure s1 points to len1 valid bytes and s2 points to len2 valid bytes
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_levenshtein(
    s1: *const c_char,
    len1: size_t,
    s2: *const c_char,
    len2: size_t,
    cost_ins: c_int,
    cost_rep: c_int,
    cost_del: c_int,
) -> c_int {
    if s1.is_null() || s2.is_null() {
        return -1;
    }

    let s1_slice = slice::from_raw_parts(s1 as *const u8, len1);
    let s2_slice = slice::from_raw_parts(s2 as *const u8, len2);

    impls::levenshtein(s1_slice, s2_slice, cost_ins, cost_rep, cost_del)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_basic() {
        unsafe {
            let s1 = CString::new("kitten").unwrap();
            let s2 = CString::new("sitting").unwrap();
            let result = php_oxidized_levenshtein(s1.as_ptr(), 6, s2.as_ptr(), 7, 1, 1, 1);
            assert_eq!(result, 3);
        }
    }

    #[test]
    fn test_ffi_null_pointers() {
        unsafe {
            let result =
                php_oxidized_levenshtein(std::ptr::null(), 0, std::ptr::null(), 0, 1, 1, 1);
            assert_eq!(result, -1);
        }
    }
}
