// https://leetcode.com/problems/count-number-of-nice-subarrays/
// 1248. Count Number of Nice Subarrays
pub struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut v = vec![0];
        let n = nums.len();
        for (n, i) in nums.into_iter().zip(1..) {
            if n % 2 == 1 {
                v.push(i);
            }
        }
        v.push(n as i32 + 1);
        let mut ans = 0;
        let k = k as usize;
        for j in k + 1..v.len() {
            ans += (v[j - k] - v[j - k - 1]) * (v[j] - v[j - 1]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2), 16);
    }
}
