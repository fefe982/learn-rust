// https://leetcode.com/problems/sum-of-good-subsequences/
// 3351. Sum of Good Subsequences
pub struct Solution;
impl Solution {
    pub fn sum_of_good_subsequences(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut sumi = [0; 100003];
        let mut cnt = [0; 100003];
        let m = 1_000_000_007;
        for i in nums {
            let ii = (i + 1) as usize;
            cnt[ii] = (cnt[ii] + cnt[ii - 1] + cnt[ii + 1] + 1) % m;
            let add = (sumi[ii - 1] + sumi[ii + 1] + i as i64 * (cnt[ii - 1] + cnt[ii + 1] + 1)) % m;
            sumi[ii] = (sumi[ii] + add) % m;
            sum = (sum + add) % m
        }
        sum as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_good_subsequences() {
        assert_eq!(Solution::sum_of_good_subsequences(vec![3, 6, 4]), 20);
        assert_eq!(Solution::sum_of_good_subsequences(vec![1, 2, 1]), 14);
        assert_eq!(Solution::sum_of_good_subsequences(vec![3, 4, 5]), 40);
    }
}
