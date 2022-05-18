.PHONY: all test
	
all:
	cargo build

format:
	find src tests -type f -exec rustfmt {} \;

test:
	cargo test
