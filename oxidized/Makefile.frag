# Makefile fragment for PHP Oxidized components
# Variables will be set by configure and substituted into Makefile

# Build the Rust library
oxidized/target/release/libphp_oxidized.a: oxidized/Cargo.toml oxidized/build.rs oxidized/src/lib.rs oxidized/src/zend/mod.rs oxidized/src/zend/memory/mod.rs oxidized/src/zend/memory/impls.rs oxidized/src/zend/hash/mod.rs oxidized/src/zend/hash/impls.rs oxidized/src/zend/operators/mod.rs oxidized/src/zend/operators/impls.rs oxidized/src/zend/strtod/mod.rs oxidized/src/zend/strtod/impls.rs oxidized/src/ext/mod.rs oxidized/src/ext/standard/mod.rs oxidized/src/ext/standard/base64/mod.rs oxidized/src/ext/standard/base64/impls.rs oxidized/src/ext/standard/baseconv/mod.rs oxidized/src/ext/standard/baseconv/impls.rs oxidized/src/ext/standard/levenshtein/mod.rs oxidized/src/ext/standard/levenshtein/impls.rs
	@echo "Building Rust library..."
	cd oxidized && cargo build --release
	@echo "Rust library built successfully"

# Generate C header from Rust code using cbindgen
oxidized/php_oxidized.h: oxidized/Cargo.toml oxidized/src/lib.rs oxidized/src/zend/mod.rs oxidized/src/zend/memory/mod.rs oxidized/src/zend/memory/impls.rs oxidized/src/zend/hash/mod.rs oxidized/src/zend/hash/impls.rs oxidized/src/zend/operators/mod.rs oxidized/src/zend/operators/impls.rs oxidized/src/zend/strtod/mod.rs oxidized/src/zend/strtod/impls.rs oxidized/src/ext/mod.rs oxidized/src/ext/standard/mod.rs oxidized/src/ext/standard/base64/mod.rs oxidized/src/ext/standard/base64/impls.rs oxidized/src/ext/standard/baseconv/mod.rs oxidized/src/ext/standard/baseconv/impls.rs oxidized/src/ext/standard/levenshtein/mod.rs oxidized/src/ext/standard/levenshtein/impls.rs
	@echo "Generating C header from Rust code..."
	cd oxidized && cbindgen --config cbindgen.toml --output php_oxidized.h
	@echo "C header generated successfully"

# Zend dependencies
Zend/zend_alloc.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h
Zend/zend_hash.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h
Zend/zend_operators.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h
Zend/zend_strtod.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h

# Extension dependencies
ext/standard/base64.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h
ext/standard/math.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h
ext/standard/levenshtein.lo: oxidized/target/release/libphp_oxidized.a oxidized/php_oxidized.h

# Clean targets
oxidized-clean:
	@if test -d oxidized; then cd oxidized && cargo clean; fi

# Test targets
oxidized-test:
	cd oxidized && cargo test --release

# Add to main clean target
clean: oxidized-clean

# Phony targets
.PHONY: oxidized-clean oxidized-test
