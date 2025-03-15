// https://leetcode.com/problems/super-pow/
// 372. Super Pow
pub struct Solution;
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let m = 1337;
        let a = a % 1337;
        if a == 0 || a == 1 {
            return a;
        }
        let mut ans = 1;
        let pow = |mut a: i32, mut b: i32| -> i32 {
            if b == 0 {
                return 1;
            }
            let mut ans = 1;
            while b > 0 {
                if b % 2 == 1 {
                    ans = (ans * a) % m;
                }
                a = (a * a) % m;
                b >>= 1;
            }
            ans
        };
        for i in b {
            ans = pow(ans, 10) * pow(a, i) % m;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn super_pow() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
    }
}
