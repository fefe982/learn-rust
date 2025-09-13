// https://leetcode.com/problems/minimum-swaps-to-make-strings-equal/
// 1247. Minimum Swaps to Make Strings Equal
pub struct Solution;
impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut nd = 0;
        let mut nx = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                nd += 1;
                if c1 == 'x' {
                    nx += 1;
                }
            }
        }
        if nd % 2 == 1 {
            -1
        } else if nx % 2 == 0 {
            nd / 2
        } else {
            nd / 2 + 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_swap() {
        assert_eq!(Solution::minimum_swap("xx".to_string(), "yy".to_string()), 1);
        assert_eq!(Solution::minimum_swap("xy".to_string(), "yx".to_string()), 2);
        assert_eq!(Solution::minimum_swap("xx".to_string(), "xy".to_string()), -1);
    }
}
