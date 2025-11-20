use std::ptr;

use libc::c_char;
use libc::c_int;
use libc::c_void;
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

// FFI bindings to C wrapper functions
#[allow(dead_code)]
extern "C" {
    /// Allocate memory (wrapper for pemalloc)
    fn zend_oxidized_pemalloc(size: size_t, persistent: c_int) -> *mut c_void;

    /// Free memory (wrapper for pefree)
    fn zend_oxidized_pefree(ptr: *mut c_void, persistent: c_int);

    /// Allocate a zend_string (wrapper for zend_string_alloc)
    fn zend_oxidized_string_alloc(len: size_t, persistent: c_int) -> *mut ZendString;

    /// Release a zend_string (wrapper for zend_string_release_ex)
    fn zend_oxidized_string_release_ex(s: *mut ZendString, persistent: c_int);

    /// Get pointer to string data (wrapper for ZSTR_VAL)
    fn zend_oxidized_str_val(s: *mut ZendString) -> *mut c_char;

    /// Get string length (wrapper for ZSTR_LEN)
    fn zend_oxidized_str_len(s: *const ZendString) -> size_t;

    /// Set string length
    fn zend_oxidized_set_len(s: *mut ZendString, len: size_t);

    /// Set hash value
    fn zend_oxidized_set_hash(s: *mut ZendString, hash: u64);

    /// Set reference count
    fn zend_oxidized_set_refcount(s: *mut ZendString, refcount: u32);

    /// Set GC type info
    fn zend_oxidized_set_type_info(s: *mut ZendString, type_info: u32);

    /// Get the global empty string
    fn zend_oxidized_get_empty_string() -> *mut ZendString;
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
        // ZendString should be zero-sized (opaque)
        assert_eq!(std::mem::size_of::<ZendString>(), 0);
    }
}
