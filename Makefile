.PHONY: fmt lint test ci doc

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings -A unused

test:
	cargo test

ci: fmt lint test

doc:
	cargo doc
