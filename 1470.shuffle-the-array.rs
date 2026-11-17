/*
 * @lc app=leetcode id=1470 lang=rust
 *
 * [1470] Shuffle the Array
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (a, b) = nums.split_at(n as usize);
        a.iter().zip(b).flat_map(|(&x, &y)| [x, y]).collect()
    }
}
// @lc code=end
