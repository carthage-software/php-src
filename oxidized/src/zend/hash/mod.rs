use libc::c_int;
use libc::c_uint;
use libc::c_void;

mod impls;

#[repr(C)]
pub struct BucketFFI {
    pub h: u64,
    pub key: *mut c_void,
    pub val: *mut c_void,
}

/// Rehash the given buckets
///
/// # Safety
///
/// Caller must ensure buckets points to num_used valid BucketFFI elements
#[no_mangle]
pub unsafe extern "C" fn zend_oxidized_hash_rehash(
    buckets: *mut BucketFFI,
    num_used: c_uint,
) -> c_int {
    if buckets.is_null() || num_used == 0 {
        return 0;
    }

    let mut bucket_data = Vec::with_capacity(num_used as usize);
    for i in 0..num_used as usize {
        let bucket = &*buckets.add(i);
        bucket_data.push(impls::BucketData {
            h: bucket.h,
            key_offset: if bucket.key.is_null() {
                u32::MAX
            } else {
                i as u32
            },
            val_offset: if bucket.val.is_null() {
                u32::MAX
            } else {
                i as u32
            },
        });
    }

    let new_used = impls::rehash(&mut bucket_data, num_used as usize);

    new_used as c_int
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_ffi_null() {
        unsafe {
            let result = zend_oxidized_hash_rehash(ptr::null_mut(), 0);
            assert_eq!(result, 0);
        }
    }
}
