test: build
	cargo test
	python3 scripts/generate_tests_cases.py | ./target/debug/isoformat
build:
	cargo build
install: release
	sudo cp target/release/isoformat /usr/local/bin/
release:
	cargo build --release
