// https://leetcode.com/problems/minimum-cost-to-merge-stones/
// 1000. Minimum Cost to Merge Stones
pub struct Solution;
impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        if stones.len() == 1 {
            return 0;
        }
        let k = k as usize;
        if (stones.len() - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut stones_sum = Vec::with_capacity(stones.len() + 1);
        stones_sum.push(0);
        for i in 0..stones.len() {
            stones_sum.push(stones_sum[i] + stones[i]);
        }
        let mut dp = vec![vec![i32::MAX; stones.len()]; stones.len()];
        for i in 0..stones.len() {
            dp[i][i] = 0;
        }
        for len in 2..=stones.len() {
            for l in 0..stones.len() - len + 1 {
                let r = l + len - 1;
                let step = (r - l - 1) / (k - 1);
                for p in 0..=step {
                    let q = l + p * (k - 1);
                    dp[l][r] = std::cmp::min(dp[l][r], dp[l][q] + dp[q + 1][r]);
                }
                if (r - l) % (k - 1) == 0 {
                    dp[l][r] += stones_sum[r + 1] - stones_sum[l]
                }
            }
        }
        dp[0][stones.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_stones() {
        assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 2), 20);
        assert_eq!(Solution::merge_stones(vec![3, 2, 4, 1], 3), -1);
        assert_eq!(Solution::merge_stones(vec![3, 5, 1, 2, 6], 3), 25);
    }
}
