// https://leetcode.com/problems/k-th-symbol-in-grammar/
// 779. K-th Symbol in Grammar
pub struct Solution;
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut res = 0;
        let mut k = k - 1;
        while k != 0 {
            res = 1 - res;
            k = k & (k - 1);
        }
        res
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
