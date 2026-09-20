// https://leetcode.com/problems/find-x-value-of-array-i/
// 3524. Find X Value of Array I
pub struct Solution;
impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        if k == 1 {
            return vec![((nums.len() + 1) * nums.len() / 2) as i64];
        }
        let k = k as usize;
        let mut res = vec![0; k];
        let mut cnt = vec![0; k];
        cnt[1] = 0;
        for n in nums {
            let mut ncnt = vec![0; k];
            for i in 0..k {
                let nr = i * n as usize % k;
                ncnt[nr] += cnt[i];
            }
            ncnt[n as usize % k] += 1;
            cnt = ncnt;
            for i in 0..k {
                res[i] += cnt[i];
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn result_array() {
        assert_eq!(Solution::result_array(vec![1, 2, 3, 4, 5], 3), vec![9, 2, 4]);
        assert_eq!(Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4), vec![18, 1, 2, 0]);
        assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
    }
}
