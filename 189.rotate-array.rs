/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();
        nums.rotate_right(k);
    }
}
// @lc code=end
