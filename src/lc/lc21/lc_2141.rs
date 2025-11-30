// https://leetcode.com/problems/maximum-running-time-of-n-computers/
// 2141. Maximum Running Time of N Computers
pub struct Solution;
impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries;
        batteries.sort_unstable();
        let n = n as usize;
        let mut s = batteries
            .iter()
            .take(batteries.len() - n)
            .fold(0i64, |acc, &b| acc + b as i64);
        for i in 0..n - 1 {
            let idx = batteries.len() - n + i;
            s += batteries[idx] as i64;
            let c = s / (i + 1) as i64;
            if c < batteries[idx + 1] as i64 {
                return c;
            }
        }
        (s + batteries[batteries.len() - 1] as i64) / n as i64
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_run_time() {
        assert_eq!(Solution::max_run_time(2, vec![3, 3, 3]), 4);
        assert_eq!(Solution::max_run_time(2, vec![1, 1, 1, 1]), 2);
    }
}
