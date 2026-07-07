// https://leetcode.com/problems/minimum-impossible-or/
// 2568. Minimum Impossible OR
pub struct Solution;
impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            set.insert(num);
        }
        let mut ans = 1;
        while set.contains(&ans) {
            ans <<= 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_impossible_or() {
        assert_eq!(Solution::min_impossible_or(vec![2, 1]), 4);
        assert_eq!(Solution::min_impossible_or(vec![5, 3, 2]), 1);
    }
}
