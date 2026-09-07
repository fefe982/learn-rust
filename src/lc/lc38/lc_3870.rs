// https://leetcode.com/problems/count-commas-in-range/
// 3870. Count Commas in Range
pub struct Solution;
impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        if n < 1000 {
            0
        } else {
            n - 999
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_commas() {
        assert_eq!(Solution::count_commas(1002), 3);
        assert_eq!(Solution::count_commas(998), 0);
    }
}
