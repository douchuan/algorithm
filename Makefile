CARGO = RUST_BACKTRACE=1 rustup run nightly cargo
TEST = $(CARGO) test
BENCH = $(CARGO) bench
BUILD = $(CARGO) build

build:
	@$(BUILD)

clippy:
	@$(CARGO) clippy

test:
	#@$(TEST) -- --nocapture
	@$(TEST) $(ARGS)

bench:
	@$(BENCH)

eg_quadratic:
	@$(CARGO) run --example quadratic

stats:
	@echo "codes: "
	@cloc . --exclude-dir=target
	@echo
	@echo
	@echo "commits: "
	@git log --oneline | wc -l
	@echo "first commit: "
	@git rev-list --max-parents=0 HEAD | git --no-pager log --pretty=%cd --date=short --stdin
	@echo
	@echo
	@echo "disk:"
	@du -d 1 -h target