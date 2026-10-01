.PHONY: build install test-setup test

build:
	cargo build

install:
	cargo install --path .

test-setup:
	make -C test all

test:
	cargo test
	exactly suite test
