.PHONY: all
	
all:
	cargo build

run: all
	./target/debug/mile39
