#[derive(Copy, Clone)]
pub struct BucketData {
    #[allow(dead_code)]
    pub h: u64,
    #[allow(dead_code)]
    pub key_offset: u32,
    pub val_offset: u32,
}

pub fn rehash(buckets: &mut [BucketData], used: usize) -> usize {
    if used == 0 {
        return 0;
    }

    let mut write_pos = 0;

    for read_pos in 0..used {
        if buckets[read_pos].val_offset != u32::MAX {
            if write_pos != read_pos {
                buckets[write_pos] = buckets[read_pos];
            }
            write_pos += 1;
        }
    }

    write_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rehash_no_holes() {
        let mut buckets = vec![
            BucketData {
                h: 1,
                key_offset: 0,
                val_offset: 0,
            },
            BucketData {
                h: 2,
                key_offset: 1,
                val_offset: 1,
            },
            BucketData {
                h: 3,
                key_offset: 2,
                val_offset: 2,
            },
        ];
        let new_used = rehash(&mut buckets, 3);
        assert_eq!(new_used, 3);
    }

    #[test]
    fn test_rehash_with_holes() {
        let mut buckets = vec![
            BucketData {
                h: 1,
                key_offset: 0,
                val_offset: 0,
            },
            BucketData {
                h: 0,
                key_offset: 0,
                val_offset: u32::MAX,
            },
            BucketData {
                h: 3,
                key_offset: 2,
                val_offset: 2,
            },
        ];
        let new_used = rehash(&mut buckets, 3);
        assert_eq!(new_used, 2);
        assert_eq!(buckets[0].h, 1);
        assert_eq!(buckets[1].h, 3);
    }

    #[test]
    fn test_rehash_empty() {
        let mut buckets = vec![];
        let new_used = rehash(&mut buckets, 0);
        assert_eq!(new_used, 0);
    }
}
