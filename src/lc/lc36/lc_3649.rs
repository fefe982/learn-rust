// https://leetcode.com/problems/number-of-perfect-pairs/
// 3649. Number of Perfect Pairs
pub struct Solution;
impl Solution {
    pub fn perfect_pairs(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.iter_mut().for_each(|x| *x = x.abs());
        nums.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        for j in 1..nums.len() {
            while nums[i] * 2 < nums[j] {
                i += 1;
            }
            ans += j as i64 - i as i64;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn perfect_pairs() {
        assert_eq!(Solution::perfect_pairs(vec![0, 1, 2, 3]), 2);
        assert_eq!(Solution::perfect_pairs(vec![-3, 2, -1, 4]), 4);
        assert_eq!(Solution::perfect_pairs(vec![1, 10, 100, 1000]), 0);
    }
}
