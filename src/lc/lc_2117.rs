// https://leetcode.com/problems/abbreviating-the-product-of-a-range/
// 2117. Abbreviating the Product of a Range
pub struct Solution;
impl Solution {
    fn get5(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n / 5;
            n /= 5;
        }
        res
    }
    pub fn abbreviate_product(left: i32, right: i32) -> String {
        let mut pre = 1.0;
        let mut suf = 1i64;
        let mut n5 = Self::get5(right) - Self::get5(left - 1);
        let mut n2 = n5;
        let mut total = 1;
        let m = 1_00000_00000i64;
        for i in left..=right {
            pre *= i as f64;
            while pre >= 10.0 {
                pre /= 10.0;
                total += 1;
            }
            let mut ii = i as i64;
            while ii % 5 == 0 {
                ii /= 5;
            }
            while n2 > 0 && ii % 2 == 0 {
                ii /= 2;
                n2 -= 1;
            }
            suf = (suf * ii) % m;
        }
        for _ in 0..n2 {
            suf = (suf * 5) % m;
        }
        n5 -= n2;
        if total - n5 <= 10 {
            format!("{suf}e{n5}")
        } else {
            format!("{}...{:05}e{n5}", (pre * 10000.0) as i32, suf % 100000)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_abbreviate_product() {
        assert_eq!(Solution::abbreviate_product(44, 9556), "10205...06688e2378");
        assert_eq!(Solution::abbreviate_product(1, 4), "24e0");
        assert_eq!(Solution::abbreviate_product(2, 11), "399168e2");
        assert_eq!(Solution::abbreviate_product(371, 375), "7219856259e3");
    }
}
