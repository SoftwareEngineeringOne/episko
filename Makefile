all:
	echo "Nothing selected"

run:
	bun run tauri dev

run_gui: run

run_cli:
	cargo run --p episkos_cli

test:
	bun run test
	cargo test --all-targets

format:
	rustfmt ./**/*.rs
	bun run format

.PHONY: run run_gui run_cli test format all
