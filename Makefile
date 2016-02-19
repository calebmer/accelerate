bin: bin/accelerate bin/accelerate-postgres

bin/accelerate:
	cargo build
	mv target/debug/accelerate $@

bin/accelerate-%:
	cargo build --features driver-$*
	mv target/debug/accelerate $@

clean:
	cargo clean

test:
	cargo test

.PHONY: clean test
