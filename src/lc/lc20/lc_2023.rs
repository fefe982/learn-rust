// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
// 2023. Number of Pairs of Strings With Concatenation Equal to Target
pub struct Solution;
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let ts = target.as_bytes();
        let mut pre = vec![0; ts.len()];
        let mut pos = vec![0; ts.len()];
        for s in nums {
            let bs = s.as_bytes();
            if bs.len() < ts.len() {
                if bs == &ts[0..bs.len()] {
                    pre[bs.len()] += 1;
                }
                if bs == &ts[ts.len() - bs.len()..] {
                    pos[bs.len()] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 1..ts.len() {
            if ts.len() % 2 == 0 && i == ts.len() / 2 && ts[0..i] == ts[i..] {
                ans += pre[i] * (pre[i] - 1);
            } else {
                ans += pre[i] * pos[ts.len() - i];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_of_pairs() {
        assert_eq!(
            Solution::num_of_pairs(vec_str!["777", "7", "77", "77"], "7777".to_string()),
            4
        );
        assert_eq!(
            Solution::num_of_pairs(vec_str!["123", "4", "12", "34"], "1234".to_string()),
            2
        );
        assert_eq!(Solution::num_of_pairs(vec_str!["1", "1", "1"], "11".to_string()), 6);
    }
}
