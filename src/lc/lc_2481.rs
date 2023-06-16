// https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/
// 2481. Minimum Cuts to Divide a Circle
pub struct Solution;
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n % 2 == 0 {
            n / 2
        } else {
            n
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_cuts() {
        assert_eq!(Solution::number_of_cuts(4), 2);
        assert_eq!(Solution::number_of_cuts(3), 3);
    }
}
