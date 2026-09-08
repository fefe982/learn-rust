// https://leetcode.com/problems/find-the-maximum-factor-score-of-array/
// 3334. Find the Maximum Factor Score of an Array
pub struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        if n == 1 {
            return nums[0] as i64 * nums[0] as i64;
        }
        let gcd = |mut x: i64, mut y: i64| -> i64 {
            loop {
                if x == 0 {
                    return y;
                }
                y = y % x;
                if y == 0 {
                    return x;
                }
                x = x % y;
            }
        };
        let lcm = |x: i64, y: i64| -> i64 { x / gcd(x, y) * y };
        let mut pre_gcd = Vec::with_capacity(nums.len());
        let mut pre_lcm = Vec::with_capacity(nums.len());
        pre_gcd.push(nums[0] as i64);
        pre_lcm.push(nums[0] as i64);
        for i in 1..nums.len() {
            pre_gcd.push(gcd(pre_gcd[i - 1], nums[i] as i64));
            pre_lcm.push(lcm(pre_lcm[i - 1], nums[i] as i64));
        }
        let mut ans = (pre_gcd[n - 1] * pre_lcm[n - 1]).max(pre_gcd[n - 2] * pre_lcm[n - 2]);
        let mut post_gcd = nums[n - 1] as i64;
        let mut post_lcm = nums[n - 1] as i64;
        for i in (1..n - 1).rev() {
            let gg = gcd(post_gcd, pre_gcd[i - 1]);
            let ll = lcm(post_lcm, pre_lcm[i - 1]);
            ans = ans.max(gg * ll);
            post_gcd = gcd(post_gcd, nums[i] as i64);
            post_lcm = lcm(post_lcm, nums[i] as i64);
        }
        ans = ans.max(post_gcd * post_lcm);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![2, 4, 8, 16]), 64);
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5]), 60);
        assert_eq!(Solution::max_score(vec![3]), 9);
    }
}
