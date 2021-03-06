#include "fib.h"

int MEMO[100] = { 0 };

size_t fib_cache_result_c(size_t n)
{
    if (0 == n) return 0;
    if (1 == n || 2 == n) return 1;
    if (0 != MEMO[n]) return MEMO[n];
    MEMO[n] = fib_cache_result_c(n - 1) + fib_cache_result_c(n - 2);
    return MEMO[n];
}

size_t fib_classic_iteration_for_c(size_t n)
{
    if (0 == n) return 0;
    if (1 == n || 2 == n) return 1;
    size_t prev = 1, curr = 1;
    for (size_t i = 3; i <= n; i++) {
        size_t sum = prev + curr;
        prev = curr;
        curr = sum;
    }
    return curr;
}
