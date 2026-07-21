// https://leetcode.com/problems/sum-of-squares-of-special-elements/
// 2778. Sum of Squares of Special Elements
pub struct Solution;
impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        for i in 1..=n {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                sum += nums[i - 1] * nums[i - 1];
                if n / i == i {
                    break;
                }
                sum += nums[n / i - 1] * nums[n / i - 1];
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_squares() {
        assert_eq!(Solution::sum_of_squares(vec![1, 2, 3, 4]), 21);
        assert_eq!(Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
    }
}
