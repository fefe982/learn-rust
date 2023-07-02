// https://leetcode.com/problems/maximum-number-of-achievable-transfer-requests/
// 1601. Maximum Number of Achievable Transfer Requests
pub struct Solution;
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut cnt = vec![0; n as usize];
        for i in 0i32..1 << requests.len() {
            let c = requests.len() as i32 - (i.count_ones() as i32);
            if c <= max {
                continue;
            }
            for (id, r) in requests.iter().enumerate() {
                if i & (1 << id) == 0 {
                    cnt[r[0] as usize] += 1;
                    cnt[r[1] as usize] -= 1;
                }
            }
            let mut ok = true;
            for id in 0..n as usize {
                if cnt[id] != 0 {
                    ok = false;
                    cnt[id] = 0;
                }
            }
            if ok {
                max = c;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_requests() {
        assert_eq!(
            Solution::maximum_requests(5, vec_vec![[0, 1], [1, 0], [0, 1], [1, 2], [2, 0], [3, 4]]),
            5
        );
        assert_eq!(
            Solution::maximum_requests(3, vec_vec![[0, 0], [1, 2], [2, 1]]),
            3
        );
        assert_eq!(
            Solution::maximum_requests(4, vec_vec![[0, 3], [3, 1], [1, 2], [2, 0]]),
            4
        );
    }
}
