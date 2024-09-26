// https://leetcode.com/problems/difference-between-element-sum-and-digit-sum-of-an-array/
// 2535. Difference Between Element Sum and Digit Sum of an Array
pub struct Solution;
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in nums {
            ans += n;
            let mut n = n;
            while n > 0 {
                ans -= n % 10;
                n /= 10;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_difference_of_sum() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
