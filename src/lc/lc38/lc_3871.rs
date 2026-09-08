// https://leetcode.com/problems/count-commas-in-range-ii/
// 3871. Count Commas in a Range II
pub struct Solution;
impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut p10 = 1000;
        let mut cnt = 0;
        while p10 <= n {
            cnt += n - p10 + 1;
            p10 *= 1000;
        }
        cnt
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
