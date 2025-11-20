/*
   +----------------------------------------------------------------------+
   | PHP Oxidized Components - Zend String Wrappers                      |
   +----------------------------------------------------------------------+
   | Copyright (c) The PHP Group                                          |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | https://www.php.net/license/3_01.txt                                 |
   +----------------------------------------------------------------------+
   |                                                                      |
   | ZERO-COPY PATTERN FOR RUST-TO-C STRING INTEGRATION                  |
   | ==================================================                    |
   |                                                                      |
   | This file provides wrapper functions that enable Rust code to        |
   | directly allocate and return zend_string objects using PHP's memory |
   | allocator (pemalloc). This achieves a zero-copy pattern that         |
   | eliminates redundant allocations and memcpy operations.              |
   |                                                                      |
   | PERFORMANCE COMPARISON:                                              |
   |                                                                      |
   | Legacy Pattern (2 allocations + 2 copies):                           |
   |   Rust Vec<u8> → malloc() → memcpy #1 → zend_string_init()          |
   |                  → memcpy #2 → free(malloc)                          |
   |                                                                      |
   | Zero-Copy Pattern (1 allocation + 1 copy):                           |
   |   Rust Vec<u8> → pemalloc(zend_string) → memcpy once → return       |
   |                                                                      |
   | PERFORMANCE GAIN: 50% fewer allocations, 50% fewer memcpy calls     |
   |                                                                      |
   | USAGE EXAMPLE:                                                       |
   |                                                                      |
   |   // Instead of the legacy pattern:                                 |
   |   size_t len;                                                        |
   |   char *result = php_oxidized_base64_encode(data, size, &len, 0);   |
   |   zend_string *str = zend_string_init(result, len, 0);              |
   |   free(result);                                                      |
   |                                                                      |
   |   // Use the zero-copy pattern:                                     |
   |   zend_string *str = php_oxidized_base64_encode_zstr(data, size, 0);|
   |                                                                      |
   | SAFETY GUARANTEES:                                                   |
   | - All returned zend_string objects have refcount=1                   |
   | - Proper null termination is always added                            |
   | - Uses PHP's memory allocator (pemalloc/pefree)                      |
   | - Compatible with zend_string reference counting                     |
   |                                                                      |
   +----------------------------------------------------------------------+
*/

#ifndef PHP_OXIDIZED_ZEND_STRING_H
#define PHP_OXIDIZED_ZEND_STRING_H

#include "zend.h"
#include "zend_alloc.h"
#include "zend_string.h"

BEGIN_EXTERN_C()

/* Memory allocation wrappers for Rust FFI */
void *zend_oxidized_pemalloc(size_t size, int persistent);
void zend_oxidized_pefree(void *ptr, int persistent);

/* String allocation wrappers (wraps inline functions) */
zend_string *zend_oxidized_string_alloc(size_t len, int persistent);
void zend_oxidized_string_release_ex(zend_string *s, int persistent);

/* Field accessor wrappers (wraps macros) */
char *zend_oxidized_str_val(zend_string *s);
size_t zend_oxidized_str_len(const zend_string *s);
void zend_oxidized_set_len(zend_string *s, size_t len);
void zend_oxidized_set_hash(zend_string *s, zend_ulong hash);
void zend_oxidized_set_refcount(zend_string *s, uint32_t refcount);
void zend_oxidized_set_type_info(zend_string *s, uint32_t type_info);

/* Global constants */
zend_string *zend_oxidized_get_empty_string(void);

END_EXTERN_C()

#endif /* PHP_OXIDIZED_ZEND_STRING_H */
