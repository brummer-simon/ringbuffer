.PHONEY: clean test format doc

clean:
	rm -r target

doc:
	cargo doc --open

test:
	cargo test -- --nocapture

benchmark:
	cargo bench -- --nocapture

format:
	cargo fmt --all

