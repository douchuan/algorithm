TEST = rustup run nightly cargo test
BENCH = rustup run nightly cargo bench
BUILD = rustup run nightly cargo build

build:
	$(BUILD)

test:
	$(TEST)

test_dp:
	$(TEST) dp

test_math:
	$(TEST) math

bench:
	$(BENCH)

bench_dp:
	$(BENCH) -- dp

bench_math:
	$(BENCH) -- math
