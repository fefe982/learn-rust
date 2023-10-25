// https://leetcode.com/problems/k-th-symbol-in-grammar/
// 779. K-th Symbol in Grammar
pub struct Solution;
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        ((k - 1).count_ones() % 2) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_grammar() {
        assert_eq!(Solution::kth_grammar(1, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 1), 0);
        assert_eq!(Solution::kth_grammar(2, 2), 1);
    }
}
