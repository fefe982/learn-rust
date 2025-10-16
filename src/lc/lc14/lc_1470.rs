// https://leetcode.com/problems/shuffle-the-array/
// 1470. Shuffle the Array
pub struct Solution;
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = vec![0; 2 * n as usize];
        for i in 0..n as usize {
            ans[2 * i] = nums[i];
            ans[2 * i + 1] = nums[i + n as usize];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shuffle() {
        assert_eq!(Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3), vec![2, 3, 5, 4, 1, 7]);
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
        assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
    }
}
