// https://leetcode.com/problems/number-of-beautiful-partitions/
// 2478. Number of Beautiful Partitions
pub struct Solution;
impl Solution {
    fn search(pos: &[usize], k: usize, min_length: usize, dp: &mut Vec<Vec<i64>>) -> i64 {
        let n = pos.len();
        if k == 0 {
            return 1;
        }
        if n < k {
            return 0;
        }
        if dp[n][k] != -1 {
            return dp[n][k];
        }
        let mut i = 1;
        while i < n && pos[i] - pos[0] < min_length {
            i += 1;
        }
        let ret = (Self::search(&pos[i..], k - 1, min_length, dp) + Self::search(&pos[1..], k, min_length, dp))
            % 1_0000_0000_7;
        dp[n][k] = ret;
        ret
    }
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let s = s
            .chars()
            .map(|c| c == '2' || c == '3' || c == '5' || c == '7')
            .collect::<Vec<_>>();
        if !s[0] || s[s.len() - 1] {
            return 0;
        }
        let mut pos = vec![];
        let min_length = min_length as usize;
        for i in min_length - 1..s.len() - min_length {
            if !s[i] && s[i + 1] {
                pos.push(i);
            }
        }
        let k = k as usize;
        Self::search(&pos[..], k - 1, min_length, &mut vec![vec![-1; k]; pos.len() + 1]) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_beautiful_partitions() {
        assert_eq!(Solution::beautiful_partitions("22".to_string(), 1, 1), 0);
        assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3, 2), 3);
        assert_eq!(Solution::beautiful_partitions("23542185131".to_string(), 3, 3), 1);
        assert_eq!(Solution::beautiful_partitions("3312958".to_string(), 3, 1), 1);
    }
}
