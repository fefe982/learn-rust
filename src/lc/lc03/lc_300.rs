// https://leetcode.com/problems/longest-increasing-subsequence/
// 300. Longest Increasing Subsequence
pub struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut r = vec![];
        for n in nums {
            if let Err(idx) = r.binary_search(&n) {
                if idx >= r.len() {
                    r.push(n);
                } else {
                    r[idx] = n;
                }
            }
        }
        r.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
