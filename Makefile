.phony: lint build run docs

-include .env

lint:
	@cargo fmt -- --check
	@cargo clippy

build:
	@cargo build --release

run:
	@target/release/yadiff --local ${YADIFF_PATH} --remote ${YADIFF_REMOTE} --token ${YADIFF_TOKEN}

docs:
	@cargo doc --open
