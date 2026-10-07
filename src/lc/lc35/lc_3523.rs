// https://leetcode.com/problems/make-array-non-decreasing/
// 3523. Make Array Non-decreasing
pub struct Solution;
impl Solution {
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        let mut last = 0;
        let mut cnt = 0;
        for n in nums {
            if n >= last {
                cnt += 1;
                last = n;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_possible_size() {
        assert_eq!(Solution::maximum_possible_size(vec![4, 2, 5, 3, 5]), 3);
        assert_eq!(Solution::maximum_possible_size(vec![1, 2, 3]), 3);
    }
}
