build:
	cargo build

release:
	cargo build -r

help:
	./target/debug/adventofcode2022 -h

run:
	RUST_LOG=info ./target/debug/adventofcode2022


