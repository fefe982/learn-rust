// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array
// 2176. Count Equal and Divisible Pairs in an Array
pub struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cnt = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] && i * j % k == 0 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
