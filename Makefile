.DEFAULT_GOAL := run

BIN := tlog
RELEASE_BIN := target/release/$(BIN)

$(RELEASE_BIN): src Cargo.toml
	cargo build --release

run: $(RELEASE_BIN)
	$(RELEASE_BIN)

debug:
	cargo build

release:
	cargo build --release

clean:
	rm -rf Cargo.lock target

.PHONY: run debug release clean
