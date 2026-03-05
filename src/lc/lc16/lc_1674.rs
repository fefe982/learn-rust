// https://leetcode.com/problems/minimum-moves-to-make-array-complementary/
// 1674. Minimum Moves to Make Array Complementary
pub struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let mut cnt = vec![0; 2 * limit + 2];
        for i in 0..nums.len() / 2 {
            let a = nums[i] as usize;
            let b = nums[nums.len() - i - 1] as usize;
            cnt[2] += 2;
            cnt[a.min(b) + 1] -= 1;
            cnt[a + b] -= 1;
            cnt[a + b + 1] += 1;
            cnt[a.max(b) + limit + 1] += 1;
        }
        let mut res = i32::MAX;
        let mut r = 0;
        for i in 2..=limit * 2 {
            r += cnt[i];
            res = res.min(r);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec![1, 2, 4, 3], 4), 1);
        assert_eq!(Solution::min_moves(vec![1, 2, 2, 1], 2), 2);
        assert_eq!(Solution::min_moves(vec![1, 2, 1, 2], 2), 0);
    }
}
