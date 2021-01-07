.phony: lint build run docs

lint:
	@cargo fmt -- --check
	@cargo check
	@cargo clippy

build:
	@cargo build --release

run:
	@target/release/yadiff

docs:
	@cargo doc --open
