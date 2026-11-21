// https://leetcode.com/problems/lexicographically-smallest-negated-permutation-that-sums-to-target/
// 3752. Lexicographically Smallest Negated Permutation that Sums to Target
pub struct Solution;
impl Solution {
    pub fn lex_smallest_negated_perm(n: i32, target: i64) -> Vec<i32> {
        let n = n as i64;
        let mut s = n * (n + 1) / 2;
        if target > s || target < -s || s % 2 != target.abs() % 2 {
            return vec![];
        }
        let mut ans = vec![0; n as usize];
        let mut i = 0;
        let mut j = n as usize - 1;
        for k in (1..=n).rev() {
            if s - k * 2 >= target {
                ans[i] = -k as i32;
                s -= k * 2;
                i += 1;
            } else {
                ans[j] = k as i32;
                j -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_smallest_negated_perm() {
        assert_eq!(Solution::lex_smallest_negated_perm(1, -1), vec![-1]);
        assert_eq!(Solution::lex_smallest_negated_perm(3, 0), vec![-3, 1, 2]);
        assert_eq!(Solution::lex_smallest_negated_perm(1, 10000000000), vec![]);
    }
}
