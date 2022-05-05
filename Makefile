.PHONY: all test
	
all:
	cargo build

run: all
	./target/debug/mile39

format:
	find src tests -type f -exec rustfmt {} \;

test:
	cargo test
