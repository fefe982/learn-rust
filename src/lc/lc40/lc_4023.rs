// https://leetcode.com/problems/elevator-requests-ii/
// 4023. Elevator Requests II
pub struct Solution;
impl Solution {
    pub fn elevator_requests(_n: i32, start: i32, requests: Vec<i32>) -> i64 {
        let mut requests = requests;
        if requests.iter().find(|&&x| x == start).is_none() {
            requests.push(start);
        }
        requests.sort();
        let p = requests.binary_search(&start).unwrap();
        let len = requests.len();
        let mut dp = vec![[i64::MAX; 2]; len];
        dp[p] = [0, 0];
        for l in 2..=len {
            let mut ndp = vec![[i64::MAX; 2]; len - l + 1];
            for i in 0..dp.len() - 1 {
                let ll = dp[i + 1][0];
                let lr = dp[i + 1][1];
                if ll != i64::MAX {
                    ndp[i][0] = ll + (requests[i + 1] - requests[i]) as i64 * (requests.len() - l + 1) as i64;
                }
                if lr != i64::MAX {
                    ndp[i][0] = ndp[i][0]
                        .min(lr + (requests[i + l - 1] - requests[i]) as i64 * (requests.len() - l + 1) as i64);
                }
                let rl = dp[i][0];
                let rr = dp[i][1];
                if rl != i64::MAX {
                    ndp[i][1] = rl + (requests[i + l - 1] - requests[i]) as i64 * (requests.len() - l + 1) as i64;
                }
                if rr != i64::MAX {
                    ndp[i][1] = ndp[i][1]
                        .min(rr + (requests[i + l - 1] - requests[i + l - 2]) as i64 * (requests.len() - l + 1) as i64);
                }
            }
            dp = ndp;
        }
        dp[0][0].min(dp[0][1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elevator_requests() {
        assert_eq!(Solution::elevator_requests(6, 4, vec![1, 5]), 6);
        assert_eq!(Solution::elevator_requests(8, 3, vec![3, 7, 1]), 10);
        assert_eq!(Solution::elevator_requests(10, 5, vec![0, 2, 9]), 22);
    }
}
