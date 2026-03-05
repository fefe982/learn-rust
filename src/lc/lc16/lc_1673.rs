// https://leetcode.com/problems/find-the-most-competitive-subsequence/
// 1673. Find the Most Competitive Subsequence
pub struct Solution;
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut q = std::collections::VecDeque::new();
        let n = nums.len();
        let k = k as usize;
        let mut res = Vec::with_capacity(k);
        for (i, num) in nums.into_iter().enumerate() {
            while let Some(&top) = q.back() {
                if top > num {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(num);
            if i + k >= n {
                res.push(q.pop_front().unwrap());
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_most_competitive() {
        assert_eq!(Solution::most_competitive(vec![3, 5, 2, 6], 2), [2, 6]);
        assert_eq!(
            Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4),
            [2, 3, 3, 4]
        );
    }
}
