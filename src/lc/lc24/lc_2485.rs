// https://leetcode.com/problems/find-the-pivot-integer/
// 2485. Find the Pivot Integer
pub struct Solution;
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let i = n * (n + 1) / 2;
        let x = (i as f64).sqrt() as i32;
        if x * x == i {
            x
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pivot_integer() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}
