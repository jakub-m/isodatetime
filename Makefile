build:
	cargo build
test: build
	cargo test
	python3 scripts/generate_tests_cases.py | ./target/debug/isodatetime
