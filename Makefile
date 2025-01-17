test: build
	cargo test
	python3 scripts/generate_tests_cases.py | ./target/debug/isodatetime
build:
	cargo build
install: release
	sudo cp target/release/isodatetime /usr/local/bin/
release:
	cargo build --release
