impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut lowest_price = i32::MAX;
        let mut highest_profit = 0;

        for price in &prices {
            if price < &lowest_price {
                lowest_price = *price;
            } else if price - &lowest_price > highest_profit {
                highest_profit = *price - lowest_price;
            }
        }
        

        highest_profit
    }
}