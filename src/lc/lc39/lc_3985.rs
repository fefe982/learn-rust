// https://leetcode.com/problems/palindromic-subarray-sum/
// 3985. Find the Longest Balanced Substring of a Binary String
pub struct Solution;
impl Solution {
    pub fn get_sum(nums: Vec<i32>) -> i64 {
        let mut cum_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            cum_sum[i + 1] = cum_sum[i] + nums[i] as i64;
        }
        let mut t = Vec::with_capacity(nums.len() * 2 + 3);
        t.push(-2);
        t.push(-1);
        for i in 0..nums.len() {
            t.push(nums[i] as i32);
            t.push(-1);
        }
        t.push(-3);
        let mut l = vec![0; t.len()];
        let mut mid = 0;
        let mut mid_r = 0;
        let mut ans = 0;
        for i in 2..t.len() - 2 {
            let mut cl = 0;
            if mid_r > i {
                cl = (mid_r - i).min(l[mid * 2 - i]);
            }
            while t[i - cl - 1] == t[i + cl + 1] {
                cl += 1;
                mid = i;
                mid_r = i + cl;
            }
            let s = (i - cl - 1) / 2;
            let t = (i + cl) / 2;
            l[i] = cl;
            ans = ans.max(cum_sum[t] - cum_sum[s]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_sum() {
        assert_eq!(Solution::get_sum(vec![10, 10]), 20);
        assert_eq!(Solution::get_sum(vec![1, 2, 3, 2, 1, 5, 6]), 9);
        assert_eq!(Solution::get_sum(vec![7, 1, 2, 1, 7, 3, 4, 3, 4]), 18);
        assert_eq!(Solution::get_sum(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(Solution::get_sum(vec![1000]), 1000);
    }
}
