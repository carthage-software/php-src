use std::slice;
use std::str;

use libc::c_char;
use libc::c_double;
use libc::size_t;

mod impls;

/// Convert string to double
///
/// # Safety
///
/// Caller must ensure str_ptr points to len valid bytes
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_strtod(
    str_ptr: *const c_char,
    len: size_t,
    endptr: *mut *const c_char,
) -> c_double {
    if str_ptr.is_null() || len == 0 {
        if !endptr.is_null() {
            *endptr = str_ptr;
        }
        return 0.0;
    }

    let input_slice = slice::from_raw_parts(str_ptr as *const u8, len);

    let input_str = match str::from_utf8(input_slice) {
        Ok(s) => s,
        Err(_) => {
            if !endptr.is_null() {
                *endptr = str_ptr;
            }
            return 0.0;
        }
    };

    match impls::parse_float(input_str) {
        Ok((value, consumed)) => {
            if !endptr.is_null() {
                *endptr = str_ptr.add(consumed);
            }
            value
        }
        Err(_) => {
            if !endptr.is_null() {
                *endptr = str_ptr;
            }
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_ffi_simple() {
        unsafe {
            let input = CString::new("123.45").unwrap();
            let mut endptr: *const c_char = ptr::null();
            let result = zend_oxidized_strtod(input.as_ptr(), 6, &mut endptr);
            assert_eq!(result, 123.45);
            assert_eq!(endptr, input.as_ptr().add(6));
        }
    }

    #[test]
    fn test_ffi_partial() {
        unsafe {
            let input = CString::new("42.5abc").unwrap();
            let mut endptr: *const c_char = ptr::null();
            let result = zend_oxidized_strtod(input.as_ptr(), 7, &mut endptr);
            assert_eq!(result, 42.5);
            assert_eq!(endptr, input.as_ptr().add(4));
        }
    }

    #[test]
    fn test_ffi_null() {
        unsafe {
            let result = zend_oxidized_strtod(ptr::null(), 0, ptr::null_mut());
            assert_eq!(result, 0.0);
        }
    }
}
