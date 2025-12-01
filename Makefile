fmt:
	cargo fmt

check: fmt lint
	cargo check

test: check
	cargo test