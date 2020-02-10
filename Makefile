.PHONEY: clean test format

clean:
	rm -r target

doc:
	cargo doc --open

test:
	cargo test -- --nocapture

format:
	cargo fmt --all
