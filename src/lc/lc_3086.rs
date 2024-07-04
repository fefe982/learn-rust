// https://leetcode.com/problems/minimum-moves-to-pick-k-ones/
// 3086. Minimum Moves to Pick K Ones
pub struct Solution;
impl Solution {
    pub fn minimum_moves(nums: Vec<i32>, k: i32, max_changes: i32) -> i64 {
        let mut c = 0;
        let mut pos = vec![];
        let mut cc = 0;
        for (p, i) in nums.into_iter().enumerate() {
            if i == 1 {
                cc += 1;
                pos.push(p as i64);
            } else {
                c = c.max(cc);
                cc = 0;
            }
        }
        let k = k as i64;
        let max_changes = max_changes as i64;
        c = c.max(cc).min(k).min(3);
        if max_changes >= k - c {
            return (c - 1).max(0) + (k - c) * 2;
        }
        println!("{:?}", pos);
        let len = (k - max_changes) as usize;
        let lm = len / 2;
        let rm = lm + if len % 2 == 1 { 1 } else { 0 };
        let mut left_sum = pos[0..lm].iter().sum::<i64>();
        let mut right_sum = pos[rm..len].iter().sum::<i64>();
        let mut res = right_sum - left_sum;
        for i in 0..pos.len() - len {
            left_sum = left_sum - pos[i] + pos[i + lm];
            right_sum = right_sum - pos[i + rm] + pos[i + len];
            res = res.min(right_sum - left_sum);
        }
        res + max_changes * 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_moves() {
        assert_eq!(Solution::minimum_moves(vec![1, 0, 1, 0, 1, 1], 5, 2), 7);
        assert_eq!(Solution::minimum_moves(vec![1, 1, 0, 0, 0, 1, 1, 0, 0, 1], 3, 1), 3);
        assert_eq!(Solution::minimum_moves(vec![0, 0, 0, 0], 2, 2), 4);
    }
}
