// https://leetcode.com/problems/count-subarrays-with-even-odd-ratio-ii/
// 4013. Count Subarrays With Even Odd Ratio II
pub struct Solution;
impl Solution {
    pub fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i64 {
        let mut s = vec![0; nums.len() + 1];
        let a = a as i64;
        let b = b as i64;
        for i in 0..nums.len() {
            s[i + 1] = s[i] + if nums[i] % 2 == 0 { -b } else { a };
        }
        fn merge(s: &mut Vec<i64>) -> i64 {
            if s.len() <= 1 {
                return 0;
            }
            let mid = s.len() / 2;
            let mut left = s[..mid].to_vec();
            let mut right = s[mid..].to_vec();
            let mut res = merge(&mut left) + merge(&mut right);
            let mut l = 0;
            let mut r = 0;
            for i in 0..s.len() {
                if l < left.len() && (r >= right.len() || left[l] <= right[r]) {
                    s[i] = left[l];
                    l += 1;
                } else {
                    res += l as i64;
                    s[i] = right[r];
                    r += 1;
                }
            }
            res
        }
        merge(&mut s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_ratio_subarrays() {
        assert_eq!(Solution::count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2), 7);
        assert_eq!(Solution::count_ratio_subarrays(vec![2, 2, 1], 2, 1), 3);
        assert_eq!(Solution::count_ratio_subarrays(vec![2, 2, 2], 1, 1), 0);
    }
}
