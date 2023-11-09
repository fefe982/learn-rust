// https://leetcode.com/problems/count-number-of-homogenous-substrings/
// 1759. Count Number of Homogenous Substrings
pub struct Solution;
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut sum = 0;
        let mut cnt = 0;
        let mut last = '0';
        for c in s.chars().chain(vec!['0']) {
            if c == last {
                cnt += 1;
            } else {
                sum += cnt as i64 * (cnt as i64 + 1) / 2;
                cnt = 1;
            }
            last = c;
        }
        (sum % 1000000007i64) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_homogenous() {
        assert_eq!(Solution::count_homogenous("abbcccaa".to_string()), 13);
        assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
        assert_eq!(Solution::count_homogenous("zzzzz".to_string()), 15);
    }
}
