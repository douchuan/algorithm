TEST = rustup run nightly cargo test
BENCH = rustup run nightly cargo bench

test:
	$(TEST)

test_fib:
	$(TEST) fib

test_coin:
	$(TEST) coin

bench:
	$(BENCH)

bench_fib:
	$(BENCH) -- fib

bench_coin:
	$(BENCH) -- coin