// https://leetcode.com/problems/number-of-stable-subsequences/
// 3686. Number of Stable Subsequences
pub struct Solution;
impl Solution {
    pub fn count_stable_subsequences(nums: Vec<i32>) -> i32 {
        let m = 1_000_000_007i64;
        let mut n00 = 1;
        let mut n01 = 0;
        let mut n10 = 0;
        let mut n11 = 1;
        for n in nums {
            if n % 2 == 0 {
                n00 = (n00 + n10) % m;
                n10 = (n10 + n11 + n01) % m;
            } else {
                n11 = (n11 + n01) % m;
                n01 = (n01 + n00 + n10) % m;
            }
        }
        ((n00 + n01 + n10 + n11 - 2 + m) % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_stable_subsequences() {
        assert_eq!(Solution::count_stable_subsequences(vec![1, 3, 5]), 6);
        assert_eq!(Solution::count_stable_subsequences(vec![2, 3, 4, 2]), 14);
    }
}
