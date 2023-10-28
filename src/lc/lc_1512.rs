// https://leetcode.com/problems/number-of-good-pairs/
// 1512. Number of Good Pairs
pub struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut count = 0;
        for (_, v) in m {
            count += v * (v - 1) / 2;
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
