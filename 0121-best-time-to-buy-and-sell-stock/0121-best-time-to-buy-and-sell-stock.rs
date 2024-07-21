impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        use std::cmp::min;
        use std::cmp::max;

        let mut buy = prices[0];
        let mut profit = 0;

        for price in prices {
            profit = max(profit, price - buy);
            buy = min(buy, price);
        }

        profit
    }
}