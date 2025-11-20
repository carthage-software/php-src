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
*/

#include "php_oxidized_zend_string.h"

/* Memory allocation wrappers for Rust FFI */
void *zend_oxidized_pemalloc(size_t size, int persistent) {
  return pemalloc(size, persistent);
}

void zend_oxidized_pefree(void *ptr, int persistent) {
  pefree(ptr, persistent);
}

/* String allocation wrappers (wraps inline functions) */
zend_string *zend_oxidized_string_alloc(size_t len, int persistent) {
  return zend_string_alloc(len, persistent);
}

void zend_oxidized_string_release_ex(zend_string *s, int persistent) {
  zend_string_release_ex(s, persistent);
}

/* Field accessor wrappers (wraps macros) */
char *zend_oxidized_str_val(zend_string *s) { return ZSTR_VAL(s); }

size_t zend_oxidized_str_len(const zend_string *s) { return ZSTR_LEN(s); }

void zend_oxidized_set_len(zend_string *s, size_t len) { ZSTR_LEN(s) = len; }

void zend_oxidized_set_hash(zend_string *s, zend_ulong hash) {
  ZSTR_H(s) = hash;
}

void zend_oxidized_set_refcount(zend_string *s, uint32_t refcount) {
  GC_SET_REFCOUNT(s, refcount);
}

void zend_oxidized_set_type_info(zend_string *s, uint32_t type_info) {
  GC_TYPE_INFO(s) = type_info;
}

/* Global constants */
zend_string *zend_oxidized_get_empty_string(void) { return zend_empty_string; }
