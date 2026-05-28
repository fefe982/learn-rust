// https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum/
// 3300. Minimum Element After Replacement With Digit Sum
pub struct Solution;
impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|num| {
                let mut sum = 0;
                let mut n = num;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                sum
            })
            .min()
            .unwrap_or(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_element() {
        assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
        assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
    }
}
