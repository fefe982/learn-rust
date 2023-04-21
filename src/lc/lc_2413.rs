// https://leetcode.com/problems/smallest-even-multiple/
// 2413. Smallest Even Multiple
pub struct Solution;
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            n * 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_even_multiple() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}
