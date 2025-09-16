// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
// 1283. Find the Smallest Divisor Given a Threshold
pub struct Solution;
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let sum = |divisor: i32| -> i32 {
            nums.iter()
                .fold(0, |acc, &x| acc.saturating_add((x + divisor - 1) / divisor))
        };
        let mut i = 1;
        let mut j = *nums.iter().max().unwrap();
        if threshold == nums.len() as i32 {
            return j;
        }
        if sum(1) <= threshold {
            return 1;
        }
        while i + 1 < j {
            let mid = i + (j - i) / 2;
            if sum(mid) <= threshold {
                j = mid;
            } else {
                i = mid;
            }
        }
        j
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_divisor() {
        assert_eq!(Solution::smallest_divisor(vec![1, 2, 5, 9], 6), 5);
        assert_eq!(Solution::smallest_divisor(vec![44, 22, 33, 11, 1], 5), 44);
    }
}
