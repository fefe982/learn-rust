// https://leetcode.com/problems/count-number-of-bad-pairs/
// 2364. Count Number of Bad Pairs
pub struct Solution;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        for i in 0..nums.len() {
            nums[i] -= i as i32;
        }
        nums.sort_unstable();
        let mut ans = nums.len() as i64 * (nums.len() as i64 - 1) / 2;
        let mut i = 0;
        let mut j = 1;
        while j < nums.len() {
            while j < nums.len() && nums[i] == nums[j] {
                j += 1;
            }
            ans -= (j - i) as i64 * ((j - i) as i64 - 1) / 2;
            i = j;
            j += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_bad_pairs() {
        assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
        assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
    }
}
