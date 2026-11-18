/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&num| num != 0);
        nums.resize(len, 0);
    }
}
// @lc code=end
