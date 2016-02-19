bin: bin/accelerate bin/accelerate-postgres

bin/accelerate:
	cargo build
	mkdir -p bin
	mv target/debug/accelerate $@

bin/accelerate-%:
	cargo build --features driver_$*
	mkdir -p bin
	mv target/debug/accelerate $@

clean:
	cargo clean
	rm -rf bin

test:
	cargo test

.PHONY: clean test
