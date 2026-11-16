/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .scan(0, |state, item| {
                *state += item;
                Some(*state)
            })
            .collect()
    }
}
// @lc code=end
