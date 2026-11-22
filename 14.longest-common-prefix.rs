/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = &strs[0];
        let end = first
            .bytes()
            .enumerate()
            .position(|(i, e)| !strs[1..].iter().all(|s| s.as_bytes().get(i) == Some(&e)))
            .unwrap_or(first.len());

        first[..end].to_owned()
    }
}
// @lc code=end
