.PHONEY: clean test format

clean:
	rm -r target

test:
	cargo test -- --nocapture

format:
	cargo fmt --all
