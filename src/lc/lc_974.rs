// https://leetcode.com/problems/subarray-sums-divisible-by-k/
// 974. Subarray Sums Divisible by K
pub struct Solution;
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize];
        cnt[0] = 1;
        let mut sum = 0;
        let mut res = 0;
        for n in nums {
            sum += n;
            let r = ((sum % k + k) % k) as usize;
            res += cnt[r];
            cnt[r] += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subarrays_div_by_k() {
        assert_eq!(Solution::subarrays_div_by_k(vec![-2], 6), 0);
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }
}
