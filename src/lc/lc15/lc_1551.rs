// https://leetcode.com/problems/minimum-operations-to-make-array-equal/
// 1551. Minimum Operations to Make Array Equal
pub struct Solution;
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        if n % 2 == 0 {
            (n / 2) * (n / 2)
        } else {
            (n / 2) * (n / 2 + 1)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(3), 2);
        assert_eq!(Solution::min_operations(6), 9);
    }
}
