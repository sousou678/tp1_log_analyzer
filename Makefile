run:
	cargo run -- samples/auth_sample.log

test:
	cargo test

lint:
	cargo fmt --check
	cargo clippy -- -D warnings

all: lint test run
