// https://leetcode.com/problems/find-triangular-sum-of-an-array/
// 2221. Find Triangular Sum of an Array
pub struct Solution;
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        fn div(mut a: i32, mut b: i32) -> i32 {
            while a % b > 0 {
                let t = 5 / b;
                b = (b * (t + 1)) % 5;
                a = (a * (t + 1)) % 5;
            }
            a / b
        }
        let l = nums.len();
        let n = l as i32 - 1;
        let mut sum = nums[0] % 10;
        let mut m5 = 1;
        let mut c2 = 0;
        let mut c5 = 0;
        for k in 1..=n {
            let mul = n - k + 1;
            c2 += mul.trailing_zeros();
            c2 -= k.trailing_zeros();
            let mut mul5 = mul;
            while mul5 % 5 == 0 {
                mul5 /= 5;
                c5 += 1;
            }
            let mut k5 = k;
            while k5 % 5 == 0 {
                k5 /= 5;
                c5 -= 1;
            }
            m5 = div((m5 * mul5) % 5, k5);
            let mm5 = if c5 > 0 { 0 } else { m5 };
            let mm2 = if c2 > 0 { 0 } else { 1 };
            sum = (sum + nums[k as usize] * (mm5 * 6 + mm2 * 5)) % 10;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triangular_sum() {
        assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
        assert_eq!(Solution::triangular_sum(vec![5]), 5);
    }
}
