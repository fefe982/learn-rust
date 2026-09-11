// https://leetcode.com/problems/identify-the-largest-outlier-in-an-array/
// 3371. Identify the Largest Outlier in an Array
pub struct Solution;
impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut sum = 0;
        for &n in &nums {
            *cnt.entry(n).or_insert(0) += 1;
            sum += n;
        }
        let mut outlier = i32::MIN;
        for &n in &nums {
            if n <= outlier {
                continue;
            }
            let mut s = sum - n;
            if s % 2 != 0 {
                continue;
            }
            s /= 2;
            if s == n {
                if cnt[&n] > 1 {
                    outlier = n;
                }
            } else if cnt.contains_key(&s) {
                outlier = n;
            }
        }
        outlier
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_largest_outlier() {
        assert_eq!(Solution::get_largest_outlier(vec![2, 3, 5, 10]), 10);
        assert_eq!(Solution::get_largest_outlier(vec![-2, -1, -3, -6, 4]), 4);
        assert_eq!(Solution::get_largest_outlier(vec![1, 1, 1, 1, 1, 5, 5]), 5);
    }
}
