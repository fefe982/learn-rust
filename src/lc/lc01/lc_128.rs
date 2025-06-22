// https://leetcode.com/problems/longest-consecutive-sequence/
// 128. Longest Consecutive Sequence
pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for n in nums {
            set.insert(n);
        }
        let mut ans = 0;
        for &n in set.iter() {
            if !set.contains(&(n - 1)) {
                let mut next = n + 1;
                while set.contains(&next) {
                    next += 1;
                }
                ans = ans.max(next - n);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
