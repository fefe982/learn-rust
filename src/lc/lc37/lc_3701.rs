// https://leetcode.com/problems/compute-alternating-sum/
// 3701. Compute Alternating Sum
pub struct Solution;
impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut flag = true;
        for i in nums {
            if flag {
                sum += i;
            } else {
                sum -= i;
            }
            flag = !flag;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alternating_sum() {
        assert_eq!(Solution::alternating_sum(vec![1, 3, 5, 7]), -4);
        assert_eq!(Solution::alternating_sum(vec![100]), 100);
    }
}
