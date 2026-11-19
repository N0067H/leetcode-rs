/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut min_price = prices[0];

        for price in prices {
            min_price = min_price.min(price);
            best = best.max(price - min_price);
        }

        best
    }
}
// @lc code=end
