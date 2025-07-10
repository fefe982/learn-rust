// https://leetcode.cn/problems/maximum-total-reward-using-operations-i/
// 3180. Maximum Total Reward Using Operations I
pub struct Solution;
impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut v = reward_values;
        v.sort_unstable();
        v.dedup();
        let max = v[v.len() - 1];
        if v.len() > 1 && v[v.len() - 2] == max - 1 {
            return max * 2 - 1;
        }
        if v.len() > 2 {
            let mut i = 0;
            let mut j = v.len() - 2;
            while i < j {
                if v[i] + v[j] == max - 1 {
                    return max * 2 - 1;
                }
                while i < j && v[i] + v[j] < max - 1 {
                    i += 1;
                }
                while i < j && v[i] + v[j] > max - 1 {
                    j -= 1;
                }
            }
        }
        let mut good = vec![false; max as usize * 2];
        good[0] = true;
        for i in 0..v.len() {
            let vi = v[i] as usize;
            for j in (vi..vi * 2).rev() {
                good[j] = good[j] || good[j - vi];
            }
        }
        for i in (0..good.len()).rev() {
            if good[i] {
                return i as i32;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_total_reward() {
        assert_eq!(Solution::max_total_reward(vec![1, 1, 3, 3]), 4);
        assert_eq!(Solution::max_total_reward(vec![1, 6, 4, 3, 2]), 11);
    }
}
