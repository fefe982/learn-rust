// https://leetcode.com/problems/binary-gap/
// 868. Binary Gap
pub struct Solution;
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut l = n.trailing_zeros();
        let mut n = n & (n - 1);
        let mut max = 0;
        while n != 0 {
            let c = n.trailing_zeros();
            max = max.max(c - l);
            l = c;
            n = n & (n - 1);
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_gap() {
        assert_eq!(Solution::binary_gap(22), 2);
        assert_eq!(Solution::binary_gap(8), 0);
        assert_eq!(Solution::binary_gap(5), 2);
    }
}
