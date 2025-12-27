.PHONY: fmt lint test ci doc

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings -W clippy::pedantic -A unused

test:
	cargo test

all: fmt lint test

doc:
	cargo doc
