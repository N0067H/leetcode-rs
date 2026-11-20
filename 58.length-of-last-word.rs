/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last = s.trim().split(' ').last().unwrap();
        last.len() as i32
    }
}
// @lc code=end
