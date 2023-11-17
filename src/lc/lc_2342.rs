// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
// 2342. Max Sum of a Pair With Equal Sum of Digits
pub struct Solution;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = -1;
        let mut max_map = std::collections::HashMap::<i32, i32>::new();
        for n in nums {
            let mut n_sum = 0;
            let mut nn = n;
            while nn > 0 {
                n_sum += nn % 10;
                nn /= 10;
            }
            if let Some(m) = max_map.get_mut(&n_sum) {
                max_sum = max_sum.max(*m + n);
                *m = n.max(*m);
            } else {
                max_map.insert(n_sum, n);
            }
        }
        max_sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_sum() {
        assert_eq!(Solution::maximum_sum(vec![18, 43, 36, 13, 7]), 54);
        assert_eq!(Solution::maximum_sum(vec![10, 12, 19, 14]), -1);
    }
}
