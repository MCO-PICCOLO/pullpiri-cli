.PHONY: default build
build:
	cargo build --release
	cp ./target/release/pullpiri-cli ./pullpiri