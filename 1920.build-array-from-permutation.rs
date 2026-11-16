/*
 * @lc app=leetcode id=1920 lang=rust
 *
 * [1920] Build Array from Permutation
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&i| nums[i as usize]).collect()
    }
}
// @lc code=end
