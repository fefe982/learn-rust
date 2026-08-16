// https://leetcode.com/problems/elevator-requests-iii/
// 4027. Elevator Requests III
pub struct Solution;
impl Solution {
    pub fn elevator_requests(_n: i32, start: i32, requests: Vec<Vec<i32>>) -> i64 {
        let nr = requests.len();
        let mut dp = vec![vec![i64::MAX; nr]; 1 << nr];
        for mask in 1usize..(1 << nr) {
            if mask.count_ones() == 1 {
                let i = mask.trailing_zeros() as usize;
                dp[mask][i] = requests[i][0].max((requests[i][1] - start).abs()) as i64;
                continue;
            }
            let mut m = mask;
            while m > 0 {
                let i = m.trailing_zeros() as usize;
                let lm = mask ^ (1 << i);
                let mut lml = lm;
                let mut res = i64::MAX;
                while lml > 0 {
                    let j = lml.trailing_zeros() as usize;
                    res = res.min(dp[lm][j] + (requests[i][1] - requests[j][1]).abs() as i64);
                    lml ^= 1 << j;
                }
                dp[mask][i] = res.max(requests[i][0] as i64);
                m ^= 1 << i;
            }
        }
        *dp[(1 << nr) - 1].iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn elevator_requests() {
        assert_eq!(Solution::elevator_requests(9, 0, vec_vec![[0, 8], [6, 5]]), 9);
        assert_eq!(Solution::elevator_requests(8, 5, vec_vec![[1, 7], [7, 3]]), 7);
        assert_eq!(Solution::elevator_requests(7, 3, vec_vec![[0, 5], [0, 1], [6, 3]]), 8);
    }
}
