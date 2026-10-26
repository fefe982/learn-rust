// https://leetcode.com/problems/maximum-number-of-subsequences-after-one-inserting/
// 3628. Maximum Number of Subsequences After One Inserting
pub struct Solution;
impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let mut l0 = 0;
        let mut l1 = 1;
        let mut lc0 = 0;
        let mut lc1 = 0;
        let mut lct0 = 0;
        let mut lct1 = 0;
        let mut nt = 0;
        for c in s.chars() {
            match c {
                'L' => {
                    l0 += 1;
                    l1 += 1;
                }
                'C' => {
                    lc0 += l0;
                    lc1 += l1;
                }
                'T' => {
                    lct0 += lc0;
                    lct1 += lc1;
                    nt += 1;
                }
                _ => {}
            }
        }
        let mut nl = 0;
        for c in s.chars() {
            match c {
                'L' => {
                    nl += 1;
                    lc0 = lc0.max(nl * nt);
                }
                'T' => {
                    nt -= 1;
                }
                _ => {}
            }
        }
        lct1.max(lct0 + lc0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_subsequences() {
        assert_eq!(Solution::num_of_subsequences("LT".to_string()), 1);
        assert_eq!(Solution::num_of_subsequences("LMCT".to_string()), 2);
        assert_eq!(Solution::num_of_subsequences("LCCT".to_string()), 4);
        assert_eq!(Solution::num_of_subsequences("L".to_string()), 0);
    }
}
