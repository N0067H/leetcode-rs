/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::with_capacity(word1.len() + word2.len());

        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        loop {
            match (iter1.next(), iter2.next()) {
                (None, None) => break,
                (a, b) => {
                    result.extend(a);
                    result.extend(b);
                }
            }
        }

        result
    }
}
// @lc code=end
