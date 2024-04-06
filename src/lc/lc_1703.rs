// https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/
// 1703. Minimum Adjacent Swaps for K Consecutive Ones
pub struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let pos = nums
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| if v == 1 { Some(i) } else { None })
            .collect::<Vec<_>>();
        let mut left = 0;
        let mut right = 0;
        let k = k as usize;
        for i in 0..k / 2 {
            left += (i + 1) * (pos[i + 1] - pos[i] - 1);
        }
        for i in k / 2..k - 1 {
            right += (k - i - 1) * (pos[i + 1] - pos[i] - 1);
        }
        let mut min = left + right;
        for i in 1..=pos.len() - k {
            left =
                left + (k / 2) * (pos[i + k / 2] - pos[i + k / 2 - 1] - 1) - (pos[i + k / 2 - 1] - pos[i - 1] - k / 2);
            right = right + (pos[i + k - 1] - pos[i + k / 2] + k / 2 + 1 - k)
                - (k - 1 - k / 2) * (pos[i + k / 2] - pos[i + k / 2 - 1] - 1);
            min = min.min(left + right);
        }
        min as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_moves() {
        assert_eq!(Solution::min_moves(vec![0, 1, 1, 0, 0, 1, 0, 0, 0], 3), 2);
        assert_eq!(Solution::min_moves(vec![1, 0, 0, 1, 0, 1], 2), 1);
        assert_eq!(Solution::min_moves(vec![1, 0, 0, 0, 0, 0, 1, 1], 3), 5);
        assert_eq!(Solution::min_moves(vec![1, 1, 0, 1], 2), 0);
    }
}
