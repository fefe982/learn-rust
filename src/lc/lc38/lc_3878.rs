// https://leetcode.com/problems/count-good-subarrays/
// 3878. Count Good Subarrays
pub struct Solution;
impl Solution {
    pub fn count_good_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut l = vec![0; n];
        let mut r = vec![n - 1; n];
        let mut prev_bit = vec![0; 31];
        let mut next_bit = vec![n; 31];
        for i in 0..n {
            for bit in 0..31 {
                if nums[i] & (1 << bit) == 0 {
                    l[i] = l[i].max(prev_bit[bit]);
                } else {
                    prev_bit[bit] = i + 1;
                }
            }
        }
        for i in (0..n).rev() {
            for bit in 0..31 {
                if nums[i] & (1 << bit) == 0 {
                    r[i] = r[i].min(next_bit[bit] - 1);
                } else {
                    next_bit[bit] = i;
                }
            }
        }
        let mut ans = 0;
        let mut map = std::collections::HashMap::new();
        for i in 0..n {
            let mut ll = l[i];
            let rr = r[i];
            if let Some(&prev) = map.get(&nums[i]) {
                ll = ll.max(prev + 1);
            }
            map.insert(nums[i], i);
            ans += (rr - i + 1) as i64 * (i - ll + 1) as i64;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good_subarrays() {
        assert_eq!(Solution::count_good_subarrays(vec![10, 8, 9]), 5);
        assert_eq!(Solution::count_good_subarrays(vec![4, 2, 3]), 4);
        assert_eq!(Solution::count_good_subarrays(vec![1, 3, 1]), 6);
    }
}
