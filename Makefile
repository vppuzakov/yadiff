.phony: lint audit check build run docs

-include .env

lint:
	@cargo fmt -- --check
	@cargo clippy

audit:
	@cargo audit

check: lint audit

build:
	@cargo build --release

run:
	@target/release/yadiff --local ${YADIFF_PATH} --remote ${YADIFF_REMOTE} --token ${YADIFF_TOKEN}

docs:
	@cargo doc --open
