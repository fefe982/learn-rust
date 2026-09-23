// https://leetcode.com/problems/maximum-frequency-after-subarray-operation/
// 3434. Maximum Frequency After Subarray Operation
pub struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; 51];
        let mut cnt_k = 0;
        let mut res = 0;
        let k = k as usize;
        for n in nums {
            let n = n as usize;
            cnt[n] = cnt[n].max(cnt_k) + 1;
            if n == k {
                res += 1;
                cnt_k += 1;
            }
            res = res.max(cnt[n]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_frequency() {
        assert_eq!(Solution::max_frequency(vec![1, 2, 3, 4, 5, 6], 1), 2);
        assert_eq!(Solution::max_frequency(vec![10, 2, 3, 4, 5, 5, 4, 3, 2, 2], 10), 4);
    }
}
