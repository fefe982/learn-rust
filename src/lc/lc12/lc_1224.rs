// https://leetcode.com/problems/maximum-equal-frequency/
// 1224. Maximum Equal Frequency
pub struct Solution;
impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut max_cnt = 0;
        let mut max_cnt_freq = 0;
        let mut max_l = 0;
        for (n, l) in nums.into_iter().zip(1..) {
            let c = cnt.entry(n).or_default();
            *c += 1;
            if *c > max_cnt {
                max_cnt = *c;
                max_cnt_freq = 1;
            } else if *c == max_cnt {
                max_cnt_freq += 1;
            }
            if (max_cnt_freq == 1 && (max_cnt - 1) * cnt.len() as i32 + 1 == l)
                || (max_cnt_freq + 1 == cnt.len() as i32 && (max_cnt * (cnt.len() as i32 - 1) + 1) == l)
                || max_cnt == 1
            {
                max_l = l;
            }
        }
        max_l
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_equal_freq() {
        assert_eq!(Solution::max_equal_freq(vec![1, 2]), 2);
        assert_eq!(Solution::max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
        assert_eq!(
            Solution::max_equal_freq(vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5]),
            13
        );
    }
}
