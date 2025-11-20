use std::ptr;
use std::slice;

use libc::c_char;
use libc::c_int;
use libc::size_t;

use crate::zend::string::{create_zend_string_from_bytes, ZendString};

mod impls;

/// Repeat a string mult times
///
/// # Safety
///
/// Caller must ensure input points to len valid bytes
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_str_repeat(
    input: *const c_char,
    len: size_t,
    mult: size_t,
) -> *mut ZendString {
    if input.is_null() || len == 0 || mult == 0 {
        return ptr::null_mut();
    }

    let input_slice = slice::from_raw_parts(input as *const u8, len);
    let result = impls::str_repeat(input_slice, mult);

    create_zend_string_from_bytes(&result, false)
}

#[repr(C)]
pub struct TrimResult {
    pub trimmed_start: size_t,
    pub trimmed_len: size_t,
    pub changed: c_int,
}

/// Trim characters from string, returns offset and length
///
/// # Safety
///
/// Caller must ensure str and what (if not null) point to valid bytes
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_trim(
    str: *const c_char,
    str_len: size_t,
    what: *const c_char,
    what_len: size_t,
    mode: c_int,
) -> TrimResult {
    if str.is_null() || str_len == 0 {
        return TrimResult {
            trimmed_start: 0,
            trimmed_len: 0,
            changed: if str_len == 0 { 0 } else { 1 },
        };
    }

    let str_slice = slice::from_raw_parts(str as *const u8, str_len);
    let what_slice = if what.is_null() || what_len == 0 {
        None
    } else {
        Some(slice::from_raw_parts(what as *const u8, what_len))
    };

    let (start, len) = impls::trim(str_slice, what_slice, mode);

    TrimResult {
        trimmed_start: start,
        trimmed_len: len,
        changed: if start == 0 && len == str_len { 0 } else { 1 },
    }
}

/// Pad string to specified length
///
/// # Safety
///
/// Caller must ensure input and pad_str point to valid bytes
#[no_mangle]
pub unsafe extern "C" fn php_oxidized_str_pad(
    input: *const c_char,
    input_len: size_t,
    pad_length: size_t,
    pad_str: *const c_char,
    pad_str_len: size_t,
    pad_type: c_int,
) -> *mut ZendString {
    if input.is_null() || pad_str.is_null() || pad_str_len == 0 {
        return ptr::null_mut();
    }

    if pad_length <= input_len {
        return ptr::null_mut();
    }

    let input_slice = slice::from_raw_parts(input as *const u8, input_len);
    let pad_str_slice = slice::from_raw_parts(pad_str as *const u8, pad_str_len);

    let result = impls::str_pad(input_slice, pad_length, pad_str_slice, pad_type);

    create_zend_string_from_bytes(&result, false)
}
