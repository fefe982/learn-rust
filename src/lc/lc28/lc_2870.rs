// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/
// 2870. Minimum Number of Operations to Make Array Empty
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        for n in nums {
            *cnt.entry(n).or_default() += 1;
        }
        let mut ans = 0;
        for (_, v) in cnt {
            if v == 1 {
                return -1;
            }
            if v % 3 == 0 {
                ans += v / 3;
            } else {
                ans += v / 3 + 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
        assert_eq!(Solution::min_operations(vec![2, 1, 2, 2, 3, 3]), -1);
    }
}
