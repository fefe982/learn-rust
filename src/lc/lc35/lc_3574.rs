// https://leetcode.com/problems/maximize-subarray-gcd-score/
// 3574. Maximize Subarray GCD Score
pub struct Solution;
impl Solution {
    pub fn max_gcd_score(nums: Vec<i32>, k: i32) -> i64 {
        let gcd = |mut a: i32, mut b: i32| loop {
            if b == 0 {
                return a;
            }
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
        };
        let n = nums.len();
        let mut max = 0;
        for i in 0..n {
            let mut g = 0;
            let mut m2 = 32;
            let mut c2 = 0;
            for j in i..n {
                g = gcd(nums[j], g);
                let z = nums[j].trailing_zeros();
                if z < m2 {
                    m2 = z;
                    c2 = 1;
                } else if z == m2 {
                    c2 += 1;
                }
                let mut mul = g as i64 * (j - i + 1) as i64;
                if c2 <= k {
                    mul *= 2
                }
                max = max.max(mul);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_gcd_score() {
        assert_eq!(Solution::max_gcd_score(vec![2, 4], 1), 8);
        assert_eq!(Solution::max_gcd_score(vec![3, 5, 7], 2), 14);
        assert_eq!(Solution::max_gcd_score(vec![5, 5, 5], 1), 15);
    }
}
