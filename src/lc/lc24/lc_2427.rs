// https://leetcode.com/problems/number-of-common-factors/
// 2427. Number of Common Factors
pub struct Solution;
impl Solution {
    pub fn common_factors(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        let mut facter_cnt = 1;
        for f in 2.. {
            if f * f > a {
                break;
            }
            let mut cnt = 1;
            while a % f == 0 {
                a = a / f;
                cnt += 1;
            }
            facter_cnt *= cnt;
        }
        if a > 1 {
            facter_cnt *= 2;
        }
        facter_cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn common_factors() {
        assert_eq!(Solution::common_factors(12, 6), 4);
        assert_eq!(Solution::common_factors(25, 30), 2);
    }
}
