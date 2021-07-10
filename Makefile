CARGO = RUST_BACKTRACE=1 rustup run nightly cargo
TEST = $(CARGO) test
BENCH = $(CARGO) bench
BUILD = $(CARGO) build

build:
	$(BUILD)

clippy:
	$(CARGO) clippy

test:
	#$(TEST) -- --nocapture
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

stats:
	@echo "codes: "
	@cloc . --exclude-dir=target
	@echo
	@echo "commits: "
	@git log --oneline | wc -l