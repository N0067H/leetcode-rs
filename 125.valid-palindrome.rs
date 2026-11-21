/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut iter = s
            .as_bytes()
            .iter()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
            if front != back {
                return false;
            }
        }
        true
    }
}
// @lc code=end
