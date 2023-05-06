// https://leetcode.cn/problems/pairs-of-songs-with-total-durations-divisible-by-60/
// 1010. Pairs of Songs With Total Durations Divisible by 60
pub struct Solution;
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = vec![0u32; 60];
        for t in time {
            cnt[(t % 60) as usize] += 1;
        }
        let mut tot = cnt[0] * (cnt[0] - 1) / 2 + cnt[30] * (cnt[30] - 1) / 2;
        for idx in 1..=29 {
            tot += cnt[idx] * cnt[60 - idx];
        }
        tot as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_pairs_divisible_by60() {
        assert_eq!(
            Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]),
            3
        );
        assert_eq!(Solution::num_pairs_divisible_by60(vec![60, 60, 60]), 3);
    }
}
