//! 凑零钱问题
//!
//! 先看下题目：
//! 给你 k 种面值的硬币，面值分别为 c1, c2 ... ck，每种硬币的数量无限，再给一个总金额 amount，
//! 问你最少需要几枚硬币凑出这个金额，如果不可能凑出，算法返回 -1 。
//!
//! 算法的函数签名如下：
//! // coins 中是可选硬币面值，amount 是目标金额
//! int coinChange(int[] coins, int amount);
//!

macro_rules! check_inputs {
    //assert no '0' coin
    ($coins:ident, $amount:ident) => {
        assert!($coins.iter().all(|&v| v != 0));
        match ($coins.is_empty(), $amount) {
            (true, 0) => return 0,
            (true, _) | (false, 0) => return -1,
            (_, amount) if amount < 0 => return -1,
            _ => (),
        }
    };
}

/// 暴力穷举
pub fn make_change_classic(coins: &[i32], amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    if amount < 0 {
        return -1;
    }

    let mut res = i32::MAX;
    for &coin in coins {
        match make_change_classic(coins, amount - coin) {
            -1 => (),
            sub => {
                res = std::cmp::min(res, 1 + sub);
            }
        }
    }

    if res != i32::MAX {
        res
    } else {
        -1
    }
}

pub fn make_change_iter(coins: &[i32], amount: i32) -> i32 {
    check_inputs!(coins, amount);
    let max_amount = amount + 1;
    // 为啥 dp 数组初始化为 amount + 1 呢，因为凑成 amount 金额的硬币数最多只
    // 可能等于 amount（全用 1 元面值的硬币），所以初始化为 amount + 1 就相当
    // 于初始化为正无穷，便于后续取最小值。
    let mut dp = vec![max_amount; (amount + 1) as usize];
    dp[0] = 0;

    for i in 0..dp.len() {
        for &coin in coins {
            if i as i32 - coin < 0 {
                continue;
            }

            dp[i] = std::cmp::min(dp[i], 1 + dp[i - coin as usize]);
        }
    }

    let v = dp[amount as usize];
    if v == max_amount {
        -1
    } else {
        v
    }
}
