// https://leetcode.com/problems/n-th-tribonacci-number/
// 1137. N-th Tribonacci Number
pub struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        let mut a = vec![0; n as usize + 1];
        a[1] = 1;
        a[2] = 1;
        for i in 3..=n as usize {
            a[i] = a[i - 1] + a[i - 2] + a[i - 3];
        }
        a[n as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tribonacci() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
