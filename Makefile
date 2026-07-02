# CID - Calibrated Inference Device
# Per-token validation for LLMs using pachinko mechanics

.PHONY: all release debug clean test install

all: release

release:
	cargo build --release

debug:
	cargo build

test:
	@echo "test" | ./target/release/cid

clean:
	cargo clean

install: release
	install -m 755 target/release/cid /usr/local/bin/cid

check: release test

size: release
	@ls -lh target/release/cid

bench: release
	@echo "=== CID Benchmark ==="
	@time echo "gate 42 math" | ./target/release/cid > /dev/null
	@time echo "beam 1,2,3,4,5 math" | ./target/release/cid > /dev/null
