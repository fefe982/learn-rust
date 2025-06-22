// https://leetcode.com/problems/single-number-iii/
// 260. Single Number III
pub struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |xor, &num| xor ^ num);
        let diff = xor & xor.wrapping_neg();
        let mut xor1 = 0;
        let mut xor2 = 0;
        for num in nums {
            if num & diff != 0 {
                xor1 ^= num;
            } else {
                xor2 ^= num;
            }
        }
        vec![xor1, xor2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
        assert_eq!(Solution::single_number(vec![-1, 0]), vec![-1, 0]);
        assert_eq!(Solution::single_number(vec![0, 1]), vec![1, 0]);
    }
}
