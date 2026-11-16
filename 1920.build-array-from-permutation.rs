/*
 * @lc app=leetcode id=1920 lang=rust
 *
 * [1920] Build Array from Permutation
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for &i in &nums {
            result.push(nums[i as usize]);
        }

        result
    }
}
// @lc code=end
