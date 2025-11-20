use std::ptr;

use libc::c_char;
use libc::c_int;
use libc::size_t;

/// Opaque type representing a zend_string
///
/// This is intentionally opaque to prevent unsafe direct field access from Rust.
#[repr(C)]
pub struct ZendString {
    _private: [u8; 0],
}

#[allow(dead_code)]
const ZEND_MM_ALIGNMENT: usize = 8;

#[cfg(not(test))]
extern "C" {
    fn zend_oxidized_string_alloc(len: size_t, persistent: c_int) -> *mut ZendString;
    fn zend_oxidized_str_val(s: *mut ZendString) -> *mut c_char;
    fn zend_oxidized_get_empty_string() -> *mut ZendString;
}

#[cfg(test)]
unsafe fn zend_oxidized_string_alloc(len: size_t, _persistent: c_int) -> *mut ZendString {
    let layout = std::alloc::Layout::from_size_align(len + 64, 8).unwrap();
    std::alloc::alloc(layout) as *mut ZendString
}

#[cfg(test)]
unsafe fn zend_oxidized_str_val(s: *mut ZendString) -> *mut c_char {
    (s as *mut u8).add(32) as *mut c_char
}

#[cfg(test)]
#[allow(static_mut_refs)] // I know what I'm doing, i think, not sure
unsafe fn zend_oxidized_get_empty_string() -> *mut ZendString {
    static mut EMPTY: [u8; 64] = [0; 64];
    EMPTY.as_mut_ptr() as *mut ZendString
}

/// Create a zend_string from a byte slice using zero-copy pattern
///
/// # Safety
///
/// This function allocates a zend_string using PHP's memory allocator.
/// The returned pointer must be managed by PHP's reference counting system.
/// The string is returned with refcount=1.
///
/// # Arguments
///
/// * `data` - The byte slice to copy into the zend_string
/// * `persistent` - Whether to use persistent (cross-request) allocation
///
/// # Returns
///
/// A pointer to a newly allocated zend_string, or null if allocation fails.
/// The caller is responsible for releasing the string when done.
pub unsafe fn create_zend_string_from_bytes(data: &[u8], persistent: bool) -> *mut ZendString {
    let len = data.len();

    if len == 0 {
        return zend_oxidized_get_empty_string();
    }

    let s = zend_oxidized_string_alloc(len, persistent as c_int);

    if s.is_null() {
        return ptr::null_mut();
    }

    let val_ptr = zend_oxidized_str_val(s);

    ptr::copy_nonoverlapping(data.as_ptr(), val_ptr as *mut u8, len);

    *val_ptr.add(len) = 0;

    s
}

/// Create a zend_string from a Vec<u8> using zero-copy pattern
///
/// # Safety
///
/// Same safety requirements as create_zend_string_from_bytes.
/// The Vec is consumed and its data is copied into the zend_string.
pub unsafe fn create_zend_string_from_vec(data: Vec<u8>, persistent: bool) -> *mut ZendString {
    create_zend_string_from_bytes(&data, persistent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zend_string_opaque_type() {
        assert_eq!(std::mem::size_of::<ZendString>(), 0);
    }
}
