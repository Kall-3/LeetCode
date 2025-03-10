use std::collections::HashMap;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }

        let mut left = 0;
        let mut best = 0;

        for i in 1..prices.len() {
            best = best.max(prices[i] - prices[left]);

            if prices[left] > prices[i] {
                left = i;
            }
        }

        best
    }
}