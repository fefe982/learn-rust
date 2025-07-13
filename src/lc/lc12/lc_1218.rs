// https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/
// 1218. Longest Arithmetic Subsequence of Given Difference
pub struct Solution;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut map = std::collections::HashMap::<i32, i32>::new();
        let mut max = 1;
        for a in arr {
            let i = *map.get(&(a - difference)).unwrap_or(&0) + 1;
            max = max.max(i);
            map.insert(a, i);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_subsequence() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
        assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
        assert_eq!(
            Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
            4
        );
    }
}
