# PHP Oxidized - Build Instructions

PHP Oxidized provides Rust implementations for performance-critical PHP components including memory management, string operations, hash tables, and standard library functions.

## Prerequisites

- Rust toolchain (rustc, cargo)
- cbindgen (optional): `cargo install cbindgen`

## Build with PHP Oxidized

```bash
./buildconf --force
./configure --enable-php-oxidized
make -j$(nproc)
```

## Build without PHP Oxidized

```bash
./buildconf --force
./configure
make -j$(nproc)
```

## Verify

```bash
# Check if PHP Oxidized is enabled
./sapi/cli/php -i | grep -i oxidized

# Or check config header
grep HAVE_PHP_OXIDIZED main/php_config.h
```

## Test

```bash
# Run Rust tests
cd oxidized && cargo test

# Run PHP tests
make test
```
